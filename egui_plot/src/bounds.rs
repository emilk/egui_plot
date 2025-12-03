use std::ops::RangeInclusive;

use ahash::HashMap;
use egui::Id;
use emath::Vec2;
use emath::Vec2b;

use crate::PlotPoint;

/// 2D bounding box of f64 precision.
///
/// The range of data values we show.
#[derive(Clone, Copy, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct PlotBounds {
    pub(crate) min: [f64; 2],
    pub(crate) max: [f64; 2],
}

impl PlotBounds {
    pub const NOTHING: Self = Self {
        min: [f64::INFINITY; 2],
        max: [-f64::INFINITY; 2],
    };

    #[inline]
    pub fn from_min_max(min: [f64; 2], max: [f64; 2]) -> Self {
        Self { min, max }
    }

    #[inline]
    pub fn min(&self) -> [f64; 2] {
        self.min
    }

    #[inline]
    pub fn max(&self) -> [f64; 2] {
        self.max
    }

    #[inline]
    pub fn new_symmetrical(half_extent: f64) -> Self {
        Self {
            min: [-half_extent; 2],
            max: [half_extent; 2],
        }
    }

    #[inline]
    pub fn is_finite(&self) -> bool {
        self.min[0].is_finite() && self.min[1].is_finite() && self.max[0].is_finite() && self.max[1].is_finite()
    }

    #[inline]
    pub fn is_finite_x(&self) -> bool {
        self.min[0].is_finite() && self.max[0].is_finite()
    }

    #[inline]
    pub fn is_finite_y(&self) -> bool {
        self.min[1].is_finite() && self.max[1].is_finite()
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        self.is_finite() && self.width() > 0.0 && self.height() > 0.0
    }

    #[inline]
    pub fn is_valid_x(&self) -> bool {
        self.is_finite_x() && self.width() > 0.0
    }

    #[inline]
    pub fn is_valid_y(&self) -> bool {
        self.is_finite_y() && self.height() > 0.0
    }

    #[inline]
    pub fn width(&self) -> f64 {
        self.max[0] - self.min[0]
    }

    #[inline]
    pub fn height(&self) -> f64 {
        self.max[1] - self.min[1]
    }

    #[inline]
    pub fn center(&self) -> PlotPoint {
        [
            emath::fast_midpoint(self.min[0], self.max[0]),
            emath::fast_midpoint(self.min[1], self.max[1]),
        ]
        .into()
    }

    /// Expand to include the given (x,y) value
    #[inline]
    pub fn extend_with(&mut self, value: &PlotPoint) {
        self.extend_with_x(value.x);
        self.extend_with_y(value.y);
    }

    /// Expand to include the given x coordinate
    #[inline]
    pub fn extend_with_x(&mut self, x: f64) {
        self.min[0] = self.min[0].min(x);
        self.max[0] = self.max[0].max(x);
    }

    /// Expand to include the given y coordinate
    #[inline]
    pub fn extend_with_y(&mut self, y: f64) {
        self.min[1] = self.min[1].min(y);
        self.max[1] = self.max[1].max(y);
    }

    #[inline]
    fn clamp_to_finite(&mut self) {
        for d in 0..2 {
            self.min[d] = self.min[d].clamp(f64::MIN, f64::MAX);
            if self.min[d].is_nan() {
                self.min[d] = 0.0;
            }

            self.max[d] = self.max[d].clamp(f64::MIN, f64::MAX);
            if self.max[d].is_nan() {
                self.max[d] = 0.0;
            }
        }
    }

    #[inline]
    pub fn expand_x(&mut self, pad: f64) {
        if pad.is_finite() {
            self.min[0] -= pad;
            self.max[0] += pad;
            self.clamp_to_finite();
        }
    }

    #[inline]
    pub fn expand_y(&mut self, pad: f64) {
        if pad.is_finite() {
            self.min[1] -= pad;
            self.max[1] += pad;
            self.clamp_to_finite();
        }
    }

    #[inline]
    pub fn merge_x(&mut self, other: &Self) {
        self.min[0] = self.min[0].min(other.min[0]);
        self.max[0] = self.max[0].max(other.max[0]);
    }

    #[inline]
    pub fn merge_y(&mut self, other: &Self) {
        self.min[1] = self.min[1].min(other.min[1]);
        self.max[1] = self.max[1].max(other.max[1]);
    }

    #[inline]
    pub fn set_x(&mut self, other: &Self) {
        self.min[0] = other.min[0];
        self.max[0] = other.max[0];
    }

    #[inline]
    pub fn set_x_center_width(&mut self, x: f64, width: f64) {
        self.min[0] = x - width / 2.0;
        self.max[0] = x + width / 2.0;
    }

    #[inline]
    pub fn set_y(&mut self, other: &Self) {
        self.min[1] = other.min[1];
        self.max[1] = other.max[1];
    }

    #[inline]
    pub fn set_y_center_height(&mut self, y: f64, height: f64) {
        self.min[1] = y - height / 2.0;
        self.max[1] = y + height / 2.0;
    }

    #[inline]
    pub fn merge(&mut self, other: &Self) {
        self.min[0] = self.min[0].min(other.min[0]);
        self.min[1] = self.min[1].min(other.min[1]);
        self.max[0] = self.max[0].max(other.max[0]);
        self.max[1] = self.max[1].max(other.max[1]);
    }

    #[inline]
    pub fn translate_x(&mut self, delta: f64) {
        if delta.is_finite() {
            self.min[0] += delta;
            self.max[0] += delta;
            self.clamp_to_finite();
        }
    }

    #[inline]
    pub fn translate_y(&mut self, delta: f64) {
        if delta.is_finite() {
            self.min[1] += delta;
            self.max[1] += delta;
            self.clamp_to_finite();
        }
    }

    #[inline]
    pub fn translate(&mut self, delta: (f64, f64)) {
        self.translate_x(delta.0);
        self.translate_y(delta.1);
    }

    #[inline]
    pub fn zoom(&mut self, zoom_factor: Vec2, center: PlotPoint) {
        self.min[0] = center.x + (self.min[0] - center.x) / (zoom_factor.x as f64);
        self.max[0] = center.x + (self.max[0] - center.x) / (zoom_factor.x as f64);
        self.min[1] = center.y + (self.min[1] - center.y) / (zoom_factor.y as f64);
        self.max[1] = center.y + (self.max[1] - center.y) / (zoom_factor.y as f64);
    }

    #[inline]
    pub fn add_relative_margin_x(&mut self, margin_fraction: Vec2) {
        let width = self.width().max(0.0);
        self.expand_x(margin_fraction.x as f64 * width);
    }

    #[inline]
    pub fn add_relative_margin_y(&mut self, margin_fraction: Vec2) {
        let height = self.height().max(0.0);
        self.expand_y(margin_fraction.y as f64 * height);
    }

    #[inline]
    pub fn range_x(&self) -> RangeInclusive<f64> {
        self.min[0]..=self.max[0]
    }

    #[inline]
    pub fn range_y(&self) -> RangeInclusive<f64> {
        self.min[1]..=self.max[1]
    }

    #[inline]
    pub fn make_x_symmetrical(&mut self) {
        let x_abs = self.min[0].abs().max(self.max[0].abs());
        self.min[0] = -x_abs;
        self.max[0] = x_abs;
    }

    #[inline]
    pub fn make_y_symmetrical(&mut self) {
        let y_abs = self.min[1].abs().max(self.max[1].abs());
        self.min[1] = -y_abs;
        self.max[1] = y_abs;
    }
}

#[derive(Clone)]
pub struct LinkedBounds {
    pub bounds: PlotBounds,
    pub auto_bounds: Vec2b,
}

#[derive(Default, Clone)]
pub struct BoundsLinkGroups(pub HashMap<Id, LinkedBounds>);

/// User-requested modifications to the plot bounds. We collect them in the plot
/// build function to later apply them at the right time, as other modifications
/// need to happen first.
pub enum BoundsModification {
    SetX(RangeInclusive<f64>),
    SetY(RangeInclusive<f64>),
    Translate(Vec2),
    AutoBounds(Vec2b),
    Zoom(Vec2, PlotPoint),
}
