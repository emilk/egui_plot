use std::ops::RangeInclusive;

use egui::Color32;
use egui::CornerRadius;
use egui::Id;
use egui::Shape;
use egui::Stroke;
use egui::Ui;
use egui::epaint::RectShape;
use emath::Float as _;
use emath::NumExt as _;
use emath::Pos2;

use crate::aesthetics::Orientation;
use crate::axis::PlotTransform;
use crate::bounds::PlotBounds;
use crate::bounds::PlotPoint;
use crate::colors::highlighted_color;
use crate::cursor::Cursor;
use crate::items::ClosestElem;
use crate::items::PlotConfig;
use crate::items::PlotGeometry;
use crate::items::PlotItem;
use crate::items::PlotItemBase;
use crate::items::add_rulers_and_text;
use crate::label::LabelFormatter;
use crate::math::find_closest_rect;
use crate::rect_elem::RectElement;

/// A bar chart.
pub struct BarChart {
    base: PlotItemBase,

    pub(crate) bars: Vec<Bar>,
    default_color: Color32,

    /// A custom element formatter
    pub(crate) element_formatter: Option<Box<dyn Fn(&Bar, &BarChart) -> String>>,
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

    /// Set the default color. It is set on all elements that do not already
    /// have a specific color. This is the color that shows up in the
    /// legend. It can be overridden at the bar level (see [[`Bar`]]).
    /// Default is `Color32::TRANSPARENT` which means a color will be
    /// auto-assigned.
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

    /// Name of this plot item.
    ///
    /// This name will show up in the plot legend, if legends are turned on.
    ///
    /// Setting the name via this method does not change the item's id, so you
    /// can use it to change the name dynamically between frames without
    /// losing the item's state. You should make sure the name passed to
    /// [`Self::new`] is unique and stable for each item, or set unique and
    /// stable ids explicitly via [`Self::id`].
    #[expect(clippy::needless_pass_by_value, reason = "to allow various string types")]
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
        _: &Option<LabelFormatter<'_>>,
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

/// One bar in a [`BarChart`]. Potentially floating, allowing stacked bar
/// charts. Width can be changed to allow variable-width histograms.
#[derive(Clone, Debug, PartialEq)]
pub struct Bar {
    /// Name of plot element in the diagram (annotated by default formatter)
    pub name: String,

    /// Which direction the bar faces in the diagram
    pub orientation: Orientation,

    /// Position on the argument (input) axis -- X if vertical, Y if horizontal
    pub argument: f64,

    /// Position on the value (output) axis -- Y if vertical, X if horizontal
    pub value: f64,

    /// For stacked bars, this denotes where the bar starts. None if base axis
    pub base_offset: Option<f64>,

    /// Thickness of the bar
    pub bar_width: f64,

    /// Line width and color
    pub stroke: Stroke,

    /// Fill color
    pub fill: Color32,
}

impl Bar {
    /// Create a bar. Its `orientation` is set by its [`BarChart`] parent.
    ///
    /// - `argument`: Position on the argument axis (X if vertical, Y if
    ///   horizontal).
    /// - `value`: Height of the bar (if vertical).
    ///
    /// By default the bar is vertical and its base is at zero.
    pub fn new(argument: f64, height: f64) -> Self {
        Self {
            argument,
            value: height,
            orientation: Orientation::default(),
            name: Default::default(),
            base_offset: None,
            bar_width: 0.5,
            stroke: Stroke::new(1.0, Color32::TRANSPARENT),
            fill: Color32::TRANSPARENT,
        }
    }

    /// Name of this bar chart element.
    #[expect(clippy::needless_pass_by_value, reason = "to allow various string types")]
    #[inline]
    pub fn name(mut self, name: impl ToString) -> Self {
        self.name = name.to_string();
        self
    }

    /// Add a custom stroke.
    #[inline]
    pub fn stroke(mut self, stroke: impl Into<Stroke>) -> Self {
        self.stroke = stroke.into();
        self
    }

    /// Add a custom fill color.
    #[inline]
    pub fn fill(mut self, color: impl Into<Color32>) -> Self {
        self.fill = color.into();
        self
    }

    /// Offset the base of the bar.
    /// This offset is on the Y axis for a vertical bar
    /// and on the X axis for a horizontal bar.
    #[inline]
    pub fn base_offset(mut self, offset: f64) -> Self {
        self.base_offset = Some(offset);
        self
    }

    /// Set the bar width.
    #[inline]
    pub fn width(mut self, width: f64) -> Self {
        self.bar_width = width;
        self
    }

    /// Set orientation of the element as vertical. Argument axis is X.
    #[inline]
    pub fn vertical(mut self) -> Self {
        self.orientation = Orientation::Vertical;
        self
    }

    /// Set orientation of the element as horizontal. Argument axis is Y.
    #[inline]
    pub fn horizontal(mut self) -> Self {
        self.orientation = Orientation::Horizontal;
        self
    }

    pub(in crate::items) fn lower(&self) -> f64 {
        if self.value.is_sign_positive() {
            self.base_offset.unwrap_or(0.0)
        } else {
            self.base_offset.map_or(self.value, |o| o + self.value)
        }
    }

    pub(in crate::items) fn upper(&self) -> f64 {
        if self.value.is_sign_positive() {
            self.base_offset.map_or(self.value, |o| o + self.value)
        } else {
            self.base_offset.unwrap_or(0.0)
        }
    }

    pub(in crate::items) fn add_shapes(&self, transform: &PlotTransform, highlighted: bool, shapes: &mut Vec<Shape>) {
        let (stroke, fill) = if highlighted {
            highlighted_color(self.stroke, self.fill)
        } else {
            (self.stroke, self.fill)
        };

        let rect = transform.rect_from_values(&self.bounds_min(), &self.bounds_max());
        let rect = Shape::Rect(RectShape::new(
            rect,
            CornerRadius::ZERO,
            fill,
            stroke,
            egui::StrokeKind::Inside,
        ));

        shapes.push(rect);
    }

    pub(in crate::items) fn add_rulers_and_text(
        &self,
        parent: &BarChart,
        plot: &PlotConfig<'_>,
        shapes: &mut Vec<Shape>,
        cursors: &mut Vec<Cursor>,
    ) {
        let text: Option<String> = parent.element_formatter.as_ref().map(|fmt| fmt(self, parent));

        add_rulers_and_text(self, plot, text, shapes, cursors);
    }
}

impl RectElement for Bar {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn bounds_min(&self) -> PlotPoint {
        self.point_at(self.argument - self.bar_width / 2.0, self.lower())
    }

    fn bounds_max(&self) -> PlotPoint {
        self.point_at(self.argument + self.bar_width / 2.0, self.upper())
    }

    fn values_with_ruler(&self) -> Vec<PlotPoint> {
        let base = self.base_offset.unwrap_or(0.0);
        let value_center = self.point_at(self.argument, base + self.value);

        let mut ruler_positions = vec![value_center];

        if let Some(offset) = self.base_offset {
            ruler_positions.push(self.point_at(self.argument, offset));
        }

        ruler_positions
    }

    fn orientation(&self) -> Orientation {
        self.orientation
    }

    fn default_values_format(&self, transform: &PlotTransform) -> String {
        let scale = transform.dvalue_dpos();
        let scale = match self.orientation {
            Orientation::Horizontal => scale[0],
            Orientation::Vertical => scale[1],
        };
        let decimals = ((-scale.abs().log10()).ceil().at_least(0.0) as usize).at_most(6);
        crate::label::format_number(self.value, decimals)
    }
}
