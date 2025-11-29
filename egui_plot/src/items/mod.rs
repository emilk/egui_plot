//! Contains items that can be added to a plot.
#![expect(clippy::type_complexity)] // TODO(#163): simplify some of the callback types with type aliases

use std::ops::RangeInclusive;

pub use arrows::Arrows;
pub use bar_chart::Bar;
pub use bar_chart::BarChart;
pub use box_plot::BoxElem;
pub use box_plot::BoxPlot;
pub use box_plot::BoxSpread;
use egui::Align2;
use egui::Color32;
use egui::Id;
use egui::NumExt as _;
use egui::PopupAnchor;
use egui::Pos2;
use egui::Shape;
use egui::TextStyle;
use egui::Ui;
use egui::pos2;
use egui::vec2;
use emath::Float as _;
pub use heatmap::Heatmap;
pub use heatmap::HeatmapErr;
pub use line::HLine;
pub use line::VLine;
pub use plot_image::PlotImage;
pub use points::Points;
pub use polygon::Polygon;
use rect_elem::RectElement;
pub use series::Line;
pub use text::Text;
pub use values::ClosestElem;
pub use values::LineStyle;
pub use values::MarkerShape;
pub use values::Orientation;
pub use values::PlotGeometry;
pub use values::PlotPoint;
pub use values::PlotPoints;

use super::Cursor;
use super::LabelFormatter;
use super::PlotBounds;
use super::PlotTransform;

mod arrows;
mod bar_chart;
mod box_plot;
mod heatmap;
mod line;
mod plot_image;
mod points;
mod polygon;
mod rect_elem;
mod series;
mod text;
mod values;

const DEFAULT_FILL_ALPHA: f32 = 0.05;

/// Base data shared by all plot items.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlotItemBase {
    name: String,
    id: Id,
    highlight: bool,
    allow_hover: bool,
}

impl PlotItemBase {
    /// Create a new plot item base with the given name.
    pub fn new(name: String) -> Self {
        let id = Id::new(&name);
        Self {
            name,
            id,
            highlight: false,
            allow_hover: true,
        }
    }
}

/// Container to pass-through several parameters related to plot visualization
pub struct PlotConfig<'a> {
    /// Reference to the UI.
    pub ui: &'a Ui,

    /// Reference to the plot transform.
    pub transform: &'a PlotTransform,

    /// Whether to show the x-axis value.
    pub show_x: bool,

    /// Whether to show the y-axis value.
    pub show_y: bool,
}

/// Trait shared by things that can be drawn in the plot.
pub trait PlotItem {
    /// Generate shapes to be drawn in the plot.
    fn shapes(&self, ui: &Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>);

    /// For plot-items which are generated based on x values (plotting
    /// functions).
    fn initialize(&mut self, x_range: RangeInclusive<f64>);

    /// Returns the name of the plot item.
    fn name(&self) -> &str {
        &self.base().name
    }

    /// Returns the color of the plot item.
    fn color(&self) -> Color32;

    /// Highlight the plot item.
    fn highlight(&mut self) {
        self.base_mut().highlight = true;
    }

    /// Returns whether the plot item is highlighted.
    fn highlighted(&self) -> bool {
        self.base().highlight
    }

    /// Can the user hover this item?
    fn allow_hover(&self) -> bool {
        self.base().allow_hover
    }

    /// Returns the geometry of the plot item.
    fn geometry(&self) -> PlotGeometry<'_>;

    /// Returns the bounds of the plot item.
    fn bounds(&self) -> PlotBounds;

    /// Returns a reference to the base data of the plot item.
    fn base(&self) -> &PlotItemBase;

    /// Returns a mutable reference to the base data of the plot item.
    fn base_mut(&mut self) -> &mut PlotItemBase;

    /// Returns the ID of the plot item.
    fn id(&self) -> Id {
        self.base().id
    }

    /// Find the closest element in the plot item to the given point.
    fn find_closest(&self, point: Pos2, transform: &PlotTransform) -> Option<ClosestElem> {
        match self.geometry() {
            PlotGeometry::None => None,

            PlotGeometry::Points(points) => points
                .iter()
                .enumerate()
                .map(|(index, value)| {
                    let pos = transform.position_from_point(value);
                    let dist_sq = point.distance_sq(pos);
                    ClosestElem { index, dist_sq }
                })
                .min_by_key(|e| e.dist_sq.ord()),

            PlotGeometry::Rects => {
                panic!("If the PlotItem is made of rects, it should implement find_closest()")
            }
        }
    }

    /// Handle hover events for the plot item.
    fn on_hover(
        &self,
        plot_area_response: &egui::Response,
        elem: ClosestElem,
        shapes: &mut Vec<Shape>,
        cursors: &mut Vec<Cursor>,
        plot: &PlotConfig<'_>,
        label_formatter: &LabelFormatter<'_>,
    ) {
        let points = match self.geometry() {
            PlotGeometry::Points(points) => points,
            PlotGeometry::None => {
                panic!("If the PlotItem has no geometry, on_hover() must not be called")
            }
            PlotGeometry::Rects => {
                panic!("If the PlotItem is made of rects, it should implement on_hover()")
            }
        };

        let line_color = if plot.ui.visuals().dark_mode {
            Color32::from_gray(100).additive()
        } else {
            Color32::from_black_alpha(180)
        };

        // this method is only called, if the value is in the result set of
        // find_closest()
        let value = points[elem.index];
        let pointer = plot.transform.position_from_point(&value);
        shapes.push(Shape::circle_filled(pointer, 3.0, line_color));

        rulers_and_tooltip_at_value(plot_area_response, value, self.name(), plot, cursors, label_formatter);
    }
}

// ----------------------------------------------------------------------------

/// Returns the x-coordinate of a possible intersection between a line segment
/// from `p1` to `p2` and a horizontal line at the given y-coordinate.
fn y_intersection(p1: &Pos2, p2: &Pos2, y: f32) -> Option<f32> {
    ((p1.y > y && p2.y < y) || (p1.y < y && p2.y > y))
        .then_some(((y * (p1.x - p2.x)) - (p1.x * p2.y - p1.y * p2.x)) / (p1.y - p2.y))
}

// ----------------------------------------------------------------------------
// Helper functions

pub(crate) fn rulers_color(ui: &Ui) -> Color32 {
    if ui.visuals().dark_mode {
        Color32::from_gray(100).additive()
    } else {
        Color32::from_black_alpha(180)
    }
}

pub(crate) fn vertical_line(pointer: Pos2, transform: &PlotTransform, line_color: Color32) -> Shape {
    let frame = transform.frame();
    Shape::line_segment(
        [pos2(pointer.x, frame.top()), pos2(pointer.x, frame.bottom())],
        (1.0, line_color),
    )
}

pub(crate) fn horizontal_line(pointer: Pos2, transform: &PlotTransform, line_color: Color32) -> Shape {
    let frame = transform.frame();
    Shape::line_segment(
        [pos2(frame.left(), pointer.y), pos2(frame.right(), pointer.y)],
        (1.0, line_color),
    )
}

fn add_rulers_and_text(
    elem: &dyn RectElement,
    plot: &PlotConfig<'_>,
    text: Option<String>,
    shapes: &mut Vec<Shape>,
    cursors: &mut Vec<Cursor>,
) {
    let orientation = elem.orientation();
    let show_argument =
        plot.show_x && orientation == Orientation::Vertical || plot.show_y && orientation == Orientation::Horizontal;
    let show_values =
        plot.show_y && orientation == Orientation::Vertical || plot.show_x && orientation == Orientation::Horizontal;

    // Rulers for argument (usually vertical)
    if show_argument {
        for pos in elem.arguments_with_ruler() {
            cursors.push(match orientation {
                Orientation::Horizontal => Cursor::Horizontal { y: pos.y },
                Orientation::Vertical => Cursor::Vertical { x: pos.x },
            });
        }
    }

    // Rulers for values (usually horizontal)
    if show_values {
        for pos in elem.values_with_ruler() {
            cursors.push(match orientation {
                Orientation::Horizontal => Cursor::Vertical { x: pos.x },
                Orientation::Vertical => Cursor::Horizontal { y: pos.y },
            });
        }
    }

    // Text
    let text = text.unwrap_or({
        let mut text = elem.name().to_owned(); // could be empty

        if show_values {
            text.push('\n');
            text.push_str(&elem.default_values_format(plot.transform));
        }

        text
    });

    let font_id = TextStyle::Body.resolve(plot.ui.style());

    let corner_value = elem.corner_value();
    plot.ui.fonts_mut(|f| {
        shapes.push(Shape::text(
            f,
            plot.transform.position_from_point(&corner_value) + vec2(3.0, -2.0),
            Align2::LEFT_BOTTOM,
            text,
            font_id,
            plot.ui.visuals().text_color(),
        ));
    });
}

/// Draws a cross of horizontal and vertical ruler at the `pointer` position,
/// and a label describing the coordinate.
///
/// `value` is used to for text displaying X/Y coordinates.
pub(super) fn rulers_and_tooltip_at_value(
    plot_area_response: &egui::Response,
    value: PlotPoint,
    name: &str,
    plot: &PlotConfig<'_>,
    cursors: &mut Vec<Cursor>,
    label_formatter: &LabelFormatter<'_>,
) {
    if plot.show_x {
        cursors.push(Cursor::Vertical { x: value.x });
    }
    if plot.show_y {
        cursors.push(Cursor::Horizontal { y: value.y });
    }

    let text = if let Some(custom_label) = label_formatter {
        let label = custom_label(name, &value);
        if label.is_empty() {
            return;
        }
        label
    } else {
        let prefix = if name.is_empty() {
            String::new()
        } else {
            format!("{name}\n")
        };
        let scale = plot.transform.dvalue_dpos();
        let x_decimals = ((-scale[0].abs().log10()).ceil().at_least(0.0) as usize).clamp(1, 6);
        let y_decimals = ((-scale[1].abs().log10()).ceil().at_least(0.0) as usize).clamp(1, 6);
        if plot.show_x && plot.show_y {
            format!(
                "{}x = {:.*}\ny = {:.*}",
                prefix, x_decimals, value.x, y_decimals, value.y
            )
        } else if plot.show_x {
            format!("{}x = {:.*}", prefix, x_decimals, value.x)
        } else if plot.show_y {
            format!("{}y = {:.*}", prefix, y_decimals, value.y)
        } else {
            unreachable!()
        }
    };

    // We show the tooltip as soon as we're hovering the plot area:
    let mut tooltip = egui::Tooltip::always_open(
        plot_area_response.ctx.clone(),
        plot_area_response.layer_id,
        plot_area_response.id,
        PopupAnchor::Pointer,
    );

    let tooltip_width = plot_area_response.ctx.style().spacing.tooltip_width;

    tooltip.popup = tooltip.popup.width(tooltip_width);

    tooltip.gap(12.0).show(|ui| {
        ui.set_max_width(tooltip_width);
        ui.label(text);
    });
}

fn find_closest_rect<'a, T>(
    rects: impl IntoIterator<Item = &'a T>,
    point: Pos2,
    transform: &PlotTransform,
) -> Option<ClosestElem>
where
    T: 'a + RectElement,
{
    rects
        .into_iter()
        .enumerate()
        .map(|(index, bar)| {
            let bar_rect = transform.rect_from_values(&bar.bounds_min(), &bar.bounds_max());
            let dist_sq = bar_rect.distance_sq_to_pos(point);

            ClosestElem { index, dist_sq }
        })
        .min_by_key(|e| e.dist_sq.ord())
}
