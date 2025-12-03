use std::ops::RangeInclusive;

use egui::Color32;
use egui::Shape;
use egui::Stroke;
use egui::Ui;
use egui::epaint::PathStroke;
use emath::Pos2;
use emath::pos2;

use crate::Id;
use crate::PlotItem;
use crate::PlotItemBase;
use crate::PlotTransform;
use crate::aesthetics::LineStyle;
use crate::bounds::PlotBounds;
use crate::data::PlotPoint;
use crate::items::PlotGeometry;

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

    /// Stroke color. Default is `Color32::TRANSPARENT` which means a color will
    /// be auto-assigned.
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

impl PlotItem for HLine {
    fn shapes(&self, _ui: &Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        let Self {
            base, y, stroke, style, ..
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

    /// Stroke color. Default is `Color32::TRANSPARENT` which means a color will
    /// be auto-assigned.
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

impl PlotItem for VLine {
    fn shapes(&self, _ui: &Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        let Self {
            base, x, stroke, style, ..
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

pub fn vertical_line(pointer: Pos2, transform: &PlotTransform, line_color: Color32) -> Shape {
    let frame = transform.frame();
    Shape::line_segment(
        [pos2(pointer.x, frame.top()), pos2(pointer.x, frame.bottom())],
        (1.0, line_color),
    )
}

pub fn horizontal_line(pointer: Pos2, transform: &PlotTransform, line_color: Color32) -> Shape {
    let frame = transform.frame();
    Shape::line_segment(
        [pos2(frame.left(), pointer.y), pos2(frame.right(), pointer.y)],
        (1.0, line_color),
    )
}
