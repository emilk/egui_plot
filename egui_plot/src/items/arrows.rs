use std::ops::RangeInclusive;

use egui::Color32;
use egui::Shape;
use egui::Stroke;
use egui::Ui;
use emath::Rot2;

use super::PlotGeometry;
use crate::Id;
use super::PlotItem;
use super::PlotItemBase;
use crate::transform::PlotTransform;
use crate::bounds::PlotBounds;
use crate::data::PlotPoints;

impl<'a> Arrows<'a> {
    pub fn new(name: impl Into<String>, origins: impl Into<PlotPoints<'a>>, tips: impl Into<PlotPoints<'a>>) -> Self {
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

/// A set of arrows.
pub struct Arrows<'a> {
    base: PlotItemBase,
    pub(crate) origins: PlotPoints<'a>,
    pub(crate) tips: PlotPoints<'a>,
    pub(crate) tip_length: Option<f32>,
    pub(crate) color: Color32,
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
        self.origins.generate_points(f64::NEG_INFINITY..=f64::INFINITY);
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
