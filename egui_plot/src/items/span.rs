use std::f32::consts::PI;
use std::ops::RangeInclusive;

use egui::Align2;
use egui::Color32;
use egui::Pos2;
use egui::Rect;
use egui::Shape;
use egui::Stroke;
use egui::TextStyle;
use egui::Ui;
use egui::Vec2;
use egui::epaint::PathStroke;
use egui::epaint::TextShape;
use egui::pos2;
use emath::TSTransform;

use super::LineStyle;
use super::PlotBounds;
use super::PlotGeometry;
use super::PlotItem;
use super::PlotItemBase;
use super::PlotPoint;
use super::PlotTransform;
use super::rect_elem::highlighted_color;
use crate::Axis;
use crate::utils::find_name_candidate;

/// Padding between the label of the span and both the edge of the view and the
/// span borders. For example, for a horizontal span, this is the padding
/// between the top of the span label and the top edge of the plot view, but
/// also the margin between the left/right edges of the span and the span label.
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
    label_align: Align2,
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
            label_align: Align2::CENTER_TOP,
        }
    }

    /// Select which axis the span applies to. This also sets the label
    /// alignment. If you want a different label alignment, you need to set
    /// it by calling `label_align` after this call.
    #[inline]
    pub fn axis(mut self, axis: Axis) -> Self {
        self.axis = axis;
        match axis {
            Axis::X => self.label_align = Align2::CENTER_TOP,
            Axis::Y => self.label_align = Align2::LEFT_CENTER,
        }
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

    /// Set the label alignment within the span.
    /// This should be called after any calls to `axis` as that would overwrite
    /// the label alignment
    #[inline]
    pub fn label_align(mut self, align: Align2) -> Self {
        self.label_align = align;
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
        if start <= end { (start, end) } else { (end, start) }
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

    fn draw_border(&self, value: f64, stroke: Stroke, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        if stroke.color == Color32::TRANSPARENT || stroke.width <= 0.0 || !value.is_finite() {
            return;
        }

        let line = match self.axis {
            Axis::X => Self::vline_points(value, transform),
            Axis::Y => Self::hline_points(value, transform),
        };

        self.border_style
            .style_line(line, PathStroke::new(stroke.width, stroke.color), false, shapes);
    }

    fn available_width_for_name(&self, rect: &Rect) -> f32 {
        match self.axis {
            Axis::X => (rect.width() - 2.0 * LABEL_PADDING).max(0.0),
            Axis::Y => (rect.height() - 2.0 * LABEL_PADDING).max(0.0),
        }
    }

    fn draw_name(&self, ui: &Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>, span_rect: &Rect) {
        let frame = *transform.frame();
        let visible_rect = span_rect.intersect(frame);

        let available_width = self.available_width_for_name(&visible_rect);
        if available_width <= 0.0 {
            return;
        }

        let font_id = TextStyle::Body.resolve(ui.style());
        let text_color = ui.visuals().text_color();
        let painter = ui.painter();

        let name = find_name_candidate(&self.base.name, available_width, painter, &font_id);

        let galley = painter.layout_no_wrap(name, font_id, text_color);

        if galley.is_empty() {
            return;
        }

        // Place text center point at origin and rotate for Y-axis.
        let mut text_shape = match self.axis {
            Axis::X => TextShape::new(pos2(-galley.size().x / 2.0, -galley.size().y / 2.0), galley, text_color),

            // For spans on the Y axis we rotate the text by 90° around its center point
            Axis::Y => TextShape::new(pos2(-galley.size().x / 2.0, -galley.size().y / 2.0), galley, text_color)
                .with_angle_and_anchor(-PI / 2.0, Align2::CENTER_CENTER),
        };

        // Take into account the rotation of the text when calculating its position
        let text_rect = text_shape.visual_bounding_rect();
        let (width, height) = (text_rect.width(), text_rect.height());

        // Calculate the position of the text based on the label alignment
        let text_pos_x = match self.label_align {
            Align2::LEFT_CENTER | Align2::LEFT_TOP | Align2::LEFT_BOTTOM => visible_rect.left() + LABEL_PADDING,
            Align2::CENTER_CENTER | Align2::CENTER_TOP | Align2::CENTER_BOTTOM => visible_rect.center().x - width / 2.0,
            Align2::RIGHT_CENTER | Align2::RIGHT_TOP | Align2::RIGHT_BOTTOM => {
                visible_rect.right() - LABEL_PADDING - width
            }
        };

        let text_pos_y = match self.label_align {
            Align2::LEFT_TOP | Align2::CENTER_TOP | Align2::RIGHT_TOP => visible_rect.top() + LABEL_PADDING,
            Align2::LEFT_CENTER | Align2::CENTER_CENTER | Align2::RIGHT_CENTER => {
                visible_rect.center().y - height / 2.0
            }
            Align2::LEFT_BOTTOM | Align2::CENTER_BOTTOM | Align2::RIGHT_BOTTOM => {
                visible_rect.bottom() - LABEL_PADDING - height
            }
        };

        // Make sure to add half the width/height since the text position is at the
        // center of the text shape
        let text_pos = pos2(text_pos_x + width / 2.0, text_pos_y + height / 2.0);

        text_shape.transform(TSTransform::from_translation(Vec2::new(text_pos.x, text_pos.y)));

        shapes.push(text_shape.into());
    }
}

impl PlotItem for Span {
    fn shapes(&self, ui: &Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        let plot_bounds = match self.axis {
            Axis::X => transform.bounds().range_x(),
            Axis::Y => transform.bounds().range_y(),
        };

        let (range_min, range_max) = self.range_sorted();

        // If the span is outside of the visible range, don't draw anything.
        if range_max < *plot_bounds.start() || range_min > *plot_bounds.end() {
            return;
        }

        let mut stroke = self.border_stroke;
        let mut fill = self.fill;
        if self.base.highlight {
            (stroke, fill) = highlighted_color(stroke, fill);
        }

        // Clamp the range to support (half-)infinite spans
        let range_min_clamped = range_min.max(*plot_bounds.start());
        let range_max_clamped = range_max.min(*plot_bounds.end());

        // Draw the rect first with the clamped range
        let span_rect = match self.axis {
            Axis::X => transform.rect_from_values(
                &PlotPoint::new(range_min_clamped, transform.bounds().min[1]),
                &PlotPoint::new(range_max_clamped, transform.bounds().max[1]),
            ),
            Axis::Y => transform.rect_from_values(
                &PlotPoint::new(transform.bounds().min[0], range_min_clamped),
                &PlotPoint::new(transform.bounds().max[0], range_max_clamped),
            ),
        };

        if fill != Color32::TRANSPARENT && span_rect.is_positive() {
            shapes.push(Shape::rect_filled(span_rect, 0.0, fill));
        }

        // Draw the first border if it is in bounds
        if plot_bounds.contains(&range_min) {
            self.draw_border(range_min, stroke, transform, shapes);
        }

        // Draw the second border if it is in bounds
        if plot_bounds.contains(&range_max) {
            self.draw_border(range_max, stroke, transform, shapes);
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
        PlotBounds::NOTHING
    }

    fn base(&self) -> &PlotItemBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut PlotItemBase {
        &mut self.base
    }
}
