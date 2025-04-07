use std::{fmt::Debug, ops::RangeInclusive, sync::Arc};

use egui::{
    emath::{remap_clamp, Rot2},
    epaint::TextShape,
    Pos2, Rangef, Rect, Response, Sense, TextStyle, TextWrapMode, Ui, Vec2, WidgetText,
};

use super::{transform::PlotTransform, GridMark};

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

/// Placement of the horizontal X-Axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VPlacement {
    Top,
    Bottom,
}

/// Placement of the vertical Y-Axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HPlacement {
    Left,
    Right,
}

/// Placement of an axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Placement {
    /// Bottom for X-axis, or left for Y-axis.
    LeftBottom,

    /// Top for x-axis and right for y-axis.
    RightTop,
}

impl From<HPlacement> for Placement {
    #[inline]
    fn from(placement: HPlacement) -> Self {
        match placement {
            HPlacement::Left => Self::LeftBottom,
            HPlacement::Right => Self::RightTop,
        }
    }
}

impl From<Placement> for HPlacement {
    #[inline]
    fn from(placement: Placement) -> Self {
        match placement {
            Placement::LeftBottom => Self::Left,
            Placement::RightTop => Self::Right,
        }
    }
}

impl From<VPlacement> for Placement {
    #[inline]
    fn from(placement: VPlacement) -> Self {
        match placement {
            VPlacement::Top => Self::RightTop,
            VPlacement::Bottom => Self::LeftBottom,
        }
    }
}

impl From<Placement> for VPlacement {
    #[inline]
    fn from(placement: Placement) -> Self {
        match placement {
            Placement::LeftBottom => Self::Bottom,
            Placement::RightTop => Self::Top,
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
}

// TODO(JohannesProgrammiert): this just a guess. It might cease to work if a user changes font size.
const LINE_HEIGHT: f32 = 12.0;

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
        }
    }

    /// Specify custom formatter for ticks.
    ///
    /// The first parameter of `formatter` is the raw tick value as `f64`.
    /// The second parameter of `formatter` is the currently shown range on this axis.
    pub fn formatter(
        mut self,
        fmt: impl Fn(GridMark, &RangeInclusive<f64>) -> String + 'a,
    ) -> Self {
        self.formatter = Arc::new(fmt);
        self
    }

    fn default_formatter(mark: GridMark, _range: &RangeInclusive<f64>) -> String {
        // Example: If the step to the next tick is `0.01`, we should use 2 decimals of precision:
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
    /// When labels get closer together than the given minimum, then they become invisible.
    /// When they get further apart than the max, they are at full opacity.
    ///
    /// Labels can never be closer together than the [`crate::Plot::grid_spacing`] setting.
    #[inline]
    pub fn label_spacing(mut self, range: impl Into<Rangef>) -> Self {
        self.label_spacing = range.into();
        self
    }

    pub(super) fn thickness(&self, axis: Axis) -> f32 {
        match axis {
            Axis::X => self.min_thickness.max(if self.label.is_empty() {
                1.0 * LINE_HEIGHT
            } else {
                3.0 * LINE_HEIGHT
            }),
            Axis::Y => {
                self.min_thickness
                    + if self.label.is_empty() {
                        0.0
                    } else {
                        LINE_HEIGHT
                    }
            }
        }
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
    /// if `rect` has width or height == 0, it will be automatically calculated from ticks and text.
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

        let galley = self.hints.label.into_galley(
            ui,
            Some(TextWrapMode::Extend),
            f32::INFINITY,
            TextStyle::Body,
        );

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
        // Add tick labels:
        for step in self.steps.iter() {
            let text = (self.hints.formatter)(*step, &self.range);
            if !text.is_empty() {
                let spacing_in_points = transform.points_at_pos_range(
                    [step.value, step.value],
                    [step.step_size, step.step_size],
                )[usize::from(axis)]
                .abs();

                if spacing_in_points <= label_spacing.min {
                    // Labels are too close together - don't paint them.
                    continue;
                }

                // Fade in labels as they get further apart:
                let strength = remap_clamp(spacing_in_points, label_spacing, 0.0..=1.0);

                let text_color = super::color_from_strength(ui, strength);
                let galley = ui
                    .painter()
                    .layout_no_wrap(text, font_id.clone(), text_color);

                if spacing_in_points < galley.size()[axis as usize] {
                    continue; // the galley won't fit (likely too wide on the X axis).
                }

                match axis {
                    Axis::X => {
                        thickness = thickness.max(galley.size().y);

                        let projected_point = super::PlotPoint::new(step.value, 0.0);
                        let center_x = transform.position_from_point(&projected_point).x;
                        let y = match VPlacement::from(self.hints.placement) {
                            VPlacement::Bottom => self.rect.min.y,
                            VPlacement::Top => self.rect.max.y - galley.size().y,
                        };
                        let pos = Pos2::new(center_x - galley.size().x / 2.0, y);
                        ui.painter().add(TextShape::new(pos, galley, text_color));
                    }
                    Axis::Y => {
                        thickness = thickness.max(galley.size().x);

                        let projected_point = super::PlotPoint::new(0.0, step.value);
                        let center_y = transform.position_from_point(&projected_point).y;

                        match HPlacement::from(self.hints.placement) {
                            HPlacement::Left => {
                                let angle = 0.0; // TODO(emilk): allow users to rotate text

                                if angle == 0.0 {
                                    let x = self.rect.max.x - galley.size().x;
                                    let pos = Pos2::new(x, center_y - galley.size().y / 2.0);
                                    ui.painter().add(TextShape::new(pos, galley, text_color));
                                } else {
                                    let right = Pos2::new(
                                        self.rect.max.x,
                                        center_y - galley.size().y / 2.0,
                                    );
                                    let width = galley.size().x;
                                    let left =
                                        right - Rot2::from_angle(angle) * Vec2::new(width, 0.0);

                                    ui.painter().add(
                                        TextShape::new(left, galley, text_color).with_angle(angle),
                                    );
                                }
                            }
                            HPlacement::Right => {
                                let x = self.rect.min.x;
                                let pos = Pos2::new(x, center_y - galley.size().y / 2.0);
                                ui.painter().add(TextShape::new(pos, galley, text_color));
                            }
                        };
                    }
                };
            }
        }
        thickness
    }
}
