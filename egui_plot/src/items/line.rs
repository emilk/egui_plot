use crate::Id;
use crate::builder_methods_for_base;
use crate::{
    LineStyle, PlotBounds, PlotGeometry, PlotItem, PlotItemBase, PlotPoint, PlotTransform,
};
use egui::epaint::PathStroke;
use egui::{Color32, Shape, Stroke, Ui};
use std::ops::RangeInclusive;

/// A horizontal line in a plot, filling the full width
#[derive(Clone, Debug, PartialEq)]
pub struct HLine {
    base: PlotItemBase,
    pub(crate) y: f64,
    pub(crate) stroke: Stroke,
    pub(crate) style: LineStyle,
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
    pub(crate) x: f64,
    pub(crate) stroke: Stroke,
    pub(crate) style: LineStyle,
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
