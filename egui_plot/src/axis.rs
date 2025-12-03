use std::fmt::Debug;
use std::ops::RangeInclusive;
use std::sync::Arc;

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
use emath::{pos2, remap, Vec2b};
use crate::{colors, PlotBounds, PlotPoint};
use crate::grid::GridMark;
use crate::placement::HPlacement;
use crate::placement::Placement;
use crate::placement::VPlacement;

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

        // Add tick labels:
        for step in self.steps.iter() {
            let text = (self.hints.formatter)(*step, &self.range);
            if !text.is_empty() {
                let spacing_in_points = (transform.dpos_dvalue()[usize::from(axis)] * step.step_size).abs() as f32;

                if spacing_in_points <= label_spacing.min {
                    // Labels are too close together - don't paint them.
                    continue;
                }

                // Fade in labels as they get further apart:
                let strength = remap_clamp(spacing_in_points, label_spacing, 0.0..=1.0);

                let text_color = colors::color_from_strength(ui, strength);
                let galley = painter.layout_no_wrap(text, font_id.clone(), text_color);
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

/// Contains the screen rectangle and the plot bounds and provides methods to
/// transform between them.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug)]
pub struct PlotTransform {
    /// The screen rectangle.
    frame: Rect,

    /// The plot bounds.
    bounds: PlotBounds,

    /// Whether to always center the x-range or y-range of the bounds.
    centered: Vec2b,

    /// Whether to always invert the x and/or y axis
    inverted_axis: Vec2b,
}

impl PlotTransform {
    pub fn new(frame: Rect, bounds: PlotBounds, center_axis: impl Into<Vec2b>) -> Self {
        debug_assert!(
            0.0 <= frame.width() && 0.0 <= frame.height(),
            "Bad plot frame: {frame:?}"
        );
        let center_axis = center_axis.into();

        // Since the current Y bounds an affect the final X bounds and vice versa, we
        // need to keep the original version of the `bounds` before we start
        // modifying it.
        let mut new_bounds = bounds;

        // Sanitize bounds.
        //
        // When a given bound axis is "thin" (e.g. width or height is 0) but finite, we
        // center the bounds around that value. If the other axis is "fat", we
        // reuse its extent for the thin axis, and default to +/- 1.0 otherwise.
        if !bounds.is_finite_x() {
            new_bounds.set_x(&PlotBounds::new_symmetrical(1.0));
        } else if bounds.width() <= 0.0 {
            new_bounds.set_x_center_width(
                bounds.center().x,
                if bounds.is_valid_y() { bounds.height() } else { 1.0 },
            );
        }

        if !bounds.is_finite_y() {
            new_bounds.set_y(&PlotBounds::new_symmetrical(1.0));
        } else if bounds.height() <= 0.0 {
            new_bounds.set_y_center_height(
                bounds.center().y,
                if bounds.is_valid_x() { bounds.width() } else { 1.0 },
            );
        }

        // Scale axes so that the origin is in the center.
        if center_axis.x {
            new_bounds.make_x_symmetrical();
        }
        if center_axis.y {
            new_bounds.make_y_symmetrical();
        }

        debug_assert!(new_bounds.is_valid(), "Bad final plot bounds: {new_bounds:?}");

        Self {
            frame,
            bounds: new_bounds,
            centered: center_axis,
            inverted_axis: Vec2b::new(false, false),
        }
    }

    pub fn new_with_invert_axis(
        frame: Rect,
        bounds: PlotBounds,
        center_axis: impl Into<Vec2b>,
        invert_axis: impl Into<Vec2b>,
    ) -> Self {
        let mut new = Self::new(frame, bounds, center_axis);
        new.inverted_axis = invert_axis.into();
        new
    }

    /// ui-space rectangle.
    #[inline]
    pub fn frame(&self) -> &Rect {
        &self.frame
    }

    /// Plot-space bounds.
    #[inline]
    pub fn bounds(&self) -> &PlotBounds {
        &self.bounds
    }

    #[inline]
    pub fn set_bounds(&mut self, bounds: PlotBounds) {
        self.bounds = bounds;
    }

    pub fn translate_bounds(&mut self, mut delta_pos: (f64, f64)) {
        if self.centered.x {
            delta_pos.0 = 0.;
        }
        if self.centered.y {
            delta_pos.1 = 0.;
        }
        delta_pos.0 *= self.dvalue_dpos()[0];
        delta_pos.1 *= self.dvalue_dpos()[1];
        self.bounds.translate((delta_pos.0, delta_pos.1));
    }

    /// Zoom by a relative factor with the given screen position as center.
    pub fn zoom(&mut self, zoom_factor: Vec2, center: Pos2) {
        let center = self.value_from_position(center);

        let mut new_bounds = self.bounds;
        new_bounds.zoom(zoom_factor, center);

        if new_bounds.is_valid() {
            self.bounds = new_bounds;
        }
    }

    pub fn position_from_point_x(&self, value: f64) -> f32 {
        remap(
            value,
            self.bounds.min[0]..=self.bounds.max[0],
            if self.inverted_axis[0] {
                (self.frame.right() as f64)..=(self.frame.left() as f64)
            } else {
                (self.frame.left() as f64)..=(self.frame.right() as f64)
            },
        ) as f32
    }

    pub fn position_from_point_y(&self, value: f64) -> f32 {
        remap(
            value,
            self.bounds.min[1]..=self.bounds.max[1],
            // negated y axis by default
            if self.inverted_axis[1] {
                (self.frame.top() as f64)..=(self.frame.bottom() as f64)
            } else {
                (self.frame.bottom() as f64)..=(self.frame.top() as f64)
            },
        ) as f32
    }

    /// Screen/ui position from point on plot.
    pub fn position_from_point(&self, value: &PlotPoint) -> Pos2 {
        pos2(self.position_from_point_x(value.x), self.position_from_point_y(value.y))
    }

    /// Plot point from screen/ui position.
    pub fn value_from_position(&self, pos: Pos2) -> PlotPoint {
        let x = remap(
            pos.x as f64,
            if self.inverted_axis[0] {
                (self.frame.right() as f64)..=(self.frame.left() as f64)
            } else {
                (self.frame.left() as f64)..=(self.frame.right() as f64)
            },
            self.bounds.range_x(),
        );
        let y = remap(
            pos.y as f64,
            // negated y axis by default
            if self.inverted_axis[1] {
                (self.frame.top() as f64)..=(self.frame.bottom() as f64)
            } else {
                (self.frame.bottom() as f64)..=(self.frame.top() as f64)
            },
            self.bounds.range_y(),
        );

        PlotPoint::new(x, y)
    }

    /// Transform a rectangle of plot values to a screen-coordinate rectangle.
    ///
    /// This typically means that the rect is mirrored vertically (top becomes
    /// bottom and vice versa), since the plot's coordinate system has +Y
    /// up, while egui has +Y down.
    pub fn rect_from_values(&self, value1: &PlotPoint, value2: &PlotPoint) -> Rect {
        let pos1 = self.position_from_point(value1);
        let pos2 = self.position_from_point(value2);

        let mut rect = Rect::NOTHING;
        rect.extend_with(pos1);
        rect.extend_with(pos2);
        rect
    }

    /// delta position / delta value = how many ui points per step in the X axis
    /// in "plot space"
    pub fn dpos_dvalue_x(&self) -> f64 {
        let flip = if self.inverted_axis[0] { -1.0 } else { 1.0 };
        flip * (self.frame.width() as f64) / self.bounds.width()
    }

    /// delta position / delta value = how many ui points per step in the Y axis
    /// in "plot space"
    pub fn dpos_dvalue_y(&self) -> f64 {
        let flip = if self.inverted_axis[1] { 1.0 } else { -1.0 };
        flip * (self.frame.height() as f64) / self.bounds.height()
    }

    /// delta position / delta value = how many ui points per step in "plot
    /// space"
    pub fn dpos_dvalue(&self) -> [f64; 2] {
        [self.dpos_dvalue_x(), self.dpos_dvalue_y()]
    }

    /// delta value / delta position = how much ground do we cover in "plot
    /// space" per ui point?
    pub fn dvalue_dpos(&self) -> [f64; 2] {
        [1.0 / self.dpos_dvalue_x(), 1.0 / self.dpos_dvalue_y()]
    }

    /// scale.x/scale.y ratio.
    ///
    /// If 1.0, it means the scale factor is the same in both axes.
    fn aspect(&self) -> f64 {
        let rw = self.frame.width() as f64;
        let rh = self.frame.height() as f64;
        (self.bounds.width() / rw) / (self.bounds.height() / rh)
    }

    /// Sets the aspect ratio by expanding the x- or y-axis.
    ///
    /// This never contracts, so we don't miss out on any data.
    pub(crate) fn set_aspect_by_expanding(&mut self, aspect: f64) {
        let current_aspect = self.aspect();

        let epsilon = 1e-5;
        if (current_aspect - aspect).abs() < epsilon {
            // Don't make any changes when the aspect is already almost correct.
            return;
        }

        if current_aspect < aspect {
            self.bounds
                .expand_x((aspect / current_aspect - 1.0) * self.bounds.width() * 0.5);
        } else {
            self.bounds
                .expand_y((current_aspect / aspect - 1.0) * self.bounds.height() * 0.5);
        }
    }

    /// Sets the aspect ratio by changing either the X or Y axis (callers
    /// choice).
    pub(crate) fn set_aspect_by_changing_axis(&mut self, aspect: f64, axis: Axis) {
        let current_aspect = self.aspect();

        let epsilon = 1e-5;
        if (current_aspect - aspect).abs() < epsilon {
            // Don't make any changes when the aspect is already almost correct.
            return;
        }

        match axis {
            Axis::X => {
                self.bounds
                    .expand_x((aspect / current_aspect - 1.0) * self.bounds.width() * 0.5);
            }
            Axis::Y => {
                self.bounds
                    .expand_y((current_aspect / aspect - 1.0) * self.bounds.height() * 0.5);
            }
        }
    }
}