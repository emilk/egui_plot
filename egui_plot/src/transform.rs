use egui::Pos2;
use egui::Rect;
use egui::Vec2;
use egui::Vec2b;
use egui::pos2;
use egui::remap;

use super::PlotPoint;
use crate::Axis;
use crate::bounds::PlotBounds;

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
