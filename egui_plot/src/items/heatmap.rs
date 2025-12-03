use std::ops::RangeInclusive;

use egui::Color32;
use egui::Mesh;
use egui::NumExt as _;
use egui::Pos2;
use egui::Rect;
use egui::Rgba;
use egui::Shape;
use egui::TextStyle;
use egui::Ui;
use egui::Vec2;
use egui::WidgetText;
use emath::Float as _;

use crate::axis::PlotTransform;
use crate::bounds::PlotBounds;
use crate::bounds::PlotPoint;
use crate::colors::BASE_COLORS;
use crate::cursor::Cursor;
use crate::items::ClosestElem;
use crate::items::PlotConfig;
use crate::items::PlotGeometry;
use crate::items::PlotItem;
use crate::items::PlotItemBase;
use crate::label::LabelFormatter;

/// Default resolution for heatmap color palette
pub const DEFAULT_RESOLUTION: usize = 128;

/// A heatmap.
pub struct Heatmap {
    base: PlotItemBase,

    /// Occupied space in absolute plot coordinates.
    pos: PlotPoint,

    /// values to plot
    pub(crate) values: Vec<f64>,

    /// number of columns in heatmap
    cols: usize,

    /// number of rows in heatmap
    rows: usize,

    /// minimum value in colormap.
    /// Everything smaller will be mapped to the first color in `palette`
    min: f64,

    /// maximum value in heatmap
    /// Everything greater will be mapped to `palette.last()`
    max: f64,

    /// formatter for labels on tiles
    formatter: Box<dyn Fn(f64) -> String>,

    /// custom mapping of values to color
    custom_mapping: Option<Box<dyn Fn(f64) -> Color32>>,

    /// show labels on tiles
    show_labels: bool,

    /// resolution of the color palette
    resolution: usize,

    /// possible colors, sorted by index
    palette: Vec<Color32>,

    /// is widget is highlighted
    highlight: bool,

    /// plot name
    name: String,

    /// Size of one tile in plot coordinates
    tile_size: Vec2,
}

impl PartialEq for Heatmap {
    /// manual implementation of `PartialEq` because formatter and color mapping
    /// do not impl `PartialEq`.
    ///
    /// > NOTE: custom_mapping and formatter are ignored
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
            && self.values == other.values
            && self.cols == other.cols
            && self.rows == other.rows
            && self.min == other.min
            && self.max == other.max
            && self.show_labels == other.show_labels
            && self.resolution == other.resolution
            && self.palette == other.palette
            && self.highlight == other.highlight
            && self.name == other.name
            && self.tile_size == other.tile_size
    }
}

impl Heatmap {
    /// Create a 2D heatmap. Will automatically infer number of rows.
    ///
    /// - `values` contains magnitude of each tile. The alignment is row by row.
    /// - `cols` is the number of columns (i.e. the length of each row).
    /// - `values.len()` should be a multiple of `cols`.
    ///
    /// Example: To display this
    ///
    /// | -- | -- |
    /// | 0.0 | 0.1 |
    /// | 0.3 | 0.4 |
    ///
    /// pass `values = vec![0.0, 0.1, 0.3, 0.4]` and `cols = 2`.
    ///
    /// If parameters are invalid (e.g., `cols` is zero, `values` is empty, or
    /// `values.len()` is not divisible by `cols`), an empty heatmap is created.
    pub fn new(values: Vec<f64>, cols: usize) -> Self {
        // Handle invalid parameters by creating an empty heatmap
        if cols == 0 || values.is_empty() || (values.len() % cols) != 0 {
            return Self::empty();
        }

        let rows = values.len() / cols;

        // determine range
        let mut min = f64::MAX;
        let mut max = f64::MIN;
        for v in &values {
            min = min.min(*v);
            max = max.max(*v);
        }

        let resolution = DEFAULT_RESOLUTION;

        Self {
            base: PlotItemBase::new(String::new()),
            pos: PlotPoint { x: 0.0, y: 0.0 },
            values,
            cols,
            rows,
            min,
            max,
            formatter: Box::new(|v| format!("{v:.1}")),
            custom_mapping: None,
            show_labels: true,
            resolution,
            palette: Self::linear_gradient_from_base_colors(&BASE_COLORS, resolution),
            highlight: false,
            name: String::new(),
            tile_size: Vec2 { x: 1.0, y: 1.0 },
        }
    }

    /// Create an empty heatmap (no tiles).
    fn empty() -> Self {
        let resolution = DEFAULT_RESOLUTION;
        Self {
            base: PlotItemBase::new(String::new()),
            pos: PlotPoint { x: 0.0, y: 0.0 },
            values: Vec::new(),
            cols: 0,
            rows: 0,
            min: 0.0,
            max: 0.0,
            formatter: Box::new(|v| format!("{v:.1}")),
            custom_mapping: None,
            show_labels: true,
            resolution,
            palette: Self::linear_gradient_from_base_colors(&BASE_COLORS, resolution),
            highlight: false,
            name: String::new(),
            tile_size: Vec2 { x: 1.0, y: 1.0 },
        }
    }

    /// Set the resolution of the color palette.
    ///
    /// Higher resolution means smoother color transitions.
    /// Default is 128.
    #[inline]
    pub fn resolution(mut self, resolution: usize) -> Self {
        self.resolution = resolution;
        self.palette = Self::linear_gradient_from_base_colors(&BASE_COLORS, resolution);
        self
    }

    /// Set color palette by specifying base colors from low to high
    #[inline]
    pub fn palette(mut self, base_colors: &[Color32]) -> Self {
        self.palette = Self::linear_gradient_from_base_colors(base_colors, self.resolution);
        self
    }

    /// Interpolate linear gradient with given resolution from an arbitrary
    /// number of base colors.
    fn linear_gradient_from_base_colors(base_colors: &[Color32], resolution: usize) -> Vec<Color32> {
        let mut interpolated = vec![Color32::TRANSPARENT; resolution];
        if base_colors.is_empty() || resolution == 0 {
            return interpolated;
        }
        if base_colors.len() == 1 || resolution == 1 {
            // single color, no gradient
            return vec![base_colors[0]; resolution];
        }
        for (i, color) in interpolated.iter_mut().enumerate() {
            let i_rel: f64 = i as f64 / (resolution - 1) as f64;
            if i_rel == 1.0 {
                // last element
                *color = *base_colors.last().expect("Base colors should not be empty");
            } else {
                let base_index_float: f64 = i_rel * (base_colors.len() - 1) as f64;
                let base_index: usize = base_index_float as usize;
                let start_color = base_colors[base_index];
                let end_color = base_colors[base_index + 1];
                let gradient_level = base_index_float - base_index as f64;

                let delta_r = (end_color.r() as f64 - start_color.r() as f64) * gradient_level;
                let delta_g = (end_color.g() as f64 - start_color.g() as f64) * gradient_level;
                let delta_b = (end_color.b() as f64 - start_color.b() as f64) * gradient_level;

                // interpolate
                let r = (start_color.r() as f64 + delta_r).round() as u8;
                let g = (start_color.g() as f64 + delta_g).round() as u8;
                let b = (start_color.b() as f64 + delta_b).round() as u8;
                *color = Color32::from_rgb(r, g, b);
            }
        }
        interpolated
    }

    /// Specify custom range of values to map onto color palette.
    ///
    /// - `min` and everything smaller will be the first color on the color
    ///   palette.
    /// - `max` and everything greater will be the last color on the color
    ///   palette.
    #[inline]
    pub fn range(mut self, min: f64, max: f64) -> Self {
        assert!(min < max, "min must be smaller than max");
        self.min = min;
        self.max = max;
        self
    }

    /// Add a custom way to format an element.
    /// Can be used to display a set number of decimals or custom labels.
    #[inline]
    pub fn formatter(mut self, formatter: Box<dyn Fn(f64) -> String>) -> Self {
        self.formatter = formatter;
        self
    }

    /// Add a custom way to map values to a color.
    #[inline]
    pub fn custom_mapping(mut self, custom_mapping: Box<dyn Fn(f64) -> Color32>) -> Self {
        self.custom_mapping = Some(custom_mapping);
        self
    }

    /// Show labels for each tile in heatmap. Defaults to 'true'
    #[inline]
    pub fn show_labels(mut self, en: bool) -> Self {
        self.show_labels = en;
        self
    }

    /// Place lower left corner of heatmap at `pos`. Default is (0.0, 0.0)
    #[inline]
    pub fn at(mut self, pos: PlotPoint) -> Self {
        self.pos = pos;
        self
    }

    /// Name of this heatmap.
    ///
    /// This name will show up in the plot legend, if legends are turned on.
    /// Multiple heatmaps may share the same name, in which case they will
    /// also share an entry in the legend.
    #[expect(clippy::needless_pass_by_value)]
    #[inline]
    pub fn name(mut self, name: impl ToString) -> Self {
        self.name = name.to_string();
        self
    }

    /// Manually set width and height of tiles in plot coordinate space.
    #[inline]
    pub fn tile_size(mut self, x: f32, y: f32) -> Self {
        self.tile_size = Vec2 { x, y };
        self
    }

    /// Set size of heatmap in plot coordinate space.
    /// Will adjust the heatmap tile size in plot coordinate space.
    #[inline]
    pub fn size(mut self, x: f32, y: f32) -> Self {
        self.tile_size = Vec2 {
            x: x / self.cols as f32,
            y: y / self.rows as f32,
        };
        self
    }

    /// Highlight all plot elements.
    #[inline]
    pub fn highlight(mut self, highlight: bool) -> Self {
        // TODO(#194): for some reason highlighting is not detected
        self.highlight = highlight;
        self
    }

    fn push_shapes(&self, ui: &Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        let mut mesh = Mesh::default();
        let mut labels: Vec<Shape> = Vec::new();
        for i in 0..self.values.len() {
            let (rect, color, text) = self.tile_view_info(ui, transform, i);
            mesh.add_colored_rect(rect, color);
            if self.show_labels {
                labels.push(text);
            }
        }
        shapes.push(Shape::mesh(mesh));
        if self.show_labels {
            shapes.extend(labels);
        }
    }

    fn tile_view_info(&self, ui: &Ui, transform: &PlotTransform, index: usize) -> (Rect, Color32, Shape) {
        let v = self.values[index];

        // calculate color value
        let mut fill_color: Color32;
        if let Some(mapping) = &self.custom_mapping {
            fill_color = mapping(v);
        } else {
            // convert to value in [0.0, 1.0]
            let v_rel = (v - self.min) / (self.max - self.min);

            // convert to color palette index
            let palette_index = (v_rel * (self.palette.len() - 1) as f64).round() as usize;

            fill_color = self.palette[palette_index];
        }

        if self.highlight {
            let fill = Rgba::from(fill_color);
            let fill_alpha = (2.0 * fill.a()).at_most(1.0);
            let fill = fill.to_opaque().multiply(fill_alpha);
            fill_color = fill.into();
        }

        let x = index % self.cols;
        let y = index / self.cols;
        let tile_rect = transform.rect_from_values(
            &PlotPoint {
                x: self.pos.x + self.tile_size.x as f64 * x as f64,
                y: self.pos.y + self.tile_size.y as f64 * y as f64,
            },
            &PlotPoint {
                x: self.pos.x + self.tile_size.x as f64 * (x + 1) as f64,
                y: self.pos.y + self.tile_size.y as f64 * (y + 1) as f64,
            },
        );
        // Text

        let text: WidgetText = (self.formatter)(v).into();

        // calculate color that is readable on coloured tiles
        let luminance =
            0.2126 * fill_color.r() as f32 + 0.7151 * fill_color.g() as f32 + 0.0721 * fill_color.b() as f32;

        let inverted_color = if luminance < 140.0 {
            Color32::WHITE
        } else {
            Color32::BLACK
        };

        let text = text.color(inverted_color);
        let galley = text.into_galley(
            ui,
            Some(egui::TextWrapMode::Truncate),
            f32::INFINITY,
            TextStyle::Monospace,
        );
        let text_pos = tile_rect.center() - galley.size() / 2.0;

        let text = Shape::galley(text_pos, galley.clone(), Color32::WHITE);
        (tile_rect, fill_color, text)
    }
}

impl PlotItem for Heatmap {
    fn shapes(&self, ui: &Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        self.push_shapes(ui, transform, shapes);
    }

    fn initialize(&mut self, _x_range: RangeInclusive<f64>) {
        // nothing to do
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn color(&self) -> Color32 {
        Color32::TRANSPARENT
    }

    fn highlight(&mut self) {
        self.highlight = true;
    }

    fn highlighted(&self) -> bool {
        self.highlight
    }

    fn geometry(&self) -> PlotGeometry<'_> {
        PlotGeometry::Rects
    }

    fn bounds(&self) -> PlotBounds {
        PlotBounds {
            min: [self.pos.x, self.pos.y],
            max: [
                self.pos.x + self.tile_size.x as f64 * self.cols as f64,
                self.pos.y + self.tile_size.y as f64 * self.rows as f64,
            ],
        }
    }

    fn find_closest(&self, point: Pos2, transform: &PlotTransform) -> Option<ClosestElem> {
        (0..self.values.len())
            .map(|index| {
                let x = index % self.cols;
                let y = index / self.cols;

                let tile_rect = transform.rect_from_values(
                    &PlotPoint {
                        x: self.pos.x + self.tile_size.x as f64 * x as f64,
                        y: self.pos.y + self.tile_size.y as f64 * y as f64,
                    },
                    &PlotPoint {
                        x: self.pos.x + self.tile_size.x as f64 * (x + 1) as f64,
                        y: self.pos.y + self.tile_size.y as f64 * (y + 1) as f64,
                    },
                );

                let dist_sq = tile_rect.distance_sq_to_pos(point);

                ClosestElem { index, dist_sq }
            })
            .min_by_key(|e| e.dist_sq.ord())
    }

    fn on_hover(
        &self,
        _plot_area_response: &egui::Response,
        elem: ClosestElem,
        shapes: &mut Vec<Shape>,
        _cursors: &mut Vec<Cursor>,
        plot: &PlotConfig<'_>,
        _: &LabelFormatter<'_>,
    ) {
        let (rect, color, text) = self.tile_view_info(plot.ui, plot.transform, elem.index);
        let mut mesh = Mesh::default();
        mesh.add_colored_rect(rect, color);
        shapes.push(Shape::mesh(mesh));
        if self.show_labels {
            shapes.push(text);
        }
    }

    fn base(&self) -> &super::PlotItemBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut super::PlotItemBase {
        &mut self.base
    }
}
