use crate::grid::{GridInput, GridMark, generate_marks_linear, uniform_grid_spacer};

/// Defines how data coordinates are transformed to plot coordinates.
///
/// This enables non-linear scales like logarithmic axes.
/// The transformation is one-dimensional (operates on a single axis).
pub trait AxisTransform: Send + Sync + std::fmt::Debug {
    /// Transform a data value to plot space.
    ///
    /// Plot space is a linear coordinate system that gets mapped to screen space.
    fn transform_to_plot(&self, data_value: f64) -> f64;

    /// Transform a plot space value back to data space.
    fn transform_from_plot(&self, plot_value: f64) -> f64;

    /// Returns the bounds in plot space for the given data space bounds.
    ///
    /// This is not simply `(to_plot(min), to_plot(max))` because for some transforms
    /// (like log), the order might flip or we need special handling.
    fn bounds_to_plot(&self, data_min: f64, data_max: f64) -> (f64, f64) {
        (self.transform_to_plot(data_min), self.transform_to_plot(data_max))
    }

    /// Generate grid marks for this transform.
    ///
    /// The input bounds are in data space, and returned marks are also in data space.
    fn generate_marks(&self, input: GridInput) -> Vec<GridMark>;

    /// Range of valid values in data space.
    ///
    /// For example, logarithmic scales are only valid for positive values.
    /// Returns `None` if all values are valid.
    fn valid_range(&self) -> Option<(f64, f64)> {
        None
    }

    /// Returns a boxed clone of this transform.
    fn boxed_clone(&self) -> Box<dyn AxisTransform>;

    /// Zoom the bounds by a factor around a center point.
    ///
    /// All values are in data space.
    /// Returns the new (min, max) bounds.
    fn zoom_bounds(&self, min: f64, max: f64, zoom_factor: f64, center: f64) -> (f64, f64);

    /// Pan the bounds by a delta in screen space.
    ///
    /// # Arguments
    ///
    /// - `min`, `max`: current bounds in data space
    /// - `delta_pixels`: how many pixels to pan
    /// - `dvalue_dpos`: how much plot space per pixel (from transform)
    ///
    /// # Returns
    ///  - The new (min, max) bounds in data space.
    fn pan_bounds(&self, min: f64, max: f64, delta_pixels: f64, dvalue_dpos: f64) -> (f64, f64);
}

impl Clone for Box<dyn AxisTransform> {
    fn clone(&self) -> Self {
        self.boxed_clone()
    }
}

/// Linear axis transform (identity transform).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinearAxisTransform;

impl AxisTransform for LinearAxisTransform {
    #[inline]
    fn transform_to_plot(&self, data_value: f64) -> f64 {
        data_value
    }

    #[inline]
    fn transform_from_plot(&self, plot_value: f64) -> f64 {
        plot_value
    }

    fn generate_marks(&self, input: GridInput) -> Vec<GridMark> {
        // Use the default uniform grid spacer
        let spacer = uniform_grid_spacer(|grid_input| {
            let step_size = grid_input.base_step_size;
            [step_size, step_size * 2.5, step_size * 10.0]
        });
        spacer(input)
    }

    fn boxed_clone(&self) -> Box<dyn AxisTransform> {
        Box::new(*self)
    }

    fn zoom_bounds(&self, min: f64, max: f64, zoom_factor: f64, center: f64) -> (f64, f64) {
        // Linear zoom: standard formula
        let new_min = center + (min - center) / zoom_factor;
        let new_max = center + (max - center) / zoom_factor;
        (new_min, new_max)
    }

    fn pan_bounds(&self, min: f64, max: f64, delta_pixels: f64, dvalue_dpos: f64) -> (f64, f64) {
        // Linear pan: translate directly in data space
        let delta_data = delta_pixels * dvalue_dpos;
        (min + delta_data, max + delta_data)
    }
}

/// Logarithmic axis transform (base 10).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogAxisTransform;

impl LogAxisTransform {
    // Minimum positive value that is used to clamp the view
    const MIN_POSITIVE: f64 = 1e-200;
    // Maximum positive value that is used to clamp the view
    const MAX_POSITIVE: f64 = 1e200;

    const BASE: f64 = 10.0;
}

impl AxisTransform for LogAxisTransform {
    fn transform_to_plot(&self, data_value: f64) -> f64 {
        if data_value <= 0.0 {
            // Handle invalid values gracefully
            f64::NEG_INFINITY
        } else {
            data_value.log10()
        }
    }

    fn transform_from_plot(&self, plot_value: f64) -> f64 {
        Self::BASE.powf(plot_value)
    }

    fn bounds_to_plot(&self, data_min: f64, data_max: f64) -> (f64, f64) {
        // For log scale, we need positive values

        let safe_min = data_min.clamp(Self::MIN_POSITIVE, Self::MAX_POSITIVE / 1.01);
        let safe_max = data_max.clamp(safe_min * 1.01, Self::MAX_POSITIVE);

        (self.transform_to_plot(safe_min), self.transform_to_plot(safe_max))
    }

    fn generate_marks(&self, input: GridInput) -> Vec<GridMark> {
        // For logarithmic scale, generate marks at powers of the base
        let (data_min, data_max) = input.bounds;

        // If the data range is invalid, return an empty mark list
        if data_min <= 0.0 || data_max <= 0.0 || data_min >= data_max {
            return Vec::new();
        }

        // Create some space for marks.
        let mut marks = Vec::with_capacity(10);

        // Calculate the actual logarithmic range (in decades)
        let log_range = (data_max / data_min).log10();

        // Special case: VERY tight zoom (< 0.05 decades, i.e., range is < 1.12x)
        // Use adaptive linear spacing to ensure we always have enough marks
        if log_range < 0.05 {
            let data_range = data_max - data_min;

            // Find the major power in or near the range
            let mid_point = f64::midpoint(data_min, data_max);
            let major_exp = mid_point.log10().round() as i32;
            let major_value = Self::BASE.powi(major_exp);

            // Add the major power if it's visible
            if major_value >= data_min && major_value <= data_max {
                marks.push(GridMark {
                    value: major_value,
                    step_size: Self::BASE,
                });
            }

            // Calculate adaptive step size to get ~10 marks across the range
            // Use a "nice" step size based on order of magnitude
            let target_step = data_range / 10.0;
            let magnitude = 10_f64.powi(target_step.log10().floor() as i32);

            // Choose step from nice values: 1, 2, 5 times the magnitude
            let step = if target_step >= magnitude * 5.0 {
                magnitude * 5.0
            } else if target_step >= magnitude * 2.0 {
                magnitude * 2.0
            } else {
                magnitude
            };

            // Generate marks using the three-tier system: step, step*2.5, step*10
            let step_sizes = [step, step * 2.5, step * 10.0];

            // Use the standard linear mark generator
            marks.extend(generate_marks_linear(step_sizes, (data_min, data_max)));

            return marks;
        }

        // Find the range of exponents we need to cover
        let min_exp = (data_min.log10()).floor() as i32;
        let max_exp = (data_max.log10()).ceil() as i32;

        // Strategy: Always show major powers, add intermediate marks based on
        // available space step_size indicates mark importance for rendering
        // (larger = more important)

        // ALWAYS show major powers (1, 10, 100, 1000, ...) - these are essential
        for exp in min_exp..=max_exp {
            let value = Self::BASE.powi(exp);
            if value >= data_min && value <= data_max {
                marks.push(GridMark {
                    value,
                    step_size: Self::BASE, // Major power
                });
            }
        }

        // Add intermediate marks based on available space

        // Tier 3: Show 2× and 5× marks (but only if BOTH fit)
        // The tightest spacing is between 2 and 5: log10(5/2) ≈ 0.398
        let tier_3_min_spacing = (5.0_f64 / 2.0_f64).log10();
        let include_tier_3 = tier_3_min_spacing >= input.base_step_size * 0.5; // More permissive!

        // Tier 4: Show additional subdivisions 3,4,6,7,8,9 when VERY zoomed in
        // Tightest spacing is between 1 and 2: log10(2) ≈ 0.301
        let tier_4_min_spacing = (2.0_f64).log10();
        let include_tier_4 = tier_4_min_spacing >= input.base_step_size * 0.2;

        // Always add tier 3 (2, 5) if there's room
        if include_tier_3 {
            // Decide if these should be labeled or just grid lines
            // When showing many decades (> 3), don't label tier 3, just show as grid
            let num_decades = max_exp - min_exp;
            let tier_3_step_size = if num_decades > 3 {
                0.5 // Too many decades - show as grid only, no label
            } else {
                1.0 // Few decades - show with labels
            };

            // Extend range by 1 in both directions to catch marks near boundaries
            for exp in (min_exp - 1)..=(max_exp + 1) {
                let major_value = Self::BASE.powi(exp);
                let next_major = major_value * Self::BASE;

                // Standard practice: only show 2× and 5× multiples
                for &multiplier in &[2.0, 5.0] {
                    let value = major_value * multiplier;

                    if value > data_min && value < data_max && value < next_major {
                        marks.push(GridMark {
                            value,
                            step_size: tier_3_step_size,
                        });
                    }
                }
            }

            // Tier 4: Add the ADDITIONAL marks (3,4,6,7,8,9) - not including 2,5 which are in tier 3
            if include_tier_4 {
                // Extend range by 1 in both directions to catch marks near boundaries
                for exp in (min_exp - 1)..=(max_exp + 1) {
                    let major_value = Self::BASE.powi(exp);
                    let next_major = major_value * Self::BASE;

                    // Add 3, 4, 6, 7, 8, 9 (NOT 2 and 5, those are tier 3)
                    for multiplier in [3, 4, 6, 7, 8, 9] {
                        let value = major_value * (multiplier as f64);

                        if value > data_min && value < data_max && value < next_major {
                            marks.push(GridMark {
                                value,
                                step_size: 0.5, // Fine subdivisions, grid only
                            });
                        }
                    }
                }
            }
        }

        marks
    }

    fn valid_range(&self) -> Option<(f64, f64)> {
        // Log scale only valid for positive values
        Some((0f64.next_up(), f64::INFINITY))
    }

    fn boxed_clone(&self) -> Box<dyn AxisTransform> {
        Box::new(*self)
    }

    fn zoom_bounds(&self, min: f64, max: f64, zoom_factor: f64, center: f64) -> (f64, f64) {
        // For log scales, zoom multiplicatively in data space
        // This makes zoom feel natural regardless of the magnitude

        // Clamp to valid range for log scale
        let min = min.max(Self::MIN_POSITIVE);
        let max = max.max(min * 1.01);
        let center = center.clamp(min, max);

        // Calculate ratios from center
        let ratio_min = min / center;
        let ratio_max = max / center;

        // Apply zoom by taking the ratio to a power
        // zoom_factor > 1 means zoom in, < 1 means zoom out
        let power = 1.0 / zoom_factor;
        let new_min = center * ratio_min.powf(power);
        let new_max = center * ratio_max.powf(power);

        (new_min, new_max)
    }

    fn pan_bounds(&self, min: f64, max: f64, delta_pixels: f64, dvalue_dpos: f64) -> (f64, f64) {
        // For log scales, pan in plot space then convert back
        // This ensures consistent pan speed across magnitudes

        // Convert to plot space
        let plot_min = self.transform_to_plot(min.max(Self::MIN_POSITIVE));
        let plot_max = self.transform_to_plot(max.max(Self::MIN_POSITIVE));

        // Pan in plot space
        let delta_plot = delta_pixels * dvalue_dpos;
        let new_plot_min = plot_min + delta_plot;
        let new_plot_max = plot_max + delta_plot;

        // Convert back to data space
        let new_min = self.transform_from_plot(new_plot_min);
        let new_max = self.transform_from_plot(new_plot_max);

        (new_min, new_max)
    }
}

/// Default axis transform configuration for an axis.
#[derive(Clone, Copy, Default, Debug)]
pub enum AxisTransformKind {
    /// Linear scale (default).
    #[default]
    Linear,
    /// Logarithmic scale.
    Log,
}

impl AxisTransformKind {
    /// Create the actual transform implementation.
    pub fn make_transform(&self) -> Box<dyn AxisTransform> {
        match self {
            Self::Linear => Box::new(LinearAxisTransform),
            Self::Log => Box::new(LogAxisTransform),
        }
    }
}
