use crate::Axis;
use std::{f32::consts::PI, ops::RangeInclusive};

use egui::{
    Color32, FontId, Painter, Pos2, Rect, Shape, Stroke, TextStyle, Ui,
    epaint::{PathStroke, TextShape},
    pos2,
};

use super::{
    LineStyle, PlotBounds, PlotGeometry, PlotItem, PlotItemBase, PlotPoint, PlotTransform,
    rect_elem::highlighted_color,
};

const LABEL_PADDING: f32 = 4.0;

/// A span covering a range on either axis.
#[derive(Clone, Debug, PartialEq)]
pub struct Span {
    base: PlotItemBase,
    axis: Axis,
    range: RangeInclusive<f64>,
    fill: Color32,
    border_stroke: Stroke,
    border_style: LineStyle,
}

impl Span {
    /// Create a new span covering the provided range on the X axis by default.
    pub fn new(name: impl Into<String>, range: impl Into<RangeInclusive<f64>>) -> Self {
        Self {
            base: PlotItemBase::new(name.into()),
            axis: Axis::X,
            range: range.into(),
            fill: Color32::TRANSPARENT,
            border_stroke: Stroke::new(1.0, Color32::TRANSPARENT),
            border_style: LineStyle::Solid,
        }
    }

    /// Select which axis the span applies to.
    #[inline]
    pub fn axis(mut self, axis: Axis) -> Self {
        self.axis = axis;
        self
    }

    /// Set the range.
    #[inline]
    pub fn range(mut self, range: impl Into<RangeInclusive<f64>>) -> Self {
        self.range = range.into();
        self
    }

    /// Set the background fill color for the span.
    #[inline]
    pub fn fill(mut self, color: impl Into<Color32>) -> Self {
        self.fill = color.into();
        self
    }

    /// Set the stroke used for both span borders.
    #[inline]
    pub fn border(mut self, stroke: impl Into<Stroke>) -> Self {
        self.border_stroke = stroke.into();
        self
    }

    /// Convenience for updating the span border width.
    #[inline]
    pub fn border_width(mut self, width: impl Into<f32>) -> Self {
        self.border_stroke.width = width.into();
        self
    }

    /// Convenience for updating the span border color.
    #[inline]
    pub fn border_color(mut self, color: impl Into<Color32>) -> Self {
        self.border_stroke.color = color.into();
        self
    }

    /// Set the style for the span borders. Defaults to `LineStyle::Solid`.
    #[inline]
    pub fn border_style(mut self, style: LineStyle) -> Self {
        self.border_style = style;
        self
    }

    #[inline]
    pub(crate) fn fill_color(&self) -> Color32 {
        self.fill
    }

    #[inline]
    pub(crate) fn border_color_value(&self) -> Color32 {
        self.border_stroke.color
    }

    fn range_sorted(&self) -> (f64, f64) {
        let start = *self.range.start();
        let end = *self.range.end();
        if start <= end {
            (start, end)
        } else {
            (end, start)
        }
    }

    fn hline_points(value: f64, transform: &PlotTransform) -> Vec<Pos2> {
        vec![
            transform.position_from_point(&PlotPoint::new(transform.bounds().min[0], value)),
            transform.position_from_point(&PlotPoint::new(transform.bounds().max[0], value)),
        ]
    }

    fn vline_points(value: f64, transform: &PlotTransform) -> Vec<Pos2> {
        vec![
            transform.position_from_point(&PlotPoint::new(value, transform.bounds().min[1])),
            transform.position_from_point(&PlotPoint::new(value, transform.bounds().max[1])),
        ]
    }

    fn draw_border(
        &self,
        value: f64,
        stroke: Stroke,
        transform: &PlotTransform,
        shapes: &mut Vec<Shape>,
    ) {
        if stroke.color == Color32::TRANSPARENT || stroke.width <= 0.0 {
            return;
        }

        let line = match self.axis {
            Axis::X => Self::vline_points(value, transform),
            Axis::Y => Self::hline_points(value, transform),
        };

        self.border_style.style_line(
            line,
            PathStroke::new(stroke.width, stroke.color),
            false,
            shapes,
        );
    }

    fn available_width_for_name(&self, rect: &Rect) -> f32 {
        match self.axis {
            Axis::X => (rect.width() - 2.0 * LABEL_PADDING).max(0.0),
            Axis::Y => (rect.height() - 2.0 * LABEL_PADDING).max(0.0),
        }
    }

    // If the span is too small to display the full name, find the longest name
    // with "..." appended that we can display within the span
    fn find_name_candidate(&self, width: f32, painter: &Painter, font_id: &FontId) -> String {
        let name = self.base.name.as_str();
        let galley = painter.layout_no_wrap(name.to_owned(), font_id.clone(), Color32::BLACK);

        if galley.size().x <= width || name.is_empty() {
            return name.to_owned();
        }

        // If we don't have enough space for the name to be displayed in the span, we search
        // for the longest candidate that fits, where a candidate is a truncated version of the
        // name followed by "...".
        let chars: Vec<char> = name.chars().collect();

        // First test the minimum candidate which is the first letter followed by "..."
        let mut min_candidate = chars[0].to_string();
        min_candidate.push_str("...");
        let galley = painter.layout_no_wrap(min_candidate.clone(), font_id.clone(), Color32::BLACK);
        if galley.size().x > width {
            return String::new();
        }

        // Then do a binary search to find the longest possible candidate
        let mut low = 1;
        let mut high = chars.len();
        let mut best = String::new();

        while low <= high {
            let mid = usize::midpoint(low, high);
            let mut candidate: String = chars[..mid].iter().collect();
            candidate.push_str("...");

            let candidate_width = painter
                .layout_no_wrap(candidate.clone(), font_id.clone(), Color32::BLACK)
                .size()
                .x;

            if candidate_width <= width {
                best = candidate;
                low = mid + 1;
            } else {
                high = mid.saturating_sub(1);
                if high == 0 {
                    break;
                }
            }
        }

        best
    }

    fn draw_name(
        &self,
        ui: &Ui,
        transform: &PlotTransform,
        shapes: &mut Vec<Shape>,
        span_rect: &Rect,
    ) {
        let frame = *transform.frame();
        let visible_rect = span_rect.intersect(frame);

        let available_width = self.available_width_for_name(&visible_rect);
        if available_width <= 0.0 {
            return;
        }

        let font_id = TextStyle::Body.resolve(ui.style());
        let text_color = ui.visuals().text_color();
        let painter = ui.painter();

        let name = self.find_name_candidate(available_width, painter, &font_id);

        let galley = painter.layout_no_wrap(name, font_id, text_color);

        if galley.is_empty() {
            return;
        }

        let text_pos = match self.axis {
            Axis::X => pos2(
                visible_rect.center().x - galley.size().x / 2.0,
                visible_rect.top() + LABEL_PADDING,
            ),
            Axis::Y => pos2(
                visible_rect.left() + LABEL_PADDING,
                visible_rect.center().y + galley.size().x / 2.0,
            ),
        };

        let text_shape = match self.axis {
            Axis::X => TextShape::new(text_pos, galley, text_color),

            // For spans on the Y axis we rotate the text by 90° around its center point
            Axis::Y => TextShape::new(text_pos, galley, text_color).with_angle(-PI / 2.0),
        };

        shapes.push(text_shape.into());
    }
}

impl PlotItem for Span {
    fn shapes(&self, ui: &Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        let (range_min, range_max) = self.range_sorted();

        if !range_min.is_finite() || !range_max.is_finite() {
            return;
        }

        let mut stroke = self.border_stroke;
        let mut fill = self.fill;
        if self.base.highlight {
            (stroke, fill) = highlighted_color(stroke, fill);
        }

        let span_rect = match self.axis {
            Axis::X => transform.rect_from_values(
                &PlotPoint::new(range_min, transform.bounds().min[1]),
                &PlotPoint::new(range_max, transform.bounds().max[1]),
            ),
            Axis::Y => transform.rect_from_values(
                &PlotPoint::new(transform.bounds().min[0], range_min),
                &PlotPoint::new(transform.bounds().max[0], range_max),
            ),
        };

        if fill != Color32::TRANSPARENT && span_rect.is_positive() {
            shapes.push(Shape::rect_filled(span_rect, 0.0, fill));
        }

        let mut border_values = vec![range_min, range_max];
        if (range_max - range_min).abs() <= f64::EPSILON {
            border_values.truncate(1);
        }

        for value in border_values {
            self.draw_border(value, stroke, transform, shapes);
        }

        self.draw_name(ui, transform, shapes, &span_rect);
    }

    fn initialize(&mut self, _x_range: RangeInclusive<f64>) {}

    fn color(&self) -> Color32 {
        if self.fill != Color32::TRANSPARENT {
            self.fill
        } else {
            self.border_stroke.color
        }
    }

    fn geometry(&self) -> PlotGeometry<'_> {
        PlotGeometry::None
    }

    fn bounds(&self) -> PlotBounds {
        let mut bounds = PlotBounds::NOTHING;
        let (min, max) = self.range_sorted();

        match self.axis {
            Axis::X => {
                bounds.extend_with_x(min);
                bounds.extend_with_x(max);
            }
            Axis::Y => {
                bounds.extend_with_y(min);
                bounds.extend_with_y(max);
            }
        }

        bounds
    }

    fn base(&self) -> &PlotItemBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut PlotItemBase {
        &mut self.base
    }
}
