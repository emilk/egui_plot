//! Contains items that can be added to a plot.
#![allow(clippy::type_complexity)] // TODO(emilk): simplify some of the callback types with type aliases

use std::{ops::RangeInclusive, sync::Arc};

use egui::{
    Align2, Color32, CornerRadius, Id, ImageOptions, Mesh, NumExt as _, PopupAnchor, Pos2, Rect,
    Rgba, Shape, Stroke, TextStyle, TextureId, Ui, Vec2, WidgetText,
    emath::Rot2,
    epaint::{CircleShape, PathStroke, TextShape},
    pos2, vec2,
};

use emath::Float as _;
use rect_elem::{RectElement, highlighted_color};

use super::{Cursor, LabelFormatter, PlotBounds, PlotTransform};

pub use bar::Bar;
pub use box_elem::{BoxElem, BoxSpread};
pub use values::{
    ClosestElem, LineStyle, MarkerShape, Orientation, PlotGeometry, PlotPoint, PlotPoints,
};

mod bar;
mod box_elem;
mod rect_elem;
mod values;

const DEFAULT_FILL_ALPHA: f32 = 0.05;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlotItemBase {
    name: String,
    id: Id,
    highlight: bool,
    allow_hover: bool,
}

impl PlotItemBase {
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

macro_rules! builder_methods_for_base {
    () => {
        /// Name of this plot item.
        ///
        /// This name will show up in the plot legend, if legends are turned on.
        #[allow(clippy::needless_pass_by_value)]
        #[inline]
        pub fn name(mut self, name: impl ToString) -> Self {
            self.base_mut().name = name.to_string();
            self
        }

        /// Highlight this plot item, typically by scaling it up.
        ///
        /// If false, the item may still be highlighted via user interaction.
        #[inline]
        pub fn highlight(mut self, highlight: bool) -> Self {
            self.base_mut().highlight = highlight;
            self
        }

        /// Allowed hovering this item in the plot. Default: `true`.
        #[inline]
        pub fn allow_hover(mut self, hovering: bool) -> Self {
            self.base_mut().allow_hover = hovering;
            self
        }

        /// Sets the id of this plot item.
        ///
        /// By default the id is determined from the name, but it can be explicitly set to a different value.
        #[inline]
        pub fn id(mut self, id: impl Into<Id>) -> Self {
            self.base_mut().id = id.into();
            self
        }
    };
}

/// Container to pass-through several parameters related to plot visualization
pub struct PlotConfig<'a> {
    pub ui: &'a Ui,
    pub transform: &'a PlotTransform,
    pub show_x: bool,
    pub show_y: bool,
}

/// Trait shared by things that can be drawn in the plot.
pub trait PlotItem {
    fn shapes(&self, ui: &Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>);

    /// For plot-items which are generated based on x values (plotting functions).
    fn initialize(&mut self, x_range: RangeInclusive<f64>);

    fn name(&self) -> &str {
        &self.base().name
    }

    fn color(&self) -> Color32;

    fn highlight(&mut self) {
        self.base_mut().highlight = true;
    }

    fn highlighted(&self) -> bool {
        self.base().highlight
    }

    /// Can the user hover this item?
    fn allow_hover(&self) -> bool {
        self.base().allow_hover
    }

    fn geometry(&self) -> PlotGeometry<'_>;

    fn bounds(&self) -> PlotBounds;

    fn base(&self) -> &PlotItemBase;

    fn base_mut(&mut self) -> &mut PlotItemBase;

    fn id(&self) -> Id {
        self.base().id
    }

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

        // this method is only called, if the value is in the result set of find_closest()
        let value = points[elem.index];
        let pointer = plot.transform.position_from_point(&value);
        shapes.push(Shape::circle_filled(pointer, 3.0, line_color));

        rulers_and_tooltip_at_value(
            plot_area_response,
            value,
            self.name(),
            plot,
            cursors,
            label_formatter,
        );
    }
}

// ----------------------------------------------------------------------------

/// A horizontal line in a plot, filling the full width
#[derive(Clone, Debug, PartialEq)]
pub struct HLine {
    base: PlotItemBase,
    pub(super) y: f64,
    pub(super) stroke: Stroke,
    pub(super) style: LineStyle,
}

impl HLine {
    pub fn new(name: impl Into<String>, y: impl Into<f64>) -> Self {
        Self {
            base: PlotItemBase::new(name.into()),
            y: y.into(),
            stroke: Stroke::new(1.0, Color32::TRANSPARENT),
            style: LineStyle::Solid,
        }
    }

    /// Add a stroke.
    #[inline]
    pub fn stroke(mut self, stroke: impl Into<Stroke>) -> Self {
        self.stroke = stroke.into();
        self
    }

    /// Stroke width. A high value means the plot thickens.
    #[inline]
    pub fn width(mut self, width: impl Into<f32>) -> Self {
        self.stroke.width = width.into();
        self
    }

    /// Stroke color. Default is `Color32::TRANSPARENT` which means a color will be auto-assigned.
    #[inline]
    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        self.stroke.color = color.into();
        self
    }

    /// Set the line's style. Default is `LineStyle::Solid`.
    #[inline]
    pub fn style(mut self, style: LineStyle) -> Self {
        self.style = style;
        self
    }

    builder_methods_for_base!();
}

impl PlotItem for HLine {
    fn shapes(&self, _ui: &Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        let Self {
            base,
            y,
            stroke,
            style,
            ..
        } = self;

        let points = vec![
            transform.position_from_point(&PlotPoint::new(transform.bounds().min[0], *y)),
            transform.position_from_point(&PlotPoint::new(transform.bounds().max[0], *y)),
        ];
        style.style_line(
            points,
            PathStroke::new(stroke.width, stroke.color),
            base.highlight,
            shapes,
        );
    }

    fn initialize(&mut self, _x_range: RangeInclusive<f64>) {}

    fn color(&self) -> Color32 {
        self.stroke.color
    }

    fn base(&self) -> &PlotItemBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut PlotItemBase {
        &mut self.base
    }

    fn geometry(&self) -> PlotGeometry<'_> {
        PlotGeometry::None
    }

    fn bounds(&self) -> PlotBounds {
        let mut bounds = PlotBounds::NOTHING;
        bounds.min[1] = self.y;
        bounds.max[1] = self.y;
        bounds
    }
}

/// A vertical line in a plot, filling the full width
#[derive(Clone, Debug, PartialEq)]
pub struct VLine {
    base: PlotItemBase,
    pub(super) x: f64,
    pub(super) stroke: Stroke,
    pub(super) style: LineStyle,
}

impl VLine {
    pub fn new(name: impl Into<String>, x: impl Into<f64>) -> Self {
        Self {
            base: PlotItemBase::new(name.into()),
            x: x.into(),
            stroke: Stroke::new(1.0, Color32::TRANSPARENT),
            style: LineStyle::Solid,
        }
    }

    /// Add a stroke.
    #[inline]
    pub fn stroke(mut self, stroke: impl Into<Stroke>) -> Self {
        self.stroke = stroke.into();
        self
    }

    /// Stroke width. A high value means the plot thickens.
    #[inline]
    pub fn width(mut self, width: impl Into<f32>) -> Self {
        self.stroke.width = width.into();
        self
    }

    /// Stroke color. Default is `Color32::TRANSPARENT` which means a color will be auto-assigned.
    #[inline]
    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        self.stroke.color = color.into();
        self
    }

    /// Set the line's style. Default is `LineStyle::Solid`.
    #[inline]
    pub fn style(mut self, style: LineStyle) -> Self {
        self.style = style;
        self
    }

    builder_methods_for_base!();
}

impl PlotItem for VLine {
    fn shapes(&self, _ui: &Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        let Self {
            base,
            x,
            stroke,
            style,
            ..
        } = self;

        let points = vec![
            transform.position_from_point(&PlotPoint::new(*x, transform.bounds().min[1])),
            transform.position_from_point(&PlotPoint::new(*x, transform.bounds().max[1])),
        ];
        style.style_line(
            points,
            PathStroke::new(stroke.width, stroke.color),
            base.highlight,
            shapes,
        );
    }

    fn initialize(&mut self, _x_range: RangeInclusive<f64>) {}

    fn color(&self) -> Color32 {
        self.stroke.color
    }

    fn base(&self) -> &PlotItemBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut PlotItemBase {
        &mut self.base
    }

    fn geometry(&self) -> PlotGeometry<'_> {
        PlotGeometry::None
    }

    fn bounds(&self) -> PlotBounds {
        let mut bounds = PlotBounds::NOTHING;
        bounds.min[0] = self.x;
        bounds.max[0] = self.x;
        bounds
    }
}

/// A series of values forming a path.
pub struct Line<'a> {
    base: PlotItemBase,
    pub(super) series: PlotPoints<'a>,
    pub(super) stroke: Stroke,
    pub(super) fill: Option<f32>,
    pub(super) fill_alpha: f32,
    pub(super) gradient_color: Option<Arc<dyn Fn(PlotPoint) -> Color32 + Send + Sync>>,
    pub(super) gradient_fill: bool,
    pub(super) style: LineStyle,
}

impl<'a> Line<'a> {
    pub fn new(name: impl Into<String>, series: impl Into<PlotPoints<'a>>) -> Self {
        Self {
            base: PlotItemBase::new(name.into()),
            series: series.into(),
            stroke: Stroke::new(1.5, Color32::TRANSPARENT), // Note: a stroke of 1.0 (or less) can look bad on low-dpi-screens
            fill: None,
            fill_alpha: DEFAULT_FILL_ALPHA,
            gradient_color: None,
            gradient_fill: false,
            style: LineStyle::Solid,
        }
    }

    /// Add a stroke.
    #[inline]
    pub fn stroke(mut self, stroke: impl Into<Stroke>) -> Self {
        self.stroke = stroke.into();
        self
    }

    /// Add an optional gradient color to the stroke using a callback. The callback
    /// receives a `PlotPoint` as input with the current X and Y values and should
    /// return a `Color32` to be used as the stroke color for that point.
    ///
    /// Setting the `gradient_fill` parameter to `true` will use the gradient
    /// color callback for the fill area as well when `fill()` is set.
    #[inline]
    pub fn gradient_color(
        mut self,
        callback: Arc<dyn Fn(PlotPoint) -> Color32 + Send + Sync>,
        gradient_fill: bool,
    ) -> Self {
        self.gradient_color = Some(callback);
        self.gradient_fill = gradient_fill;
        self
    }

    /// Stroke width. A high value means the plot thickens.
    #[inline]
    pub fn width(mut self, width: impl Into<f32>) -> Self {
        self.stroke.width = width.into();
        self
    }

    /// Stroke color. Default is `Color32::TRANSPARENT` which means a color will be auto-assigned.
    #[inline]
    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        self.stroke.color = color.into();
        self
    }

    /// Fill the area between this line and a given horizontal reference line.
    #[inline]
    pub fn fill(mut self, y_reference: impl Into<f32>) -> Self {
        self.fill = Some(y_reference.into());
        self
    }

    /// Set the fill area's alpha channel. Default is `0.05`.
    #[inline]
    pub fn fill_alpha(mut self, alpha: impl Into<f32>) -> Self {
        self.fill_alpha = alpha.into();
        self
    }

    /// Set the line's style. Default is `LineStyle::Solid`.
    #[inline]
    pub fn style(mut self, style: LineStyle) -> Self {
        self.style = style;
        self
    }

    builder_methods_for_base!();
}

/// Returns the x-coordinate of a possible intersection between a line segment from `p1` to `p2` and
/// a horizontal line at the given y-coordinate.
fn y_intersection(p1: &Pos2, p2: &Pos2, y: f32) -> Option<f32> {
    ((p1.y > y && p2.y < y) || (p1.y < y && p2.y > y))
        .then_some(((y * (p1.x - p2.x)) - (p1.x * p2.y - p1.y * p2.x)) / (p1.y - p2.y))
}

impl PlotItem for Line<'_> {
    fn shapes(&self, _ui: &Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        let Self {
            base,
            series,
            stroke,
            fill,
            gradient_fill,
            style,
            ..
        } = self;
        let mut fill = *fill;

        let mut final_stroke: PathStroke = (*stroke).into();
        // if we have a gradient color, we need to wrap the stroke callback to transpose the position to a value
        // the caller can reason about
        if let Some(gradient_callback) = self.gradient_color.clone() {
            let local_transform = *transform;
            let wrapped_callback = move |_rec: Rect, pos: Pos2| -> Color32 {
                let point = local_transform.value_from_position(pos);
                gradient_callback(point)
            };
            final_stroke = PathStroke::new_uv(stroke.width, wrapped_callback.clone());
        }

        let values_tf: Vec<_> = series
            .points()
            .iter()
            .map(|v| transform.position_from_point(v))
            .collect();
        let n_values = values_tf.len();

        // Fill the area between the line and a reference line, if required.
        if n_values < 2 {
            fill = None;
        }
        if let Some(y_reference) = fill {
            let mut fill_alpha = self.fill_alpha;
            if base.highlight {
                fill_alpha = (2.0 * fill_alpha).at_most(1.0);
            }
            let y = transform
                .position_from_point(&PlotPoint::new(0.0, y_reference))
                .y;
            let mut fill_color = Rgba::from(stroke.color)
                .to_opaque()
                .multiply(fill_alpha)
                .into();
            let mut mesh = Mesh::default();
            let expected_intersections = 20;
            mesh.reserve_triangles((n_values - 1) * 2);
            mesh.reserve_vertices(n_values * 2 + expected_intersections);
            values_tf.windows(2).for_each(|w| {
                if *gradient_fill && self.gradient_color.is_some() {
                    fill_color = Rgba::from(self
                        .gradient_color
                        .clone()
                        .expect("Could not find gradient color callback")(
                        transform.value_from_position(w[1]),
                    ))
                    .to_opaque()
                    .multiply(fill_alpha)
                    .into();
                }
                let i = mesh.vertices.len() as u32;
                mesh.colored_vertex(w[0], fill_color);
                mesh.colored_vertex(pos2(w[0].x, y), fill_color);
                if let Some(x) = y_intersection(&w[0], &w[1], y) {
                    let point = pos2(x, y);
                    mesh.colored_vertex(point, fill_color);
                    mesh.add_triangle(i, i + 1, i + 2);
                    mesh.add_triangle(i + 2, i + 3, i + 4);
                } else {
                    mesh.add_triangle(i, i + 1, i + 2);
                    mesh.add_triangle(i + 1, i + 2, i + 3);
                }
            });
            let last = values_tf[n_values - 1];
            mesh.colored_vertex(last, fill_color);
            mesh.colored_vertex(pos2(last.x, y), fill_color);
            shapes.push(Shape::Mesh(std::sync::Arc::new(mesh)));
        }
        style.style_line(values_tf, final_stroke, base.highlight, shapes);
    }

    fn initialize(&mut self, x_range: RangeInclusive<f64>) {
        self.series.generate_points(x_range);
    }

    fn color(&self) -> Color32 {
        self.stroke.color
    }

    fn base(&self) -> &PlotItemBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut PlotItemBase {
        &mut self.base
    }

    fn geometry(&self) -> PlotGeometry<'_> {
        PlotGeometry::Points(self.series.points())
    }

    fn bounds(&self) -> PlotBounds {
        self.series.bounds()
    }
}

/// A convex polygon.
pub struct Polygon<'a> {
    base: PlotItemBase,
    pub(super) series: PlotPoints<'a>,
    pub(super) stroke: Stroke,
    pub(super) fill_color: Option<Color32>,
    pub(super) style: LineStyle,
}

impl<'a> Polygon<'a> {
    pub fn new(name: impl Into<String>, series: impl Into<PlotPoints<'a>>) -> Self {
        Self {
            base: PlotItemBase::new(name.into()),
            series: series.into(),
            stroke: Stroke::new(1.0, Color32::TRANSPARENT),
            fill_color: None,
            style: LineStyle::Solid,
        }
    }

    /// Add a custom stroke.
    #[inline]
    pub fn stroke(mut self, stroke: impl Into<Stroke>) -> Self {
        self.stroke = stroke.into();
        self
    }

    /// Set the stroke width.
    #[inline]
    pub fn width(mut self, width: impl Into<f32>) -> Self {
        self.stroke.width = width.into();
        self
    }

    /// Fill color. Defaults to the stroke color with added transparency.
    #[inline]
    pub fn fill_color(mut self, color: impl Into<Color32>) -> Self {
        self.fill_color = Some(color.into());
        self
    }

    /// Set the outline's style. Default is `LineStyle::Solid`.
    #[inline]
    pub fn style(mut self, style: LineStyle) -> Self {
        self.style = style;
        self
    }

    builder_methods_for_base!();
}

impl PlotItem for Polygon<'_> {
    fn shapes(&self, _ui: &Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        let Self {
            base,
            series,
            stroke,
            fill_color,
            style,
            ..
        } = self;

        let mut values_tf: Vec<_> = series
            .points()
            .iter()
            .map(|v| transform.position_from_point(v))
            .collect();

        let fill_color = fill_color.unwrap_or(stroke.color.linear_multiply(DEFAULT_FILL_ALPHA));

        let shape = Shape::convex_polygon(values_tf.clone(), fill_color, Stroke::NONE);
        shapes.push(shape);

        if let Some(first) = values_tf.first() {
            values_tf.push(*first); // close the polygon
        }

        style.style_line(
            values_tf,
            PathStroke::new(stroke.width, stroke.color),
            base.highlight,
            shapes,
        );
    }

    fn initialize(&mut self, x_range: RangeInclusive<f64>) {
        self.series.generate_points(x_range);
    }

    fn color(&self) -> Color32 {
        self.stroke.color
    }

    fn geometry(&self) -> PlotGeometry<'_> {
        PlotGeometry::Points(self.series.points())
    }

    fn bounds(&self) -> PlotBounds {
        self.series.bounds()
    }

    fn base(&self) -> &PlotItemBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut PlotItemBase {
        &mut self.base
    }
}

/// Text inside the plot.
#[derive(Clone)]
pub struct Text {
    base: PlotItemBase,
    pub(super) text: WidgetText,
    pub(super) position: PlotPoint,
    pub(super) color: Color32,
    pub(super) anchor: Align2,
}

impl Text {
    pub fn new(name: impl Into<String>, position: PlotPoint, text: impl Into<WidgetText>) -> Self {
        Self {
            base: PlotItemBase::new(name.into()),
            text: text.into(),
            position,
            color: Color32::TRANSPARENT,
            anchor: Align2::CENTER_CENTER,
        }
    }

    /// Text color.
    #[inline]
    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        self.color = color.into();
        self
    }

    /// Anchor position of the text. Default is `Align2::CENTER_CENTER`.
    #[inline]
    pub fn anchor(mut self, anchor: Align2) -> Self {
        self.anchor = anchor;
        self
    }

    builder_methods_for_base!();
}

impl PlotItem for Text {
    fn shapes(&self, ui: &Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        let color = if self.color == Color32::TRANSPARENT {
            ui.style().visuals.text_color()
        } else {
            self.color
        };

        let galley = self.text.clone().into_galley(
            ui,
            Some(egui::TextWrapMode::Extend),
            f32::INFINITY,
            TextStyle::Small,
        );

        let pos = transform.position_from_point(&self.position);
        let rect = self.anchor.anchor_size(pos, galley.size());

        shapes.push(TextShape::new(rect.min, galley, color).into());

        if self.base.highlight {
            shapes.push(Shape::rect_stroke(
                rect.expand(1.0),
                1.0,
                Stroke::new(0.5, color),
                egui::StrokeKind::Outside,
            ));
        }
    }

    fn initialize(&mut self, _x_range: RangeInclusive<f64>) {}

    fn color(&self) -> Color32 {
        self.color
    }

    fn geometry(&self) -> PlotGeometry<'_> {
        PlotGeometry::None
    }

    fn bounds(&self) -> PlotBounds {
        let mut bounds = PlotBounds::NOTHING;
        bounds.extend_with(&self.position);
        bounds
    }

    fn base(&self) -> &PlotItemBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut PlotItemBase {
        &mut self.base
    }
}

/// A set of points.
pub struct Points<'a> {
    base: PlotItemBase,

    pub(super) series: PlotPoints<'a>,

    pub(super) shape: MarkerShape,

    /// Color of the marker. `Color32::TRANSPARENT` means that it will be picked automatically.
    pub(super) color: Color32,

    /// Whether to fill the marker. Does not apply to all types.
    pub(super) filled: bool,

    /// The maximum extent of the marker from its center.
    pub(super) radius: f32,

    pub(super) stems: Option<f32>,
}

impl<'a> Points<'a> {
    pub fn new(name: impl Into<String>, series: impl Into<PlotPoints<'a>>) -> Self {
        Self {
            base: PlotItemBase::new(name.into()),
            series: series.into(),
            shape: MarkerShape::Circle,
            color: Color32::TRANSPARENT,
            filled: true,
            radius: 1.0,
            stems: None,
        }
    }

    /// Set the shape of the markers.
    #[inline]
    pub fn shape(mut self, shape: MarkerShape) -> Self {
        self.shape = shape;
        self
    }

    /// Set the marker's color.
    #[inline]
    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        self.color = color.into();
        self
    }

    /// Whether to fill the marker.
    #[inline]
    pub fn filled(mut self, filled: bool) -> Self {
        self.filled = filled;
        self
    }

    /// Whether to add stems between the markers and a horizontal reference line.
    #[inline]
    pub fn stems(mut self, y_reference: impl Into<f32>) -> Self {
        self.stems = Some(y_reference.into());
        self
    }

    /// Set the maximum extent of the marker around its position, in ui points.
    #[inline]
    pub fn radius(mut self, radius: impl Into<f32>) -> Self {
        self.radius = radius.into();
        self
    }

    builder_methods_for_base!();
}

impl PlotItem for Points<'_> {
    #[allow(clippy::too_many_lines)] // TODO(emilk): shorten this function
    fn shapes(&self, _ui: &Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        let sqrt_3 = 3_f32.sqrt();
        let frac_sqrt_3_2 = 3_f32.sqrt() / 2.0;
        let frac_1_sqrt_2 = 1.0 / 2_f32.sqrt();

        let Self {
            base,
            series,
            shape,
            color,
            filled,
            radius,
            stems,
            ..
        } = self;

        let mut radius = *radius;

        let stroke_size = radius / 5.0;

        let default_stroke = Stroke::new(stroke_size, *color);
        let mut stem_stroke = default_stroke;
        let (fill, stroke) = if *filled {
            (*color, Stroke::NONE)
        } else {
            (Color32::TRANSPARENT, default_stroke)
        };

        if base.highlight {
            radius *= 2f32.sqrt();
            stem_stroke.width *= 2.0;
        }

        let y_reference = stems.map(|y| transform.position_from_point(&PlotPoint::new(0.0, y)).y);

        series
            .points()
            .iter()
            .map(|value| transform.position_from_point(value))
            .for_each(|center| {
                let tf = |dx: f32, dy: f32| -> Pos2 { center + radius * vec2(dx, dy) };

                if let Some(y) = y_reference {
                    let stem = Shape::line_segment([center, pos2(center.x, y)], stem_stroke);
                    shapes.push(stem);
                }

                match shape {
                    MarkerShape::Circle => {
                        shapes.push(Shape::Circle(CircleShape {
                            center,
                            radius,
                            fill,
                            stroke,
                        }));
                    }
                    MarkerShape::Diamond => {
                        let points = vec![
                            tf(0.0, 1.0),  // bottom
                            tf(-1.0, 0.0), // left
                            tf(0.0, -1.0), // top
                            tf(1.0, 0.0),  // right
                        ];
                        shapes.push(Shape::convex_polygon(points, fill, stroke));
                    }
                    MarkerShape::Square => {
                        let points = vec![
                            tf(-frac_1_sqrt_2, frac_1_sqrt_2),
                            tf(-frac_1_sqrt_2, -frac_1_sqrt_2),
                            tf(frac_1_sqrt_2, -frac_1_sqrt_2),
                            tf(frac_1_sqrt_2, frac_1_sqrt_2),
                        ];
                        shapes.push(Shape::convex_polygon(points, fill, stroke));
                    }
                    MarkerShape::Cross => {
                        let diagonal1 = [
                            tf(-frac_1_sqrt_2, -frac_1_sqrt_2),
                            tf(frac_1_sqrt_2, frac_1_sqrt_2),
                        ];
                        let diagonal2 = [
                            tf(frac_1_sqrt_2, -frac_1_sqrt_2),
                            tf(-frac_1_sqrt_2, frac_1_sqrt_2),
                        ];
                        shapes.push(Shape::line_segment(diagonal1, default_stroke));
                        shapes.push(Shape::line_segment(diagonal2, default_stroke));
                    }
                    MarkerShape::Plus => {
                        let horizontal = [tf(-1.0, 0.0), tf(1.0, 0.0)];
                        let vertical = [tf(0.0, -1.0), tf(0.0, 1.0)];
                        shapes.push(Shape::line_segment(horizontal, default_stroke));
                        shapes.push(Shape::line_segment(vertical, default_stroke));
                    }
                    MarkerShape::Up => {
                        let points =
                            vec![tf(0.0, -1.0), tf(0.5 * sqrt_3, 0.5), tf(-0.5 * sqrt_3, 0.5)];
                        shapes.push(Shape::convex_polygon(points, fill, stroke));
                    }
                    MarkerShape::Down => {
                        let points = vec![
                            tf(0.0, 1.0),
                            tf(-0.5 * sqrt_3, -0.5),
                            tf(0.5 * sqrt_3, -0.5),
                        ];
                        shapes.push(Shape::convex_polygon(points, fill, stroke));
                    }
                    MarkerShape::Left => {
                        let points =
                            vec![tf(-1.0, 0.0), tf(0.5, -0.5 * sqrt_3), tf(0.5, 0.5 * sqrt_3)];
                        shapes.push(Shape::convex_polygon(points, fill, stroke));
                    }
                    MarkerShape::Right => {
                        let points = vec![
                            tf(1.0, 0.0),
                            tf(-0.5, 0.5 * sqrt_3),
                            tf(-0.5, -0.5 * sqrt_3),
                        ];
                        shapes.push(Shape::convex_polygon(points, fill, stroke));
                    }
                    MarkerShape::Asterisk => {
                        let vertical = [tf(0.0, -1.0), tf(0.0, 1.0)];
                        let diagonal1 = [tf(-frac_sqrt_3_2, 0.5), tf(frac_sqrt_3_2, -0.5)];
                        let diagonal2 = [tf(-frac_sqrt_3_2, -0.5), tf(frac_sqrt_3_2, 0.5)];
                        shapes.push(Shape::line_segment(vertical, default_stroke));
                        shapes.push(Shape::line_segment(diagonal1, default_stroke));
                        shapes.push(Shape::line_segment(diagonal2, default_stroke));
                    }
                }
            });
    }

    fn initialize(&mut self, x_range: RangeInclusive<f64>) {
        self.series.generate_points(x_range);
    }

    fn color(&self) -> Color32 {
        self.color
    }

    fn geometry(&self) -> PlotGeometry<'_> {
        PlotGeometry::Points(self.series.points())
    }

    fn bounds(&self) -> PlotBounds {
        self.series.bounds()
    }

    fn base(&self) -> &PlotItemBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut PlotItemBase {
        &mut self.base
    }
}

/// A set of arrows.
pub struct Arrows<'a> {
    base: PlotItemBase,
    pub(super) origins: PlotPoints<'a>,
    pub(super) tips: PlotPoints<'a>,
    pub(super) tip_length: Option<f32>,
    pub(super) color: Color32,
}

impl<'a> Arrows<'a> {
    pub fn new(
        name: impl Into<String>,
        origins: impl Into<PlotPoints<'a>>,
        tips: impl Into<PlotPoints<'a>>,
    ) -> Self {
        Self {
            base: PlotItemBase::new(name.into()),
            origins: origins.into(),
            tips: tips.into(),
            tip_length: None,
            color: Color32::TRANSPARENT,
        }
    }

    /// Set the length of the arrow tips
    #[inline]
    pub fn tip_length(mut self, tip_length: f32) -> Self {
        self.tip_length = Some(tip_length);
        self
    }

    /// Set the arrows' color.
    #[inline]
    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        self.color = color.into();
        self
    }

    builder_methods_for_base!();
}

impl PlotItem for Arrows<'_> {
    fn shapes(&self, _ui: &Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        let Self {
            origins,
            tips,
            tip_length,
            color,
            base,
            ..
        } = self;
        let stroke = Stroke::new(if base.highlight { 2.0 } else { 1.0 }, *color);
        origins
            .points()
            .iter()
            .zip(tips.points().iter())
            .map(|(origin, tip)| {
                (
                    transform.position_from_point(origin),
                    transform.position_from_point(tip),
                )
            })
            .for_each(|(origin, tip)| {
                let vector = tip - origin;
                let rot = Rot2::from_angle(std::f32::consts::TAU / 10.0);
                let tip_length = if let Some(tip_length) = tip_length {
                    *tip_length
                } else {
                    vector.length() / 4.0
                };
                let tip = origin + vector;
                let dir = vector.normalized();
                shapes.push(Shape::line_segment([origin, tip], stroke));
                shapes.push(Shape::line(
                    vec![
                        tip - tip_length * (rot.inverse() * dir),
                        tip,
                        tip - tip_length * (rot * dir),
                    ],
                    stroke,
                ));
            });
    }

    fn initialize(&mut self, _x_range: RangeInclusive<f64>) {
        self.origins
            .generate_points(f64::NEG_INFINITY..=f64::INFINITY);
        self.tips.generate_points(f64::NEG_INFINITY..=f64::INFINITY);
    }

    fn color(&self) -> Color32 {
        self.color
    }

    fn geometry(&self) -> PlotGeometry<'_> {
        PlotGeometry::Points(self.origins.points())
    }

    fn bounds(&self) -> PlotBounds {
        self.origins.bounds()
    }

    fn base(&self) -> &PlotItemBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut PlotItemBase {
        &mut self.base
    }
}

/// An image in the plot.
#[derive(Clone)]
pub struct PlotImage {
    base: PlotItemBase,
    pub(super) position: PlotPoint,
    pub(super) texture_id: TextureId,
    pub(super) uv: Rect,
    pub(super) size: Vec2,
    pub(crate) rotation: f64,
    pub(super) bg_fill: Color32,
    pub(super) tint: Color32,
}

impl PlotImage {
    /// Create a new image with position and size in plot coordinates.
    pub fn new(
        name: impl Into<String>,
        texture_id: impl Into<TextureId>,
        center_position: PlotPoint,
        size: impl Into<Vec2>,
    ) -> Self {
        Self {
            base: PlotItemBase::new(name.into()),
            position: center_position,
            texture_id: texture_id.into(),
            uv: Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
            size: size.into(),
            rotation: 0.0,
            bg_fill: Default::default(),
            tint: Color32::WHITE,
        }
    }

    /// Select UV range. Default is (0,0) in top-left, (1,1) bottom right.
    #[inline]
    pub fn uv(mut self, uv: impl Into<Rect>) -> Self {
        self.uv = uv.into();
        self
    }

    /// A solid color to put behind the image. Useful for transparent images.
    #[inline]
    pub fn bg_fill(mut self, bg_fill: impl Into<Color32>) -> Self {
        self.bg_fill = bg_fill.into();
        self
    }

    /// Multiply image color with this. Default is WHITE (no tint).
    #[inline]
    pub fn tint(mut self, tint: impl Into<Color32>) -> Self {
        self.tint = tint.into();
        self
    }

    /// Rotate the image counter-clockwise around its center by an angle in radians.
    #[inline]
    pub fn rotate(mut self, angle: f64) -> Self {
        self.rotation = angle;
        self
    }

    builder_methods_for_base!();
}

impl PlotItem for PlotImage {
    fn shapes(&self, ui: &Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        let Self {
            position,
            rotation,
            texture_id,
            uv,
            size,
            bg_fill,
            tint,
            base,
            ..
        } = self;
        let image_screen_rect = {
            let left_top = PlotPoint::new(
                position.x - 0.5 * size.x as f64,
                position.y - 0.5 * size.y as f64,
            );
            let right_bottom = PlotPoint::new(
                position.x + 0.5 * size.x as f64,
                position.y + 0.5 * size.y as f64,
            );
            let left_top_screen = transform.position_from_point(&left_top);
            let right_bottom_screen = transform.position_from_point(&right_bottom);
            Rect::from_two_pos(left_top_screen, right_bottom_screen)
        };
        let screen_rotation = -*rotation as f32;

        egui::paint_texture_at(
            ui.painter(),
            image_screen_rect,
            &ImageOptions {
                uv: *uv,
                bg_fill: *bg_fill,
                tint: *tint,
                rotation: Some((Rot2::from_angle(screen_rotation), Vec2::splat(0.5))),
                corner_radius: CornerRadius::ZERO,
            },
            &(*texture_id, image_screen_rect.size()).into(),
        );
        if base.highlight {
            let center = image_screen_rect.center();
            let rotation = Rot2::from_angle(screen_rotation);
            let outline = [
                image_screen_rect.right_bottom(),
                image_screen_rect.right_top(),
                image_screen_rect.left_top(),
                image_screen_rect.left_bottom(),
            ]
            .iter()
            .map(|point| center + rotation * (*point - center))
            .collect();
            shapes.push(Shape::closed_line(
                outline,
                Stroke::new(1.0, ui.visuals().strong_text_color()),
            ));
        }
    }

    fn initialize(&mut self, _x_range: RangeInclusive<f64>) {}

    fn color(&self) -> Color32 {
        Color32::TRANSPARENT
    }

    fn geometry(&self) -> PlotGeometry<'_> {
        PlotGeometry::None
    }

    fn bounds(&self) -> PlotBounds {
        let mut bounds = PlotBounds::NOTHING;
        let left_top = PlotPoint::new(
            self.position.x as f32 - self.size.x / 2.0,
            self.position.y as f32 - self.size.y / 2.0,
        );
        let right_bottom = PlotPoint::new(
            self.position.x as f32 + self.size.x / 2.0,
            self.position.y as f32 + self.size.y / 2.0,
        );
        bounds.extend_with(&left_top);
        bounds.extend_with(&right_bottom);
        bounds
    }

    fn base(&self) -> &PlotItemBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut PlotItemBase {
        &mut self.base
    }
}

// ----------------------------------------------------------------------------

/// A bar chart.
pub struct BarChart {
    base: PlotItemBase,

    pub(super) bars: Vec<Bar>,
    default_color: Color32,

    /// A custom element formatter
    pub(super) element_formatter: Option<Box<dyn Fn(&Bar, &BarChart) -> String>>,
}

impl BarChart {
    /// Create a bar chart. It defaults to vertically oriented elements.
    pub fn new(name: impl Into<String>, bars: Vec<Bar>) -> Self {
        Self {
            base: PlotItemBase::new(name.into()),
            bars,
            default_color: Color32::TRANSPARENT,
            element_formatter: None,
        }
    }

    /// Set the default color. It is set on all elements that do not already have a specific color.
    /// This is the color that shows up in the legend.
    /// It can be overridden at the bar level (see [[`Bar`]]).
    /// Default is `Color32::TRANSPARENT` which means a color will be auto-assigned.
    #[inline]
    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        let plot_color = color.into();
        self.default_color = plot_color;
        for b in &mut self.bars {
            if b.fill == Color32::TRANSPARENT && b.stroke.color == Color32::TRANSPARENT {
                b.fill = plot_color.linear_multiply(0.2);
                b.stroke.color = plot_color;
            }
        }
        self
    }

    /// Set all elements to be in a vertical orientation.
    /// Argument axis will be X and bar values will be on the Y axis.
    #[inline]
    pub fn vertical(mut self) -> Self {
        for b in &mut self.bars {
            b.orientation = Orientation::Vertical;
        }
        self
    }

    /// Set all elements to be in a horizontal orientation.
    /// Argument axis will be Y and bar values will be on the X axis.
    #[inline]
    pub fn horizontal(mut self) -> Self {
        for b in &mut self.bars {
            b.orientation = Orientation::Horizontal;
        }
        self
    }

    /// Set the width (thickness) of all its elements.
    #[inline]
    pub fn width(mut self, width: f64) -> Self {
        for b in &mut self.bars {
            b.bar_width = width;
        }
        self
    }

    /// Add a custom way to format an element.
    /// Can be used to display a set number of decimals or custom labels.
    #[inline]
    pub fn element_formatter(mut self, formatter: Box<dyn Fn(&Bar, &Self) -> String>) -> Self {
        self.element_formatter = Some(formatter);
        self
    }

    /// Stacks the bars on top of another chart.
    /// Positive values are stacked on top of other positive values.
    /// Negative values are stacked below other negative values.
    #[inline]
    pub fn stack_on(mut self, others: &[&Self]) -> Self {
        for (index, bar) in self.bars.iter_mut().enumerate() {
            let new_base_offset = if bar.value.is_sign_positive() {
                others
                    .iter()
                    .filter_map(|other_chart| other_chart.bars.get(index).map(|bar| bar.upper()))
                    .max_by_key(|value| value.ord())
            } else {
                others
                    .iter()
                    .filter_map(|other_chart| other_chart.bars.get(index).map(|bar| bar.lower()))
                    .min_by_key(|value| value.ord())
            };

            if let Some(value) = new_base_offset {
                bar.base_offset = Some(value);
            }
        }
        self
    }

    builder_methods_for_base!();
}

impl PlotItem for BarChart {
    fn shapes(&self, _ui: &Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        for b in &self.bars {
            b.add_shapes(transform, self.base.highlight, shapes);
        }
    }

    fn initialize(&mut self, _x_range: RangeInclusive<f64>) {
        // nothing to do
    }

    fn color(&self) -> Color32 {
        self.default_color
    }

    fn geometry(&self) -> PlotGeometry<'_> {
        PlotGeometry::Rects
    }

    fn bounds(&self) -> PlotBounds {
        let mut bounds = PlotBounds::NOTHING;
        for b in &self.bars {
            bounds.merge(&b.bounds());
        }
        bounds
    }

    fn find_closest(&self, point: Pos2, transform: &PlotTransform) -> Option<ClosestElem> {
        find_closest_rect(&self.bars, point, transform)
    }

    fn on_hover(
        &self,
        _plot_area_response: &egui::Response,
        elem: ClosestElem,
        shapes: &mut Vec<Shape>,
        cursors: &mut Vec<Cursor>,
        plot: &PlotConfig<'_>,
        _: &LabelFormatter<'_>,
    ) {
        let bar = &self.bars[elem.index];

        bar.add_shapes(plot.transform, true, shapes);
        bar.add_rulers_and_text(self, plot, shapes, cursors);
    }

    fn base(&self) -> &PlotItemBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut PlotItemBase {
        &mut self.base
    }
}

/// A diagram containing a series of [`BoxElem`] elements.
pub struct BoxPlot {
    base: PlotItemBase,

    pub(super) boxes: Vec<BoxElem>,
    default_color: Color32,

    /// A custom element formatter
    pub(super) element_formatter: Option<Box<dyn Fn(&BoxElem, &BoxPlot) -> String>>,
}

impl BoxPlot {
    /// Create a plot containing multiple `boxes`. It defaults to vertically oriented elements.
    pub fn new(name: impl Into<String>, boxes: Vec<BoxElem>) -> Self {
        Self {
            base: PlotItemBase::new(name.into()),
            boxes,
            default_color: Color32::TRANSPARENT,
            element_formatter: None,
        }
    }

    /// Set the default color. It is set on all elements that do not already have a specific color.
    /// This is the color that shows up in the legend.
    /// It can be overridden at the element level (see [`BoxElem`]).
    /// Default is `Color32::TRANSPARENT` which means a color will be auto-assigned.
    #[inline]
    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        let plot_color = color.into();
        self.default_color = plot_color;
        for box_elem in &mut self.boxes {
            if box_elem.fill == Color32::TRANSPARENT
                && box_elem.stroke.color == Color32::TRANSPARENT
            {
                box_elem.fill = plot_color.linear_multiply(0.2);
                box_elem.stroke.color = plot_color;
            }
        }
        self
    }

    /// Set all elements to be in a vertical orientation.
    /// Argument axis will be X and values will be on the Y axis.
    #[inline]
    pub fn vertical(mut self) -> Self {
        for box_elem in &mut self.boxes {
            box_elem.orientation = Orientation::Vertical;
        }
        self
    }

    /// Set all elements to be in a horizontal orientation.
    /// Argument axis will be Y and values will be on the X axis.
    #[inline]
    pub fn horizontal(mut self) -> Self {
        for box_elem in &mut self.boxes {
            box_elem.orientation = Orientation::Horizontal;
        }
        self
    }

    /// Add a custom way to format an element.
    /// Can be used to display a set number of decimals or custom labels.
    #[inline]
    pub fn element_formatter(mut self, formatter: Box<dyn Fn(&BoxElem, &Self) -> String>) -> Self {
        self.element_formatter = Some(formatter);
        self
    }

    builder_methods_for_base!();
}

impl PlotItem for BoxPlot {
    fn shapes(&self, _ui: &Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        for b in &self.boxes {
            b.add_shapes(transform, self.base.highlight, shapes);
        }
    }

    fn initialize(&mut self, _x_range: RangeInclusive<f64>) {
        // nothing to do
    }

    fn color(&self) -> Color32 {
        self.default_color
    }

    fn geometry(&self) -> PlotGeometry<'_> {
        PlotGeometry::Rects
    }

    fn bounds(&self) -> PlotBounds {
        let mut bounds = PlotBounds::NOTHING;
        for b in &self.boxes {
            bounds.merge(&b.bounds());
        }
        bounds
    }

    fn find_closest(&self, point: Pos2, transform: &PlotTransform) -> Option<ClosestElem> {
        find_closest_rect(&self.boxes, point, transform)
    }

    fn on_hover(
        &self,
        _plot_area_response: &egui::Response,
        elem: ClosestElem,
        shapes: &mut Vec<Shape>,
        cursors: &mut Vec<Cursor>,
        plot: &PlotConfig<'_>,
        _: &LabelFormatter<'_>,
    ) {
        let box_plot = &self.boxes[elem.index];

        box_plot.add_shapes(plot.transform, true, shapes);
        box_plot.add_rulers_and_text(self, plot, shapes, cursors);
    }

    fn base(&self) -> &PlotItemBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut PlotItemBase {
        &mut self.base
    }
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

pub(crate) fn vertical_line(
    pointer: Pos2,
    transform: &PlotTransform,
    line_color: Color32,
) -> Shape {
    let frame = transform.frame();
    Shape::line_segment(
        [
            pos2(pointer.x, frame.top()),
            pos2(pointer.x, frame.bottom()),
        ],
        (1.0, line_color),
    )
}

pub(crate) fn horizontal_line(
    pointer: Pos2,
    transform: &PlotTransform,
    line_color: Color32,
) -> Shape {
    let frame = transform.frame();
    Shape::line_segment(
        [
            pos2(frame.left(), pointer.y),
            pos2(frame.right(), pointer.y),
        ],
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
    let show_argument = plot.show_x && orientation == Orientation::Vertical
        || plot.show_y && orientation == Orientation::Horizontal;
    let show_values = plot.show_y && orientation == Orientation::Vertical
        || plot.show_x && orientation == Orientation::Horizontal;

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
    plot.ui.fonts(|f| {
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
#[allow(clippy::too_many_arguments)]
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
        custom_label(name, &value)
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
