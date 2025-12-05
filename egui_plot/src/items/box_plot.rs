use std::ops::RangeInclusive;

use egui::Color32;
use egui::CornerRadius;
use egui::Id;
use egui::Shape;
use egui::Stroke;
use egui::Ui;
use egui::epaint::RectShape;
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

/// A diagram containing a series of [`BoxElem`] elements.
pub struct BoxPlot {
    base: PlotItemBase,

    pub(crate) boxes: Vec<BoxElem>,
    default_color: Color32,

    /// A custom element formatter
    pub(crate) element_formatter: Option<Box<dyn Fn(&BoxElem, &BoxPlot) -> String>>,
}

impl BoxPlot {
    /// Create a plot containing multiple `boxes`. It defaults to vertically
    /// oriented elements.
    pub fn new(name: impl Into<String>, boxes: Vec<BoxElem>) -> Self {
        Self {
            base: PlotItemBase::new(name.into()),
            boxes,
            default_color: Color32::TRANSPARENT,
            element_formatter: None,
        }
    }

    /// Set the default color. It is set on all elements that do not already
    /// have a specific color. This is the color that shows up in the
    /// legend. It can be overridden at the element level (see [`BoxElem`]).
    /// Default is `Color32::TRANSPARENT` which means a color will be
    /// auto-assigned.
    #[inline]
    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        let plot_color = color.into();
        self.default_color = plot_color;
        for box_elem in &mut self.boxes {
            if box_elem.fill == Color32::TRANSPARENT && box_elem.stroke.color == Color32::TRANSPARENT {
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

/// Contains the values of a single box in a box plot.
#[derive(Clone, Debug, PartialEq)]
pub struct BoxSpread {
    /// Value of lower whisker (typically minimum).
    ///
    /// The whisker is not drawn if `lower_whisker >= quartile1`.
    pub lower_whisker: f64,

    /// Value of lower box threshold (typically 25% quartile)
    pub quartile1: f64,

    /// Value of middle line in box (typically median)
    pub median: f64,

    /// Value of upper box threshold (typically 75% quartile)
    pub quartile3: f64,

    /// Value of upper whisker (typically maximum)
    ///
    /// The whisker is not drawn if `upper_whisker <= quartile3`.
    pub upper_whisker: f64,
}

impl BoxSpread {
    pub fn new(lower_whisker: f64, quartile1: f64, median: f64, quartile3: f64, upper_whisker: f64) -> Self {
        Self {
            lower_whisker,
            quartile1,
            median,
            quartile3,
            upper_whisker,
        }
    }
}

/// A box in a [`BoxPlot`] diagram.
///
/// This is a low-level graphical element; it will not compute quartiles and
/// whiskers, letting one use their preferred formula. Use
/// [`Points`][`super::Points`] to draw the outliers.
#[derive(Clone, Debug, PartialEq)]
pub struct BoxElem {
    /// Name of plot element in the diagram (annotated by default formatter).
    pub name: String,

    /// Which direction the box faces in the diagram.
    pub orientation: Orientation,

    /// Position on the argument (input) axis -- X if vertical, Y if horizontal.
    pub argument: f64,

    /// Values of the box
    pub spread: BoxSpread,

    /// Thickness of the box
    pub box_width: f64,

    /// Width of the whisker at minimum/maximum
    pub whisker_width: f64,

    /// Line width and color
    pub stroke: Stroke,

    /// Fill color
    pub fill: Color32,
}

impl BoxElem {
    /// Create a box element. Its `orientation` is set by its [`BoxPlot`]
    /// parent.
    ///
    /// Check [`BoxElem`] fields for detailed description.
    pub fn new(argument: f64, spread: BoxSpread) -> Self {
        Self {
            argument,
            orientation: Orientation::default(),
            name: String::default(),
            spread,
            box_width: 0.25,
            whisker_width: 0.15,
            stroke: Stroke::new(1.0, Color32::TRANSPARENT),
            fill: Color32::TRANSPARENT,
        }
    }

    /// Name of this box element.
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

    /// Set the box width.
    #[inline]
    pub fn box_width(mut self, width: f64) -> Self {
        self.box_width = width;
        self
    }

    /// Set the whisker width.
    #[inline]
    pub fn whisker_width(mut self, width: f64) -> Self {
        self.whisker_width = width;
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

    pub(in crate::items) fn add_shapes(&self, transform: &PlotTransform, highlighted: bool, shapes: &mut Vec<Shape>) {
        let (stroke, fill) = if highlighted {
            highlighted_color(self.stroke, self.fill)
        } else {
            (self.stroke, self.fill)
        };

        let rect = transform.rect_from_values(
            &self.point_at(self.argument - self.box_width / 2.0, self.spread.quartile1),
            &self.point_at(self.argument + self.box_width / 2.0, self.spread.quartile3),
        );
        let rect = Shape::Rect(RectShape::new(
            rect,
            CornerRadius::ZERO,
            fill,
            stroke,
            egui::StrokeKind::Inside,
        ));
        shapes.push(rect);

        let line_between = |v1, v2| {
            Shape::line_segment(
                [transform.position_from_point(&v1), transform.position_from_point(&v2)],
                stroke,
            )
        };
        let median = line_between(
            self.point_at(self.argument - self.box_width / 2.0, self.spread.median),
            self.point_at(self.argument + self.box_width / 2.0, self.spread.median),
        );
        shapes.push(median);

        if self.spread.upper_whisker > self.spread.quartile3 {
            let high_whisker = line_between(
                self.point_at(self.argument, self.spread.quartile3),
                self.point_at(self.argument, self.spread.upper_whisker),
            );
            shapes.push(high_whisker);
            if self.box_width > 0.0 {
                let high_whisker_end = line_between(
                    self.point_at(self.argument - self.whisker_width / 2.0, self.spread.upper_whisker),
                    self.point_at(self.argument + self.whisker_width / 2.0, self.spread.upper_whisker),
                );
                shapes.push(high_whisker_end);
            }
        }

        if self.spread.lower_whisker < self.spread.quartile1 {
            let low_whisker = line_between(
                self.point_at(self.argument, self.spread.quartile1),
                self.point_at(self.argument, self.spread.lower_whisker),
            );
            shapes.push(low_whisker);
            if self.box_width > 0.0 {
                let low_whisker_end = line_between(
                    self.point_at(self.argument - self.whisker_width / 2.0, self.spread.lower_whisker),
                    self.point_at(self.argument + self.whisker_width / 2.0, self.spread.lower_whisker),
                );
                shapes.push(low_whisker_end);
            }
        }
    }

    pub(in crate::items) fn add_rulers_and_text(
        &self,
        parent: &BoxPlot,
        plot: &PlotConfig<'_>,
        shapes: &mut Vec<Shape>,
        cursors: &mut Vec<Cursor>,
    ) {
        let text: Option<String> = parent.element_formatter.as_ref().map(|fmt| fmt(self, parent));

        add_rulers_and_text(self, plot, text, shapes, cursors);
    }
}

impl RectElement for BoxElem {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn bounds_min(&self) -> PlotPoint {
        let argument = self.argument - self.box_width.max(self.whisker_width) / 2.0;
        let value = self.spread.lower_whisker;
        self.point_at(argument, value)
    }

    fn bounds_max(&self) -> PlotPoint {
        let argument = self.argument + self.box_width.max(self.whisker_width) / 2.0;
        let value = self.spread.upper_whisker;
        self.point_at(argument, value)
    }

    fn values_with_ruler(&self) -> Vec<PlotPoint> {
        let median = self.point_at(self.argument, self.spread.median);
        let q1 = self.point_at(self.argument, self.spread.quartile1);
        let q3 = self.point_at(self.argument, self.spread.quartile3);
        let upper = self.point_at(self.argument, self.spread.upper_whisker);
        let lower = self.point_at(self.argument, self.spread.lower_whisker);

        vec![median, q1, q3, upper, lower]
    }

    fn orientation(&self) -> Orientation {
        self.orientation
    }

    fn corner_value(&self) -> PlotPoint {
        self.point_at(self.argument, self.spread.upper_whisker)
    }

    fn default_values_format(&self, transform: &PlotTransform) -> String {
        let scale = transform.dvalue_dpos();
        let scale = match self.orientation {
            Orientation::Horizontal => scale[0],
            Orientation::Vertical => scale[1],
        };
        let y_decimals = ((-scale.abs().log10()).ceil().at_least(0.0) as usize)
            .at_most(6)
            .at_least(1);
        format!(
            "Max = {max:.decimals$}\
             \nQuartile 3 = {q3:.decimals$}\
             \nMedian = {med:.decimals$}\
             \nQuartile 1 = {q1:.decimals$}\
             \nMin = {min:.decimals$}",
            max = self.spread.upper_whisker,
            q3 = self.spread.quartile3,
            med = self.spread.median,
            q1 = self.spread.quartile1,
            min = self.spread.lower_whisker,
            decimals = y_decimals
        )
    }
}
