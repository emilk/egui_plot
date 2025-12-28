#![allow(dead_code)] // While in development, keep colormap private.
use egui::Color32;

/// Colormap is used to map some parameter (usually in the range [0.0, 1.0]) to [`egui::Color32`].
///
/// Currently, colors are interpolated from provided keypoints in sRGB space.
/// Keypoints may be uniformly distributed ([`Colormap::new_uniform`])
/// or have explicit positions ([`Colormap::new_with_positions`]).
///
/// If building your own Colormap with explicit positions, it is strongly recommended that
/// keypoints cover the full range from 0.0 to 1.0. This is because PlotItem consumers
/// make this assumption and will only sample the colormap in this range.
/// However, this is not enforced, so feel free to go against this for your own special use-cases.
pub struct Colormap {
    data: ColormapData,
}

impl Colormap {
    /// Create a new colormap from a list of colors, uniformly distributed from 0.0 to 1.0.
    ///
    /// If no colors are provided, defaults to Color32::TRANSPARENT.
    /// If a single color is provided, places the color at positions 0.0 and 1.0.
    pub fn new_uniform(colors: Vec<Color32>) -> Self {
        Self {
            data: ColormapData::new_uniform(colors),
        }
    }

    /// Create a new colormap from keypoints.
    ///
    /// If no keypoints are provided, defaults to Color32::TRANSPARENT.
    /// If a single keypoint is provided, places the color at positions 0.0 and 1.0.
    ///
    /// PlotItem consumers will expect that keypoints cover the full range from 0.0 to 1.0,
    /// but this is not enforced. Feel free to go against this for your own special use-cases,
    /// but be aware that PlotItems will only consider the [0.0, 1.0] range.
    pub fn new_with_positions(keypoints: Vec<(f32, Color32)>) -> Self {
        Self {
            data: ColormapData::new_nonuniform(keypoints),
        }
    }

    /// Get color at a given position.
    pub fn get(&self, t: f32) -> Color32 {
        self.data.get(t, ColorInterpolation::SRGB)
    }
}

// Internal representation of colormap data.
// May support things like functions in the future.
enum ColormapData {
    Uniform(UniformKeypoints),
    Nonuniform(NonuniformKeypoints),
}

// Keypoints are uniformly distributed between 0.0 and 1.0
struct UniformKeypoints {
    colors: Vec<Color32>,
}

// Keypoints have explicit positions. Consumers expect positions to be in [0.0, 1.0],
// but this is not enforced.
struct NonuniformKeypoints {
    keypoints: Vec<(f32, Color32)>,
}

impl ColormapData {
    // If no keypoints are provided, defaults to Color32::TRANSPARENT.
    // If a single keypoint is provided, it is duplicated.
    fn new_uniform(colors: Vec<Color32>) -> Self {
        if colors.is_empty() {
            log::warn!("Colormap created with no colors, defaulting to transparent");
            return Self::Uniform(UniformKeypoints {
                colors: vec![Color32::TRANSPARENT; 2],
            });
        }

        if colors.len() == 1 {
            log::warn!("Colormap created with a single color, duplicating it for full range");
            let single = colors[0];
            return Self::Uniform(UniformKeypoints {
                colors: vec![single; 2],
            });
        }

        Self::Uniform(UniformKeypoints { colors })
    }

    // If no keypoints are provided, defaults to Color32::TRANSPARENT.
    // If a single keypoint is provided, it is duplicated at positions 0.0 and 1.0.
    fn new_nonuniform(mut keypoints: Vec<(f32, Color32)>) -> Self {
        if keypoints.is_empty() {
            log::warn!("Colormap created with no keypoints, defaulting to transparent");
            return Self::Nonuniform(NonuniformKeypoints {
                keypoints: vec![(0.0, Color32::TRANSPARENT), (1.0, Color32::TRANSPARENT)],
            });
        }

        if keypoints.len() == 1 {
            log::warn!("Colormap created with a single keypoint, duplicating it for full range");
            let single = keypoints[0];
            return Self::Nonuniform(NonuniformKeypoints {
                keypoints: vec![(0.0, single.1), (1.0, single.1)],
            });
        }

        // Ensure keypoints are sorted by position
        keypoints.sort_by(|a, b| a.0.total_cmp(&b.0));

        if let Some(&(first_pos, _)) = keypoints.first()
            && first_pos != 0.0
        {
            log::warn!("Colormap keypoints start at {}, instead of 0.0", first_pos);
        }
        if let Some(&(last_pos, _)) = keypoints.last()
            && last_pos != 1.0
        {
            log::warn!("Colormap keypoints end at {}, instead of 1.0", last_pos);
        }
        Self::Nonuniform(NonuniformKeypoints { keypoints })
    }

    fn get(&self, t: f32, interpolate: impl Fn(&Color32, &Color32, f32) -> Color32) -> Color32 {
        match self {
            ColormapData::Uniform(uniform) => uniform.get(t, interpolate),
            ColormapData::Nonuniform(nonuniform) => nonuniform.get(t, interpolate),
        }
    }
}

impl UniformKeypoints {
    // Maps t in [0.0, 1.0] to colors uniformly distributed in the underlying colors array.
    // Result is interpolated using the provided interpolation function.
    fn get(&self, t: f32, interpolate: impl Fn(&Color32, &Color32, f32) -> Color32) -> Color32 {
        let n = self.colors.len();
        if n == 0 {
            return Color32::TRANSPARENT;
        }
        if n == 1 {
            return self.colors[0];
        }

        // Map t from [0.0, 1.0] to a pair of sequential indexes in the colors array
        let t = t.clamp(0.0, 1.0);
        let scaled_t = t * (n - 1) as f32;
        let index0 = scaled_t.floor() as usize;
        let index1 = scaled_t.ceil() as usize;

        // Occurs when t is exactly on a keypoint
        if index0 == index1 {
            return self.colors[index0];
        }

        // Interpolate between the two colors
        let color0 = self.colors[index0];
        let color1 = self.colors[index1];
        interpolate(&color0, &color1, scaled_t - index0 as f32)
    }
}

impl NonuniformKeypoints {
    // Maps t to a pair of sequential keypoints in the underlying keypoints array.
    // Result is interpolated using the provided interpolation function.
    fn get(&self, t: f32, interpolate: impl Fn(&Color32, &Color32, f32) -> Color32) -> Color32 {
        let Some(first) = self.keypoints.first() else {
            return Color32::TRANSPARENT;
        };
        let Some(last) = self.keypoints.last() else {
            return Color32::TRANSPARENT;
        };

        if t <= first.0 {
            return first.1;
        }
        if t >= last.0 {
            return last.1;
        }

        for i in 1..self.keypoints.len() {
            let (t1, color1) = self.keypoints[i];
            // Find the first keypoint where t <= t1. It is implied that t0 <= t.
            if t <= t1 {
                let (t0, color0) = self.keypoints[i - 1];
                return interpolate(&color0, &color1, (t - t0) / (t1 - t0));
            }
        }

        log::warn!("Colormap get reached unexpected code path, returning last color");
        last.1
    }
}

/// Different methods for interpolating between colors.
///
/// For those uninitiated in color interpolation, see
/// [this blog post](https://raphlinus.github.io/color/2021/01/18/oklab-critique.html)
/// for a great visual review of the behavior of different interpolation methods.
pub struct ColorInterpolation;

impl ColorInterpolation {
    /// Simple linear interpolation in sRGB space.
    /// It is cheap to compute and works well for nearby colors, but can produce
    /// perceptibly unexpected results for distant colors.
    pub const SRGB: fn(&Color32, &Color32, f32) -> Color32 = srgb_interpolate;
}

fn srgb_interpolate(c0: &Color32, c1: &Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let s = 1.0 - t;
    let r = (c0.r() as f32 * s + c1.r() as f32 * t).round() as u8;
    let g = (c0.g() as f32 * s + c1.g() as f32 * t).round() as u8;
    let b = (c0.b() as f32 * s + c1.b() as f32 * t).round() as u8;
    let a = (c0.a() as f32 * s + c1.a() as f32 * t).round() as u8;
    Color32::from_rgba_premultiplied(r, g, b, a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colormap_uniform() {
        let colors = vec![
            Color32::from_rgb(255, 0, 0),
            Color32::from_rgb(0, 255, 0),
            Color32::from_rgb(0, 0, 255),
        ];
        let colormap = Colormap::new_uniform(colors);
        assert_eq!(colormap.get(0.0), Color32::from_rgb(255, 0, 0));
        assert_eq!(colormap.get(0.25), Color32::from_rgb(128, 128, 0));
        assert_eq!(colormap.get(0.5), Color32::from_rgb(0, 255, 0));
        assert_eq!(colormap.get(0.75), Color32::from_rgb(0, 128, 128));
        assert_eq!(colormap.get(1.0), Color32::from_rgb(0, 0, 255));

        // Out of bounds
        assert_eq!(colormap.get(-0.5), Color32::from_rgb(255, 0, 0));
        assert_eq!(colormap.get(1.5), Color32::from_rgb(0, 0, 255));
    }

    #[test]
    fn colormap_uniform_empty() {
        let colors = vec![];
        let colormap = Colormap::new_uniform(colors);
        for t in [-0.5, 0.0, 0.25, 0.5, 0.75, 1.0, 1.5] {
            assert_eq!(colormap.get(t), Color32::TRANSPARENT);
        }
    }

    #[test]
    fn colormap_uniform_single() {
        let colors = vec![Color32::from_rgb(255, 0, 0)];
        let colormap = Colormap::new_uniform(colors);
        for t in [-0.5, 0.0, 0.25, 0.5, 0.75, 1.0, 1.5] {
            assert_eq!(colormap.get(t), Color32::from_rgb(255, 0, 0));
        }
    }

    #[test]
    fn colormap_nonuniform() {
        let colors = vec![
            (0.0, Color32::from_rgb(254, 0, 0)),
            (0.4, Color32::from_rgb(0, 254, 0)),
            (1.0, Color32::from_rgb(0, 0, 254)),
        ];
        let colormap = Colormap::new_with_positions(colors);
        assert_eq!(colormap.get(0.0), Color32::from_rgb(254, 0, 0));
        assert_eq!(colormap.get(0.2), Color32::from_rgb(127, 127, 0));
        assert_eq!(colormap.get(0.4), Color32::from_rgb(0, 254, 0));
        assert_eq!(colormap.get(0.7), Color32::from_rgb(0, 127, 127));
        assert_eq!(colormap.get(1.0), Color32::from_rgb(0, 0, 254));

        // Out of bounds
        assert_eq!(colormap.get(-0.5), Color32::from_rgb(254, 0, 0));
        assert_eq!(colormap.get(1.5), Color32::from_rgb(0, 0, 254));
    }

    #[test]
    fn colormap_nonuniform_empty() {
        let colors = vec![];
        let colormap = Colormap::new_with_positions(colors);
        for t in [-0.5, 0.0, 0.25, 0.5, 0.75, 1.0, 1.5] {
            assert_eq!(colormap.get(t), Color32::TRANSPARENT);
        }
    }

    #[test]
    fn colormap_nonuniform_single() {
        let colors = vec![(0.3, Color32::from_rgb(0, 255, 0))];
        let colormap = Colormap::new_with_positions(colors);
        for t in [-0.5, 0.0, 0.25, 0.5, 0.75, 1.0, 1.5] {
            assert_eq!(colormap.get(t), Color32::from_rgb(0, 255, 0));
        }
    }
}
