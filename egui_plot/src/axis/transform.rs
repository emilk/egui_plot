//! Handles the transformations between the data space
//! and the screen space.
//!
// Broadly speaking f32 values are screen units and f64 are
// value units. We could consider a unit type in the future to
// make this explicit.

use std::fmt::Debug;
use crate::{Axis, AxisScale, GridInput, PlotBounds, PlotPoint};
use emath::{Pos2, Rect, Vec2, Vec2b, pos2};
use std::ops::RangeInclusive;
use crate::axis::AxisSpaceImpl;

pub trait AxisSpace: Debug + Clone {
    /// The minimum value of the axis.
    fn value_min(&self) -> f64;
    /// The maximum value of the axis.
    fn value_max(&self) -> f64;
    /// The smallest edge of the frame.
    fn frame_min(&self) -> f32;
    /// The largest edge of the frame.
    fn frame_max(&self) -> f32;

    /// Get the length of the value range.
    fn value_length(&self) -> f64 {
        self.value_max() - self.value_min()
    }

    /// Confirm all aspects of the axis space are valid.
    fn is_valid(&self) -> bool {
        self.value_min().is_finite() && self.value_max().is_finite() && self.value_length() > 0.0
    }
    /// Set whether the axis is inverted on the screen.
    fn set_inverted(&mut self, invert: bool);

    /// Set the value range of the axis.
    fn set_value_range(&mut self, range: RangeInclusive<f64>);

    /// Convert a value to a screen position.
    fn position_from_value(&self, value: f64) -> f32;

    /// Convert a screen position to a value.
    fn value_from_position(&self, position: f32) -> f64;

    /// Provides the minimum value step for the screen step size provided.
    fn minimum_value_step(&self, spacing: f32) -> f64;

    /// Get the grid input configuration for the axis given the provided
    /// minimum gap in screen units.
    fn grid_input(&self, spacing: f32) -> GridInput;

    /// Screen distance between two points.
    fn screen_distance_between_values(&self, value1: f64, value2: f64) -> f32;

    /// Alter the value minimum and maximum to produce a translation
    /// in the screen space.
    fn translate(&mut self, frame_distance: f32);
    /// Zoom around the center point.
    fn zoom(&mut self, factor: f32, center: f64);

    /// Extend both ends of the value range by the provided `pad` value.
    fn expand(&mut self, pad: f64) {
        if pad.is_finite() {
            let new_min = self.value_min() - pad;
            let new_max = self.value_max() + pad;
            self.set_value_range(new_min..=new_max);
        }
    }
}

/// Contains the screen rectangle and the plot bounds and provides methods to
/// transform between them.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug)]
pub struct PlotTransform {
    /// The X axis space
    x_axis: AxisSpaceImpl,

    /// The Y axis space
    y_axis: AxisSpaceImpl,

    /// Whether to always center the x-range or y-range of the bounds.
    centered: Vec2b,
}

impl PlotTransform {
    pub fn new(frame: Rect, bounds: PlotBounds, x_scale: AxisScale, y_scale: AxisScale, center_axis: impl Into<Vec2b>) -> Self {
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
        let x_axis = x_scale.new_axis(bounds.range_x(), frame.x_range(), false);
        // Y is naturally inverted.
        let y_axis = y_scale.new_axis(bounds.range_y(), frame.y_range(), true);

        Self {
            centered: center_axis,
            x_axis,
            y_axis,
        }
    }

    pub fn new_with_invert_axis(
        frame: Rect,
        bounds: PlotBounds,
        x_scale: AxisScale,
        y_scale: AxisScale,
        center_axis: impl Into<Vec2b>,
        invert_axis: impl Into<Vec2b>,
    ) -> Self {
        let mut new = Self::new(frame, bounds, x_scale, y_scale, center_axis);
        let inverted = invert_axis.into();
        new.x_axis.set_inverted(inverted.x);
        // y is naturally inverted so !inverted.y is the required input.
        new.y_axis.set_inverted(!inverted.y);
        new
    }

    /// ui-space rectangle.
    #[inline]
    pub fn frame(&self) -> Rect {
        let min = Pos2::new(self.x_axis.frame_min(), self.y_axis.frame_min());
        let max = Pos2::new(self.x_axis.frame_max(), self.y_axis.frame_max());
        Rect::from_min_max(min, max)
    }

    /// Plot-space bounds.
    #[inline]
    pub fn bounds(&self) -> PlotBounds {
        PlotBounds::from_min_max(
            [self.x_axis.value_min(), self.y_axis.value_min()],
            [self.x_axis.value_max(), self.y_axis.value_max()],
        )
    }

    #[inline]
    pub fn set_bounds(&mut self, bounds: PlotBounds) {
        self.x_axis.set_value_range(bounds.range_x());
        self.y_axis.set_value_range(bounds.range_y());
    }

    pub fn translate_bounds(&mut self, mut delta_pos: (f64, f64)) {
        if self.centered.x {
            delta_pos.0 = 0.;
        }
        if self.centered.y {
            delta_pos.1 = 0.;
        }
        self.x_axis.translate(delta_pos.0 as f32);
        self.y_axis.translate(delta_pos.1 as f32);
    }

    /// Zoom by a relative factor with the given screen position as center.
    pub fn zoom(&mut self, zoom_factor: Vec2, center: Pos2) {
        let center = self.value_from_position(center);

        let mut new_x = self.x_axis;
        let mut new_y = self.y_axis;
        new_x.zoom(zoom_factor.x, center.x);
        new_y.zoom(zoom_factor.y, center.y);

        if new_x.is_valid() && new_y.is_valid() {
            self.x_axis = new_x;
            self.y_axis = new_y;
        }
    }

    pub(crate) fn axis_space(&self, axis: Axis) -> &impl AxisSpace {
        match axis {
            Axis::X => &self.x_axis,
            Axis::Y => &self.y_axis,
        }
    }

    pub fn position_from_point_x(&self, value: f64) -> f32 {
        self.x_axis.position_from_value(value)
    }

    pub fn position_from_point_y(&self, value: f64) -> f32 {
        self.y_axis.position_from_value(value)
    }

    /// Screen/ui position from point on plot.
    pub fn position_from_point(&self, value: &PlotPoint) -> Pos2 {
        pos2(self.position_from_point_x(value.x), self.position_from_point_y(value.y))
    }

    /// Plot point from screen/ui position.
    pub fn value_from_position(&self, pos: Pos2) -> PlotPoint {
        let x = self.x_axis.value_from_position(pos.x);
        let y = self.y_axis.value_from_position(pos.y);
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

    /// scale.x/scale.y ratio.
    ///
    /// If 1.0, it means the scale factor is the same in both axes.
    fn aspect(&self) -> f64 {
        let x_value_range = self.x_axis.value_length().abs();
        let y_value_range = self.y_axis.value_length().abs();
        let x_frame_range = self.x_axis.frame_max() - self.x_axis.frame_min();
        let y_frame_range = self.y_axis.frame_max() - self.y_axis.frame_min();
        (x_value_range / x_frame_range as f64) / (y_value_range / y_frame_range as f64)
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
            self.x_axis
                .expand((aspect / current_aspect - 1.0) * self.x_axis.value_length() * 0.5);
        } else {
            self.y_axis
                .expand((current_aspect / aspect - 1.0) * self.y_axis.value_length() * 0.5);
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
                self.x_axis
                    .expand((aspect / current_aspect - 1.0) * self.x_axis.value_length() * 0.5);
            }
            Axis::Y => {
                self.y_axis
                    .expand((current_aspect / aspect - 1.0) * self.y_axis.value_length() * 0.5);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertables::assert_approx_eq;

    #[test]
    fn test_basic_linear_axis_transform() {
        let frame = Rect::from_min_max(pos2(0.0, 0.0), pos2(100.0, 100.0));
        let bounds = PlotBounds::new_symmetrical(10.0);

        let transform = PlotTransform::new(frame, bounds, AxisScale::Linear, AxisScale::Linear, false);

        assert_eq!(
            transform.position_from_point(&PlotPoint::new(0.0, 0.0)),
            pos2(50.0, 50.0)
        );
        assert_eq!(
            transform.value_from_position(pos2(50.0, 50.0)),
            PlotPoint::new(0.0, 0.0)
        );
        assert_eq!(
            transform.position_from_point(&PlotPoint::new(10.0, 10.0)),
            pos2(100.0, 0.0)
        );
        assert_eq!(
            transform.value_from_position(pos2(100.0, 0.0)),
            PlotPoint::new(10.0, 10.0)
        );
        assert_eq!(
            transform.position_from_point(&PlotPoint::new(-10.0, -10.0)),
            pos2(0.0, 100.0)
        );
        assert_eq!(
            transform.value_from_position(pos2(0.0, 100.0)),
            PlotPoint::new(-10.0, -10.0)
        );
        assert_eq!(
            transform.position_from_point(&PlotPoint::new(5.0, -5.0)),
            pos2(75.0, 75.0)
        );
        assert_eq!(
            transform.value_from_position(pos2(75.0, 75.0)),
            PlotPoint::new(5.0, -5.0)
        );
    }

    #[test]
    fn test_inverted_linear_axis_transform() {
        let frame = Rect::from_min_max(pos2(0.0, 0.0), pos2(100.0, 100.0));
        let bounds = PlotBounds::new_symmetrical(10.0);
        let invert_axis = Vec2b::new(true, false);
        let transform = PlotTransform::new_with_invert_axis(frame, bounds, AxisScale::Linear, AxisScale::Linear,false, invert_axis);
        assert_eq!(
            transform.position_from_point(&PlotPoint::new(0.0, 0.0)),
            pos2(50.0, 50.0)
        );
        assert_eq!(
            transform.value_from_position(pos2(50.0, 50.0)),
            PlotPoint::new(0.0, 0.0)
        );
        assert_eq!(
            transform.position_from_point(&PlotPoint::new(10.0, 10.0)),
            pos2(0.0, 0.0)
        );
        assert_eq!(
            transform.value_from_position(pos2(0.0, 0.0)),
            PlotPoint::new(10.0, 10.0)
        );
        assert_eq!(
            transform.position_from_point(&PlotPoint::new(-10.0, -10.0)),
            pos2(100.0, 100.0)
        );
        assert_eq!(
            transform.value_from_position(pos2(100.0, 100.0)),
            PlotPoint::new(-10.0, -10.0)
        );
        assert_eq!(
            transform.position_from_point(&PlotPoint::new(5.0, -5.0)),
            pos2(25.0, 75.0)
        );
        assert_eq!(
            transform.value_from_position(pos2(25.0, 75.0)),
            PlotPoint::new(5.0, -5.0)
        );
    }

    #[test]
    fn aspect_ratio_calculation() {
        let frame = Rect::from_min_max(pos2(0.0, 0.0), pos2(100.0, 100.0));
        let bounds = PlotBounds::new_symmetrical(10.0);
        let transform = PlotTransform::new(frame, bounds, AxisScale::Linear, AxisScale::Linear, false);
        assert_eq!(transform.aspect(), 1.0);

        let frame = Rect::from_min_max(pos2(0.0, 0.0), pos2(100.0, 100.0));
        let bounds = PlotBounds::from_min_max([0.0, 0.0], [100.0, 50.0]);
        let transform = PlotTransform::new(frame, bounds, AxisScale::Linear, AxisScale::Linear, false);
        assert_eq!(transform.aspect(), 2.0);

        let frame = Rect::from_min_max(pos2(0.0, 0.0), pos2(100.0, 50.0));
        let bounds = PlotBounds::new_symmetrical(10.0);
        let transform = PlotTransform::new(frame, bounds, AxisScale::Linear, AxisScale::Linear, false);
        assert_eq!(transform.aspect(), 0.5);
    }

    /// Real values captured from a test which was failing.
    #[test]
    fn aspect_calculation_from_custom_axes() {
        let frame = Rect::from_min_max(Pos2::new(22.0, 49.0), Pos2::new(778.0, 564.0));
        let bounds = PlotBounds::from_min_max([-360.0, -0.04294], [7560.0, 1.049]);
        let transform = PlotTransform::new(frame, bounds, AxisScale::Linear, AxisScale::Linear, [false, false]);
        assert_approx_eq!(transform.aspect(), 4940.965708);
    }

    fn assert_bounds_eq(actual: PlotBounds, expected_min: [f64; 2], expected_max: [f64; 2]) {
        let epsilon = 1e-9;
        assert!(
            (actual.min()[0] - expected_min[0]).abs() < epsilon
                && (actual.min()[1] - expected_min[1]).abs() < epsilon
                && (actual.max()[0] - expected_max[0]).abs() < epsilon
                && (actual.max()[1] - expected_max[1]).abs() < epsilon,
            "bounds mismatch: got min={:?} max={:?}, expected min={:?} max={:?}",
            actual.min(),
            actual.max(),
            expected_min,
            expected_max,
        );
    }

    #[test]
    fn set_aspect_by_expanding_noop_when_already_correct() {
        let frame = Rect::from_min_max(pos2(0.0, 0.0), pos2(100.0, 100.0));
        let bounds = PlotBounds::new_symmetrical(10.0);
        let mut transform = PlotTransform::new(frame, bounds, AxisScale::Linear, AxisScale::Linear, false);
        let original = transform.bounds();

        transform.set_aspect_by_expanding(1.0);

        assert_bounds_eq(transform.bounds(), original.min(), original.max());
    }

    #[test]
    fn set_aspect_by_expanding_grows_x_when_current_is_smaller() {
        // current_aspect = (10/100) / (20/100) = 0.5, target = 1.0 -> grow x
        let frame = Rect::from_min_max(pos2(0.0, 0.0), pos2(100.0, 100.0));
        let bounds = PlotBounds::from_min_max([0.0, 0.0], [10.0, 20.0]);
        let mut transform = PlotTransform::new(frame, bounds, AxisScale::Linear, AxisScale::Linear, false);

        transform.set_aspect_by_expanding(1.0);

        assert_bounds_eq(transform.bounds(), [-5.0, 0.0], [15.0, 20.0]);
        assert!((transform.aspect() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn set_aspect_by_expanding_grows_y_when_current_is_larger() {
        // current_aspect = (20/100) / (10/100) = 2.0, target = 1.0 -> grow y
        let frame = Rect::from_min_max(pos2(0.0, 0.0), pos2(100.0, 100.0));
        let bounds = PlotBounds::from_min_max([0.0, 0.0], [20.0, 10.0]);
        let mut transform = PlotTransform::new(frame, bounds, AxisScale::Linear, AxisScale::Linear, false);

        transform.set_aspect_by_expanding(1.0);

        assert_bounds_eq(transform.bounds(), [0.0, -5.0], [20.0, 15.0]);
        assert!((transform.aspect() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn set_aspect_by_expanding_accounts_for_non_square_frame() {
        // frame 100x50, bounds 20x20 -> current_aspect = (20/100)/(20/50) = 0.5
        // target 1.0 -> grow x by (1/0.5 - 1) * 20 * 0.5 = 10 on each side
        let frame = Rect::from_min_max(pos2(0.0, 0.0), pos2(100.0, 50.0));
        let bounds = PlotBounds::new_symmetrical(10.0);
        let mut transform = PlotTransform::new(frame, bounds, AxisScale::Linear, AxisScale::Linear, false);

        transform.set_aspect_by_expanding(1.0);

        assert_bounds_eq(transform.bounds(), [-20.0, -10.0], [20.0, 10.0]);
        assert!((transform.aspect() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn set_aspect_by_changing_axis_x_grows() {
        // current_aspect = 0.5, target = 1.0, change X -> grow x
        let frame = Rect::from_min_max(pos2(0.0, 0.0), pos2(100.0, 100.0));
        let bounds = PlotBounds::from_min_max([0.0, 0.0], [10.0, 20.0]);
        let mut transform = PlotTransform::new(frame, bounds, AxisScale::Linear, AxisScale::Linear, false);

        transform.set_aspect_by_changing_axis(1.0, Axis::X);

        assert_bounds_eq(transform.bounds(), [-5.0, 0.0], [15.0, 20.0]);
        assert!((transform.aspect() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn set_aspect_by_changing_axis_x_shrinks() {
        // current_aspect = 2.0, target = 1.0, change X -> shrink x
        // pad = (1/2 - 1) * 20 * 0.5 = -5 -> min += 5, max -= 5
        let frame = Rect::from_min_max(pos2(0.0, 0.0), pos2(100.0, 100.0));
        let bounds = PlotBounds::from_min_max([0.0, 0.0], [20.0, 10.0]);
        let mut transform = PlotTransform::new(frame, bounds, AxisScale::Linear, AxisScale::Linear, false);

        transform.set_aspect_by_changing_axis(1.0, Axis::X);

        assert_bounds_eq(transform.bounds(), [5.0, 0.0], [15.0, 10.0]);
        assert!((transform.aspect() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn set_aspect_by_changing_axis_y_grows() {
        // current_aspect = 2.0, target = 1.0, change Y -> grow y
        let frame = Rect::from_min_max(pos2(0.0, 0.0), pos2(100.0, 100.0));
        let bounds = PlotBounds::from_min_max([0.0, 0.0], [20.0, 10.0]);
        let mut transform = PlotTransform::new(frame, bounds, AxisScale::Linear, AxisScale::Linear, false);

        transform.set_aspect_by_changing_axis(1.0, Axis::Y);

        assert_bounds_eq(transform.bounds(), [0.0, -5.0], [20.0, 15.0]);
        assert!((transform.aspect() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn set_aspect_by_changing_axis_y_shrinks() {
        // current_aspect = 0.5, target = 1.0, change Y -> shrink y
        // pad = (0.5/1 - 1) * 20 * 0.5 = -5 -> min += 5, max -= 5
        let frame = Rect::from_min_max(pos2(0.0, 0.0), pos2(100.0, 100.0));
        let bounds = PlotBounds::from_min_max([0.0, 0.0], [10.0, 20.0]);
        let mut transform = PlotTransform::new(frame, bounds, AxisScale::Linear, AxisScale::Linear, false);

        transform.set_aspect_by_changing_axis(1.0, Axis::Y);

        assert_bounds_eq(transform.bounds(), [0.0, 5.0], [10.0, 15.0]);
        assert!((transform.aspect() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn set_aspect_by_changing_axis_noop_when_already_correct() {
        let frame = Rect::from_min_max(pos2(0.0, 0.0), pos2(100.0, 100.0));
        let bounds = PlotBounds::new_symmetrical(10.0);
        let mut transform = PlotTransform::new(frame, bounds, AxisScale::Linear, AxisScale::Linear, false);
        let original = transform.bounds();

        transform.set_aspect_by_changing_axis(1.0, Axis::X);
        assert_bounds_eq(transform.bounds(), original.min(), original.max());

        transform.set_aspect_by_changing_axis(1.0, Axis::Y);
        assert_bounds_eq(transform.bounds(), original.min(), original.max());
    }
}
