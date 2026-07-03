use crate::GridInput;
use crate::axis::linear::LinearAxisSpace;
use crate::axis::transform::AxisSpace;
use emath::Rangef;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogBase {
    Base10,
    Base2,
}

impl LogBase {
    /// Returns the exponent of the given value (the logarithm of the value).
    ///
    /// Returns `None` if the value is negative or zero.
    pub fn exponent(&self, value: f64) -> Option<f64> {
        if value <= 0.0 {
            return None;
        }
        match self {
            Self::Base10 => Some(value.log10()),
            Self::Base2 => Some(value.log2()),
        }
    }

    pub fn power(&self, value: f64) -> f64 {
        match self {
            Self::Base10 => f64::powf(10.0, value),
            Self::Base2 => value.exp2(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LogAxis {
    /// A linear scale for the exponent of the logarithm.
    ///
    /// This allows us to reuse much of the linear logic.
    exponent_scale: LinearAxisSpace,
    /// The base of the logarithm.
    base: LogBase,
}

impl LogAxis {
    pub fn new(value_range: RangeInclusive<f64>, frame_range: Rangef, invert: bool, base: LogBase) -> Self {
        let exponent_min = base.exponent(*value_range.start()).unwrap_or(0.0);
        let exponent_max = base.exponent(*value_range.end()).unwrap_or(0.0);
        Self {
            exponent_scale: LinearAxisSpace::new(exponent_min..=exponent_max, frame_range, invert),
            base,
        }
    }
}

impl AxisSpace for LogAxis {
    fn value_min(&self) -> f64 {
        self.base.power(self.exponent_scale.value_min())
    }

    fn value_max(&self) -> f64 {
        self.base.power(self.exponent_scale.value_max())
    }

    fn frame_min(&self) -> f32 {
        self.exponent_scale.frame_min()
    }

    fn frame_max(&self) -> f32 {
        self.exponent_scale.frame_max()
    }

    fn set_inverted(&mut self, invert: bool) {
        self.exponent_scale.set_inverted(invert);
    }

    fn set_value_range(&mut self, range: RangeInclusive<f64>) {
        let exponent_min = self.base.exponent(*range.start()).unwrap_or(0.0);
        let exponent_max = self.base.exponent(*range.end()).unwrap_or(0.0);
        self.exponent_scale.set_value_range(exponent_min..=exponent_max);
    }

    fn position_from_value(&self, value: f64) -> f32 {
        let exponent = self.base.exponent(value).unwrap_or(0.0);
        self.exponent_scale.position_from_value(exponent)
    }

    fn value_from_position(&self, position: f32) -> f64 {
        let exponent = self.exponent_scale.value_from_position(position);
        self.base.power(exponent)
    }

    fn minimum_value_step(&self, spacing: f32) -> f64 {
        let linear_step = self.exponent_scale.minimum_value_step(spacing);
        let linear_min = self.exponent_scale.value_min();
        let log_min_step = self.base.power(linear_step + linear_min);
        log_min_step - self.value_min()
    }

    fn grid_input(&self, spacing: f32) -> GridInput {
       GridInput {
           bounds: (self.value_min(), self.value_max()),
           base_step_size: self.minimum_value_step(spacing),
       }
    }

    fn screen_distance_between_values(&self, value1: f64, value2: f64) -> f32 {
        let exponent1 = self.base.exponent(value1).unwrap_or(0.0);
        let exponent2 = self.base.exponent(value2).unwrap_or(0.0);
        self.exponent_scale.screen_distance_between_values(exponent1, exponent2)
    }

    fn translate(&mut self, frame_distance: f32) {
        self.exponent_scale.translate(frame_distance);
    }

    fn zoom(&mut self, factor: f32, center: f64) {
        if let Some(exponent) = self.base.exponent(center) {
            self.exponent_scale.zoom(factor, exponent);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertables::{assert_approx_eq, assert_in_delta};


    #[test]
    fn basic_log10_conversions() {
        let pairings = [[100.0, 2.0], [1000.0, 3.0], [0.1, -1.0]];
        for [value, exponent] in pairings {
            assert_approx_eq!(exponent, LogBase::Base10.exponent(value).unwrap());
            assert_approx_eq!(value, LogBase::Base10.power(exponent));
        }
        assert!(LogBase::Base10.exponent(0.0).is_none());
        assert!(LogBase::Base10.exponent(-1.0).is_none());
    }
    #[test]
    fn test_value_changes_map_to_linear_and_back() {
        let mut log_axis = LogAxis::new(1.0..=1000.0, Rangef::new(0.0, 1.0), false, LogBase::Base10);

        let value = 100.0;
        let position = log_axis.position_from_value(value);
        let value_back = log_axis.value_from_position(position);
        assert_in_delta!(value, value_back, 1e-4);
        assert_approx_eq!(position, 0.666666666);

        log_axis.set_value_range(10.0..=10000.0);
        let position = log_axis.position_from_value(value);
        let value_back = log_axis.value_from_position(position);
        assert_in_delta!(value, value_back, 1e-4);
        assert_approx_eq!(position, 0.333333333);
        assert_approx_eq!(log_axis.value_min(), 10.0);
        assert_approx_eq!(log_axis.value_max(), 10000.0);
        assert_approx_eq!(log_axis.frame_min(), 0.0);
        assert_approx_eq!(log_axis.frame_max(), 1.0);
    }
    #[test]
    fn test_value_changes_map_to_linear_and_back_base2() {
        let mut log_axis = LogAxis::new(1.0..=8.0, Rangef::new(0.0, 1.0), false, LogBase::Base2);

        let value = 4.0;
        let position = log_axis.position_from_value(value);
        let value_back = log_axis.value_from_position(position);
        assert_in_delta!(value, value_back, 1e-4);
        assert_approx_eq!(position, 0.666666666);

        log_axis.set_value_range(2.0..=16.0);
        let position = log_axis.position_from_value(value);
        let value_back = log_axis.value_from_position(position);
        assert_in_delta!(value, value_back, 1e-4);
        assert_in_delta!(position, 0.33333333, 1e-4);
        assert_approx_eq!(log_axis.value_min(), 2.0);
        assert_approx_eq!(log_axis.value_max(), 16.0);
        assert_approx_eq!(log_axis.frame_min(), 0.0);
        assert_approx_eq!(log_axis.frame_max(), 1.0);
    }
}
