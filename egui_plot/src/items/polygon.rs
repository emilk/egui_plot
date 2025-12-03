use std::ops::RangeInclusive;

use egui::Color32;
use egui::Id;
use egui::Shape;
use egui::Stroke;
use egui::Ui;
use egui::epaint::PathStroke;

use crate::PlotItem;
use crate::PlotItemBase;
use crate::PlotTransform;
use crate::aesthetics::LineStyle;
use crate::bounds::PlotBounds;
use crate::colors::DEFAULT_FILL_ALPHA;
use crate::data::PlotPoints;
use crate::items::PlotGeometry;

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
