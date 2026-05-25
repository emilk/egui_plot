use std::ops::RangeInclusive;
use emath::{remap, Rangef};
use crate::axis::transform::AxisSpace;

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
            invert
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

}

impl AxisSpace for LinearAxisSpace {
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


    fn position_from_value(&self, value: f64) -> f32 {
        remap(
            value,
            self.min..=self.max,
            self.frame_range_f64()
        ) as f32
    }

    fn value_from_position(&self, position: f32) -> f64 {
        remap(position as f64, self.frame_range_f64(), self.min..=self.max)
    }

    fn dvalue_per_dpos(&self) -> f64 {
        let frame_range = self.frame_range_f64();
        self.value_length() / (frame_range.end() - frame_range.start())
    }

    fn dpos_per_dvalue(&self) -> f32 {
        let frame_range = self.frame_range();
        (frame_range.end() - frame_range.start()) / (self.value_length() as f32)
    }

    fn expand(&mut self, pad: f64) {
        if pad.is_finite() {
            self.min -= pad;
            self.max += pad;
            self.clamp_to_finite();
        }
    }

    fn set_inverted(&mut self, invert: bool) {
        self.invert = invert;
    }

    fn set_value_range(&mut self, range: RangeInclusive<f64>) {
        self.min = *range.start();
        self.max = *range.end();
        self.clamp_to_finite();
    }
}