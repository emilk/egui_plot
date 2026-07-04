use crate::GridInput;
use crate::axis::transform::AxisSpace;
use emath::{Rangef, remap};
use std::ops::RangeInclusive;

#[derive(Debug, Copy, Clone)]
pub struct LinearAxisSpace {
    min: f64,
    max: f64,
    invert: bool,
    frame_start: f32,
    frame_end: f32,
}

impl LinearAxisSpace {
    pub fn new(value_range: RangeInclusive<f64>, frame_range: Rangef, invert: bool) -> Self {
        Self {
            min: *value_range.start(),
            max: *value_range.end(),
            frame_start: frame_range.min,
            frame_end: frame_range.max,
            invert,
        }
    }
    fn clamp_to_finite(&mut self) {
        self.min = self.min.clamp(f64::MIN, f64::MAX);
        if self.min.is_nan() {
            self.min = 0.0;
        }

        self.max = self.max.clamp(f64::MIN, f64::MAX);
        if self.max.is_nan() {
            self.max = 0.0;
        }
    }

    /// Specifies the output range of the space, inverting it
    /// if inversion is selected.
    fn frame_range(&self) -> RangeInclusive<f32> {
        if self.invert {
            (self.frame_end)..=(self.frame_start)
        } else {
            (self.frame_start)..=(self.frame_end)
        }
    }

    /// Get the frame range as f64 format. This is not the natural
    /// format for screen units but is needed for remapping to and
    /// from values.
    fn frame_range_f64(&self) -> RangeInclusive<f64> {
        let as_f32 = self.frame_range();
        *as_f32.start() as f64..=*as_f32.end() as f64
    }

    fn dvalue_per_dpos(&self) -> f64 {
        let frame_range = self.frame_range_f64();
        self.value_length() / (frame_range.end() - frame_range.start())
    }
}

impl AxisSpace for LinearAxisSpace {
    fn value_min(&self) -> f64 {
        self.min
    }

    fn value_max(&self) -> f64 {
        self.max
    }

    fn frame_min(&self) -> f32 {
        self.frame_start
    }

    fn frame_max(&self) -> f32 {
        self.frame_end
    }

    fn set_inverted(&mut self, invert: bool) {
        self.invert = invert;
    }

    fn set_value_range(&mut self, range: RangeInclusive<f64>) {
        self.min = *range.start();
        self.max = *range.end();
        self.clamp_to_finite();
    }

    fn position_from_value(&self, value: f64) -> f32 {
        remap(value, self.min..=self.max, self.frame_range_f64()) as f32
    }

    fn value_from_position(&self, position: f32) -> f64 {
        remap(position as f64, self.frame_range_f64(), self.min..=self.max)
    }

    fn position_delta_from_screen_delta(&self, _start_position: f64, drag_delta: f32) -> f64 {
        drag_delta as f64 * self.dvalue_per_dpos()
    }

    fn grid_input(&self, spacing: f32) -> GridInput {
        GridInput {
            bounds: (self.min, self.max),
            base_step_size: self.dvalue_per_dpos().abs() * (spacing as f64),
        }
    }

    fn screen_distance_between_values(&self, value1: f64, value2: f64) -> f32 {
        let delta = value2 - value1;
        (delta / self.dvalue_per_dpos()) as f32
    }

    fn translate(&mut self, frame_distance: f32) {
        let dvalue_per_dpos = self.dvalue_per_dpos();
        let value_translation = frame_distance as f64 * dvalue_per_dpos;
        self.min += value_translation;
        self.max += value_translation;
        self.clamp_to_finite();
    }

    fn zoom(&mut self, zoom_factor: f32, center: f64) {
        self.min = center + (self.min - center) / (zoom_factor as f64);
        self.max = center + (self.max - center) / (zoom_factor as f64);
    }
}
