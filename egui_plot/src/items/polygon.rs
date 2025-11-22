use crate::builder_methods_for_base;
use crate::items::DEFAULT_FILL_ALPHA;
use crate::{
    LineStyle, PlotBounds, PlotGeometry, PlotItem, PlotItemBase, PlotPoints, PlotTransform,
};
use egui::epaint::PathStroke;
use egui::{Color32, Id, Shape, Stroke, Ui};
use std::ops::RangeInclusive;

/// A convex polygon.
pub struct Polygon<'a> {
    base: PlotItemBase,
    pub(crate) series: PlotPoints<'a>,
    pub(crate) stroke: Stroke,
    pub(crate) fill_color: Option<Color32>,
    pub(crate) style: LineStyle,
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
