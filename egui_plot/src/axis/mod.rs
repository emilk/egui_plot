pub mod linear;
pub mod log;
pub mod transform;

use std::fmt::Debug;
use std::ops::RangeInclusive;
use std::sync::Arc;

use egui::Color32;
use egui::FontId;
use egui::Pos2;
use egui::Rangef;
use egui::Rect;
use egui::Response;
use egui::Sense;
use egui::TextStyle;
use egui::TextWrapMode;
use egui::Ui;
use egui::Vec2;
use egui::WidgetText;
use egui::emath::Rot2;
use egui::emath::remap_clamp;
use egui::epaint::TextShape;

use crate::axis::transform::AxisSpace;
use crate::grid::GridMark;
use crate::placement::HPlacement;
use crate::placement::Placement;
use crate::placement::VPlacement;
pub use transform::PlotTransform;
use crate::axis::linear::LinearAxisSpace;
use crate::axis::log::{LogAxis, LogBase};
use crate::GridInput;

// Gap between tick labels and axis label in units of the axis label height
const AXIS_LABEL_GAP: f32 = 0.25;

pub(super) type AxisFormatterFn<'a> = dyn Fn(GridMark, &RangeInclusive<f64>) -> String + 'a;

/// X or Y axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    /// Horizontal X-Axis
    X = 0,

    /// Vertical Y-axis
    Y = 1,
}

impl From<Axis> for usize {
    #[inline]
    fn from(value: Axis) -> Self {
        match value {
            Axis::X => 0,
            Axis::Y => 1,
        }
    }
}

/// The scaling method for the axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AxisScale {
    Linear,
    Log10,
    Log2,
}

impl AxisScale {
    pub(crate) fn new_axis(&self, bounds: RangeInclusive<f64>, frame: Rangef, invert: bool) -> AxisSpaceImpl {
        match self {
            AxisScale::Linear => LinearAxisSpace::new(bounds, frame, invert).into(),
            AxisScale::Log10 => LogAxis::new(bounds, frame, invert, LogBase::Base10).into(),
            AxisScale::Log2 => LogAxis::new(bounds, frame, invert, LogBase::Base2).into(),
        }
    }
}

/// An enum style dispatch for the axis space.
#[derive(Debug, Clone, Copy)]
pub enum AxisSpaceImpl {
    Linear(LinearAxisSpace),
    Log(LogAxis),
}

impl From<LinearAxisSpace> for AxisSpaceImpl {
    fn from(value: LinearAxisSpace) -> Self {
        Self::Linear(value)
    }
}

impl From<LogAxis> for AxisSpaceImpl {
    fn from(value: LogAxis) -> Self {
        Self::Log(value)
    }
}

impl AxisSpace for AxisSpaceImpl {
    fn value_min(&self) -> f64 {
        match self {
            AxisSpaceImpl::Linear(axis) => axis.value_min(),
            AxisSpaceImpl::Log(axis) => axis.value_min(),
        }
    }

    fn value_max(&self) -> f64 {
        match self {
            AxisSpaceImpl::Linear(axis) => axis.value_max(),
            AxisSpaceImpl::Log(axis) => axis.value_max(),
        }
    }

    fn frame_min(&self) -> f32 {
        match self {
            AxisSpaceImpl::Linear(axis) => axis.frame_min(),
            AxisSpaceImpl::Log(axis) => axis.frame_min(),
        }
    }

    fn frame_max(&self) -> f32 {
        match self {
            AxisSpaceImpl::Linear(axis) => axis.frame_max(),
            AxisSpaceImpl::Log(axis) => axis.frame_max(),
        }
    }

    fn value_length(&self) -> f64 {
        match self {
            AxisSpaceImpl::Linear(axis) => axis.value_length(),
            AxisSpaceImpl::Log(axis) => axis.value_length(),
        }
    }

    fn is_valid(&self) -> bool {
        match self {
            AxisSpaceImpl::Linear(axis) => axis.is_valid(),
            AxisSpaceImpl::Log(axis) => axis.is_valid(),
        }
    }

    fn set_inverted(&mut self, invert: bool) {
        match self {
            AxisSpaceImpl::Linear(axis) => axis.set_inverted(invert),
            AxisSpaceImpl::Log(axis) => axis.set_inverted(invert),
        }
    }

    fn set_value_range(&mut self, range: RangeInclusive<f64>) {
        match self {
            AxisSpaceImpl::Linear(axis) => axis.set_value_range(range),
            AxisSpaceImpl::Log(axis) => axis.set_value_range(range),
        }
    }

    fn position_from_value(&self, value: f64) -> f32 {
        match self {
            AxisSpaceImpl::Linear(axis) => axis.position_from_value(value),
            AxisSpaceImpl::Log(axis) => axis.position_from_value(value),
        }
    }

    fn value_from_position(&self, position: f32) -> f64 {
        match self {
            AxisSpaceImpl::Linear(axis) => axis.value_from_position(position),
            AxisSpaceImpl::Log(axis) => axis.value_from_position(position),
        }
    }

    fn minimum_value_step(&self, spacing: f32) -> f64 {
        match self {
            AxisSpaceImpl::Linear(axis) => axis.minimum_value_step(spacing),
            AxisSpaceImpl::Log(axis) => axis.minimum_value_step(spacing),
        }
    }

    fn grid_input(&self, spacing: f32) -> GridInput {
        match self {
            AxisSpaceImpl::Linear(axis) => axis.grid_input(spacing),
            AxisSpaceImpl::Log(axis) => axis.grid_input(spacing),
        }
    }

    fn screen_distance_between_values(&self, value1: f64, value2: f64) -> f32 {
        match self {
            AxisSpaceImpl::Linear(axis) => axis.screen_distance_between_values(value1, value2),
            AxisSpaceImpl::Log(axis) => axis.screen_distance_between_values(value1, value2),
        }
    }

    fn translate(&mut self, frame_distance: f32) {
        match self {
            AxisSpaceImpl::Linear(axis) => axis.translate(frame_distance),
            AxisSpaceImpl::Log(axis) => axis.translate(frame_distance),
        }
    }

    fn zoom(&mut self, factor: f32, center: f64) {
        match self {
            AxisSpaceImpl::Linear(axis) => axis.zoom(factor, center),
            AxisSpaceImpl::Log(axis) => axis.zoom(factor, center),
        }
    }

    fn expand(&mut self, pad: f64) {
        match self {
            AxisSpaceImpl::Linear(axis) => axis.expand(pad),
            AxisSpaceImpl::Log(axis) => axis.expand(pad),
        }
    }
}

/// Axis configuration.
///
/// Used to configure axis label and ticks.
#[derive(Clone)]
pub struct AxisHints<'a> {
    pub(super) label: WidgetText,
    pub(super) formatter: Arc<AxisFormatterFn<'a>>,
    pub(super) min_thickness: f32,
    pub(super) placement: Placement,
    pub(super) label_spacing: Rangef,
    pub(super) tick_label_color: Option<Color32>,
    pub(super) tick_label_font: Option<FontId>,
}

impl<'a> AxisHints<'a> {
    /// Initializes a default axis configuration for the X axis.
    pub fn new_x() -> Self {
        Self::new(Axis::X)
    }

    /// Initializes a default axis configuration for the Y axis.
    pub fn new_y() -> Self {
        Self::new(Axis::Y)
    }

    /// Initializes a default axis configuration for the specified axis.
    ///
    /// `label` is empty.
    /// `formatter` is default float to string formatter.
    pub fn new(axis: Axis) -> Self {
        Self {
            label: Default::default(),
            formatter: Arc::new(Self::default_formatter),
            min_thickness: 14.0,
            placement: Placement::LeftBottom,
            label_spacing: match axis {
                Axis::X => Rangef::new(60.0, 80.0), // labels can get pretty wide
                Axis::Y => Rangef::new(20.0, 30.0), // text isn't very high
            },
            tick_label_color: None,
            tick_label_font: None,
        }
    }

    /// Specify custom formatter for ticks.
    ///
    /// The first parameter of `formatter` is the raw tick value as `f64`.
    /// The second parameter of `formatter` is the currently shown range on this
    /// axis.
    #[inline]
    pub fn formatter(mut self, fmt: impl Fn(GridMark, &RangeInclusive<f64>) -> String + 'a) -> Self {
        self.formatter = Arc::new(fmt);
        self
    }

    fn default_formatter(mark: GridMark, _range: &RangeInclusive<f64>) -> String {
        // Example: If the step to the next tick is `0.01`, we should use 2 decimals of
        // precision:
        let num_decimals = -mark.step_size.log10().round() as usize;

        emath::format_with_decimals_in_range(mark.value, num_decimals..=num_decimals)
    }

    /// Specify axis label.
    ///
    /// The default is 'x' for x-axes and 'y' for y-axes.
    #[inline]
    pub fn label(mut self, label: impl Into<WidgetText>) -> Self {
        self.label = label.into();
        self
    }

    /// Specify minimum thickness of the axis
    #[inline]
    pub fn min_thickness(mut self, min_thickness: f32) -> Self {
        self.min_thickness = min_thickness;
        self
    }

    /// Specify maximum number of digits for ticks.
    #[inline]
    #[deprecated = "Use `min_thickness` instead"]
    pub fn max_digits(self, digits: usize) -> Self {
        self.min_thickness(12.0 * digits as f32)
    }

    /// Specify the placement of the axis.
    ///
    /// For X-axis, use [`VPlacement`].
    /// For Y-axis, use [`HPlacement`].
    #[inline]
    pub fn placement(mut self, placement: impl Into<Placement>) -> Self {
        self.placement = placement.into();
        self
    }

    /// Set the minimum spacing between labels
    ///
    /// When labels get closer together than the given minimum, then they become
    /// invisible. When they get further apart than the max, they are at
    /// full opacity.
    ///
    /// Labels can never be closer together than the
    /// [`crate::Plot::grid_spacing`] setting.
    #[inline]
    pub fn label_spacing(mut self, range: impl Into<Rangef>) -> Self {
        self.label_spacing = range.into();
        self
    }

    /// Set the color of the axis tick labels.
    #[inline]
    pub fn tick_label_color(mut self, color: impl Into<Color32>) -> Self {
        self.tick_label_color = Some(color.into());
        self
    }

    /// Set the font of the axis tick labels.
    #[inline]
    pub fn tick_label_font(mut self, font: FontId) -> Self {
        self.tick_label_font = Some(font);
        self
    }
}

#[derive(Clone)]
pub(super) struct AxisWidget<'a> {
    pub range: RangeInclusive<f64>,
    pub hints: AxisHints<'a>,

    /// The region where we draw the axis labels.
    pub rect: Rect,
    pub transform: Option<PlotTransform>,
    pub steps: Arc<Vec<GridMark>>,
}

impl<'a> AxisWidget<'a> {
    /// if `rect` has width or height == 0, it will be automatically calculated
    /// from ticks and text.
    pub fn new(hints: AxisHints<'a>, rect: Rect) -> Self {
        Self {
            range: (0.0..=0.0),
            hints,
            rect,
            transform: None,
            steps: Default::default(),
        }
    }

    /// Returns the actual thickness of the axis.
    pub fn ui(self, ui: &mut Ui, axis: Axis) -> (Response, f32) {
        let response = ui.allocate_rect(self.rect, Sense::hover());

        if !ui.is_rect_visible(response.rect) {
            return (response, 0.0);
        }

        let Some(transform) = self.transform else {
            return (response, 0.0);
        };
        let tick_labels_thickness = self.add_tick_labels(ui, transform, axis);

        if self.hints.label.is_empty() {
            return (response, tick_labels_thickness);
        }

        let galley = self
            .hints
            .label
            .into_galley(ui, Some(TextWrapMode::Extend), f32::INFINITY, TextStyle::Body);

        let text_pos = match self.hints.placement {
            Placement::LeftBottom => match axis {
                Axis::X => {
                    let pos = response.rect.center_bottom();
                    Pos2 {
                        x: pos.x - galley.size().x * 0.5,
                        y: pos.y - galley.size().y * (1.0 + AXIS_LABEL_GAP),
                    }
                }
                Axis::Y => {
                    let pos = response.rect.left_center();
                    Pos2 {
                        x: pos.x - galley.size().y * AXIS_LABEL_GAP,
                        y: pos.y + galley.size().x * 0.5,
                    }
                }
            },
            Placement::RightTop => match axis {
                Axis::X => {
                    let pos = response.rect.center_top();
                    Pos2 {
                        x: pos.x - galley.size().x * 0.5,
                        y: pos.y + galley.size().y * AXIS_LABEL_GAP,
                    }
                }
                Axis::Y => {
                    let pos = response.rect.right_center();
                    Pos2 {
                        x: pos.x - galley.size().y * (1.0 - AXIS_LABEL_GAP),
                        y: pos.y + galley.size().x * 0.5,
                    }
                }
            },
        };
        let axis_label_thickness = galley.size().y * (1.0 + AXIS_LABEL_GAP);
        let angle = match axis {
            Axis::X => 0.0,
            Axis::Y => -std::f32::consts::FRAC_PI_2,
        };

        ui.painter()
            .add(TextShape::new(text_pos, galley, ui.visuals().text_color()).with_angle(angle));

        (response, tick_labels_thickness + axis_label_thickness)
    }

    /// Add tick labels to the axis. Returns the thickness of the axis.
    fn add_tick_labels(&self, ui: &Ui, transform: PlotTransform, axis: Axis) -> f32 {
        let font_id = TextStyle::Body.resolve(ui.style());
        let label_spacing = self.hints.label_spacing;
        let mut thickness: f32 = 0.0;

        const SIDE_MARGIN: f32 = 4.0; // Add some margin to both sides of the text on the Y axis.
        let painter = ui.painter();
        let axis_space = transform.axis_space(axis);

        // Add tick labels:
        for step in self.steps.iter() {
            let text = (self.hints.formatter)(*step, &self.range);
            if !text.is_empty() {
                let spacing_in_points = axis_space
                    .screen_distance_between_values(step.value, step.value + step.step_size)
                    .abs();

                if spacing_in_points <= label_spacing.min {
                    // Labels are too close together - don't paint them.
                    continue;
                }

                // Fade in labels as they get further apart:
                let strength = remap_clamp(spacing_in_points, label_spacing, 0.0..=1.0);

                let text_color = if let Some(color) = self.hints.tick_label_color {
                    color.gamma_multiply(strength.sqrt())
                } else {
                    super::color_from_strength(ui, strength)
                };

                let label_font_id = self.hints.tick_label_font.clone().unwrap_or_else(|| font_id.clone());

                let galley = painter.layout_no_wrap(text, label_font_id, text_color);
                let galley_size = match axis {
                    Axis::X => galley.size(),
                    Axis::Y => galley.size() + 2.0 * SIDE_MARGIN * Vec2::X,
                };

                if spacing_in_points < galley_size[axis as usize] {
                    continue; // the galley won't fit (likely too wide on the X axis).
                }

                match axis {
                    Axis::X => {
                        thickness = thickness.max(galley_size.y);

                        let projected_point = super::PlotPoint::new(step.value, 0.0);
                        let center_x = transform.position_from_point(&projected_point).x;
                        let y = match VPlacement::from(self.hints.placement) {
                            VPlacement::Bottom => self.rect.min.y,
                            VPlacement::Top => self.rect.max.y - galley_size.y,
                        };
                        let pos = Pos2::new(center_x - galley_size.x / 2.0, y);
                        painter.add(TextShape::new(pos, galley, text_color));
                    }
                    Axis::Y => {
                        thickness = thickness.max(galley_size.x);

                        let projected_point = super::PlotPoint::new(0.0, step.value);
                        let center_y = transform.position_from_point(&projected_point).y;

                        match HPlacement::from(self.hints.placement) {
                            HPlacement::Left => {
                                let angle = 0.0; // TODO(#162): allow users to rotate text

                                if angle == 0.0 {
                                    let x = self.rect.max.x - galley_size.x + SIDE_MARGIN;
                                    let pos = Pos2::new(x, center_y - galley_size.y / 2.0);
                                    painter.add(TextShape::new(pos, galley, text_color));
                                } else {
                                    let right = Pos2::new(self.rect.max.x, center_y - galley_size.y / 2.0);
                                    let width = galley_size.x;
                                    let left = right - Rot2::from_angle(angle) * Vec2::new(width, 0.0);

                                    painter.add(TextShape::new(left, galley, text_color).with_angle(angle));
                                }
                            }
                            HPlacement::Right => {
                                let x = self.rect.min.x + SIDE_MARGIN;
                                let pos = Pos2::new(x, center_y - galley_size.y / 2.0);
                                painter.add(TextShape::new(pos, galley, text_color));
                            }
                        }
                    }
                }
            }
        }
        thickness
    }
}
