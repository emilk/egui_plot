use std::ops::RangeInclusive;
use std::sync::Arc;

use egui::Color32;
use egui::Mesh;
use egui::Rgba;
use egui::Shape;
use egui::Stroke;
use egui::Ui;
use egui::epaint::PathStroke;
use emath::NumExt as _;
use emath::Pos2;
use emath::Rect;
use emath::pos2;

use super::DEFAULT_FILL_ALPHA;
use crate::math::y_intersection;
use crate::Id;
use crate::values::LineStyle;
use crate::PlotBounds;
use crate::values::PlotGeometry;
use crate::PlotItem;
use crate::PlotItemBase;
use crate::values::PlotPoint;
use crate::values::PlotPoints;
use crate::PlotTransform;

/// A series of values forming a path.
pub struct Line<'a> {
    base: PlotItemBase,
    pub(crate) series: PlotPoints<'a>,
    pub(crate) stroke: Stroke,
    pub(crate) fill: Option<f32>,
    pub(crate) fill_alpha: f32,
    pub(crate) gradient_color: Option<Arc<dyn Fn(PlotPoint) -> Color32 + Send + Sync>>,
    pub(crate) gradient_fill: bool,
    pub(crate) style: LineStyle,
}

impl<'a> Line<'a> {
    pub fn new(name: impl Into<String>, series: impl Into<PlotPoints<'a>>) -> Self {
        Self {
            base: PlotItemBase::new(name.into()),
            series: series.into(),
            stroke: Stroke::new(1.5, Color32::TRANSPARENT), /* Note: a stroke of 1.0 (or less) can look bad on
                                                             * low-dpi-screens */
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

    /// Add an optional gradient color to the stroke using a callback. The
    /// callback receives a `PlotPoint` as input with the current X and Y
    /// values and should return a `Color32` to be used as the stroke color
    /// for that point.
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

    /// Stroke color. Default is `Color32::TRANSPARENT` which means a color will
    /// be auto-assigned.
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

    /// Name of this plot item.
    ///
    /// This name will show up in the plot legend, if legends are turned on.
    ///
    /// Setting the name via this method does not change the item's id, so you
    /// can use it to change the name dynamically between frames without
    /// losing the item's state. You should make sure the name passed to
    /// [`Self::new`] is unique and stable for each item, or set unique and
    /// stable ids explicitly via [`Self::id`].
    #[expect(clippy::needless_pass_by_value)]
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
    /// By default the id is determined from the name passed to [`Self::new`],
    /// but it can be explicitly set to a different value.
    #[inline]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.base_mut().id = id.into();
        self
    }
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
        // if we have a gradient color, we need to wrap the stroke callback to transpose
        // the position to a value the caller can reason about
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
            let y = transform.position_from_point(&PlotPoint::new(0.0, y_reference)).y;
            let default_fill_color = Rgba::from(stroke.color).to_opaque().multiply(fill_alpha).into();

            let fill_color_for_point = |pos| {
                if *gradient_fill && self.gradient_color.is_some() {
                    Rgba::from(self
                        .gradient_color
                        .clone()
                        .expect("Could not find gradient color callback")(
                        transform.value_from_position(pos),
                    ))
                    .to_opaque()
                    .multiply(fill_alpha)
                    .into()
                } else {
                    default_fill_color
                }
            };

            let mut mesh = Mesh::default();
            let expected_intersections = 20;
            mesh.reserve_triangles((n_values - 1) * 2);
            mesh.reserve_vertices(n_values * 2 + expected_intersections);
            values_tf.windows(2).for_each(|w| {
                let fill_color = fill_color_for_point(w[0]);
                let i = mesh.vertices.len() as u32;
                mesh.colored_vertex(w[0], fill_color);
                mesh.colored_vertex(pos2(w[0].x, y), fill_color);
                if let Some(x) = y_intersection(&w[0], &w[1], y) {
                    let point = pos2(x, y);
                    mesh.colored_vertex(point, fill_color_for_point(point));
                    mesh.add_triangle(i, i + 1, i + 2);
                    mesh.add_triangle(i + 2, i + 3, i + 4);
                } else {
                    mesh.add_triangle(i, i + 1, i + 2);
                    mesh.add_triangle(i + 1, i + 2, i + 3);
                }
            });
            let last = values_tf[n_values - 1];
            let fill_color = fill_color_for_point(last);
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
