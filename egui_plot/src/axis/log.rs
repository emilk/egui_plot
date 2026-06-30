use std::ops::RangeInclusive;
use emath::Rangef;
use crate::axis::linear::LinearAxisSpace;
use crate::axis::transform::AxisSpace;

enum LogBase {
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
            LogBase::Base10 => Some(value.log10()),
            LogBase::Base2 => Some(value.log2()),
        }
    }

    pub fn power(&self, value: f64) -> f64 {
        match self {
            LogBase::Base10 => f64::powf(10.0, value),
            LogBase::Base2 => value.exp2(),
        }
    }
}

struct LogAxis {
    /// A linear scale for the exponent of the logarithm.
    ///
    /// This allows us to reuse much of the linear logic.
    exponent_scale: LinearAxisSpace,
    /// The base of the logarithm.
    base: LogBase
}

impl LogAxis {
    pub fn new(value_range: RangeInclusive<f64>, frame_range: Rangef, invert: bool, base: LogBase) -> Self {
        let exponent_min = base.exponent(*value_range.start()).unwrap();
        let exponent_max = base.exponent(*value_range.end()).unwrap();
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
        let exponent_min = self.base.exponent(*range.start()).unwrap();
        let exponent_max = self.base.exponent(*range.end()).unwrap();
        self.exponent_scale.set_value_range(exponent_min..=exponent_max);
    }

    fn position_from_value(&self, value: f64) -> f32 {
        let exponent = self.base.exponent(value).unwrap();
        self.exponent_scale.position_from_value(exponent)
    }

    fn value_from_position(&self, position: f32) -> f64 {
        let exponent = self.exponent_scale.value_from_position(position);
        self.base.power(exponent)
    }

    fn minimum_value_step(&self, spacing: f32) -> f64 {
        todo!()
    }

    fn translate(&mut self, frame_distance: f32) {
        todo!()
    }

    fn zoom(&mut self, factor: f32, center: f64) {
        if let Some(exponent) = self.base.exponent(center) {
            self.exponent_scale.zoom(factor, exponent);
        }
    }

}

#[cfg(test)]
mod tests {
    use assertables::{assert_approx_eq, assert_in_delta};
    use super::*;

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
