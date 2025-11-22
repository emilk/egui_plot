use std::ops::RangeInclusive;

use egui::Color32;
use egui::Shape;
use egui::Stroke;
use egui::Ui;
use egui::epaint::CircleShape;
use emath::Pos2;
use emath::pos2;
use emath::vec2;

use crate::Id;
use crate::MarkerShape;
use crate::PlotBounds;
use crate::PlotGeometry;
use crate::PlotItem;
use crate::PlotItemBase;
use crate::PlotPoint;
use crate::PlotPoints;
use crate::PlotTransform;
use crate::builder_methods_for_base;

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

    /// Whether to add stems between the markers and a horizontal reference
    /// line.
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

    /// Name of this plot item.
    ///
    /// This name will show up in the plot legend, if legends are turned on.
    ///
    /// Setting the name via this method does not change the item's id, so you can use it to
    /// change the name dynamically between frames without losing the item's state. You should
    /// make sure the name passed to [`Self::new`] is unique and stable for each item, or
    /// set unique and stable ids explicitly via [`Self::id`].
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
    /// By default the id is determined from the name passed to [`Self::new`], but it can be
    /// explicitly set to a different value.
    #[inline]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.base_mut().id = id.into();
        self
    }
}

/// A set of points.
pub struct Points<'a> {
    base: PlotItemBase,

    pub(crate) series: PlotPoints<'a>,

    pub(crate) shape: MarkerShape,

    /// Color of the marker. `Color32::TRANSPARENT` means that it will be picked
    /// automatically.
    pub(crate) color: Color32,

    /// Whether to fill the marker. Does not apply to all types.
    pub(crate) filled: bool,

    /// The maximum extent of the marker from its center.
    pub(crate) radius: f32,

    pub(crate) stems: Option<f32>,
}

impl PlotItem for Points<'_> {
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
                        let diagonal1 = [tf(-frac_1_sqrt_2, -frac_1_sqrt_2), tf(frac_1_sqrt_2, frac_1_sqrt_2)];
                        let diagonal2 = [tf(frac_1_sqrt_2, -frac_1_sqrt_2), tf(-frac_1_sqrt_2, frac_1_sqrt_2)];
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
                        let points = vec![tf(0.0, -1.0), tf(0.5 * sqrt_3, 0.5), tf(-0.5 * sqrt_3, 0.5)];
                        shapes.push(Shape::convex_polygon(points, fill, stroke));
                    }
                    MarkerShape::Down => {
                        let points = vec![tf(0.0, 1.0), tf(-0.5 * sqrt_3, -0.5), tf(0.5 * sqrt_3, -0.5)];
                        shapes.push(Shape::convex_polygon(points, fill, stroke));
                    }
                    MarkerShape::Left => {
                        let points = vec![tf(-1.0, 0.0), tf(0.5, -0.5 * sqrt_3), tf(0.5, 0.5 * sqrt_3)];
                        shapes.push(Shape::convex_polygon(points, fill, stroke));
                    }
                    MarkerShape::Right => {
                        let points = vec![tf(1.0, 0.0), tf(-0.5, 0.5 * sqrt_3), tf(-0.5, -0.5 * sqrt_3)];
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
