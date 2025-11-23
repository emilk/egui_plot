//! Simple plotting library for [`egui`](https://github.com/emilk/egui).
//!
//! Check out [`Plot`] for how to get started.
//!
//! [**Looking for maintainer!**](https://github.com/emilk/egui/issues/4705)
//!
//! ## Feature flags
#![cfg_attr(feature = "document-features", doc = document_features::document_features!())]
//!

mod axis;
mod items;
mod legend;
mod memory;
mod plot;
mod plot_ui;
mod transform;

use std::cmp::Ordering;
use std::ops::RangeInclusive;

use ahash::HashMap;
use egui::Color32;
use egui::Id;
use egui::NumExt as _;
use egui::Response;
use egui::Ui;
use egui::Vec2;
use egui::Vec2b;

pub use crate::axis::Axis;
pub use crate::axis::AxisHints;
pub use crate::axis::HPlacement;
pub use crate::axis::Placement;
pub use crate::axis::VPlacement;
pub use crate::items::Arrows;
pub use crate::items::Bar;
pub use crate::items::BarChart;
pub use crate::items::BoxElem;
pub use crate::items::BoxPlot;
pub use crate::items::BoxSpread;
pub use crate::items::ClosestElem;
pub use crate::items::HLine;
pub use crate::items::Line;
pub use crate::items::LineStyle;
pub use crate::items::MarkerShape;
pub use crate::items::Orientation;
pub use crate::items::PlotConfig;
pub use crate::items::PlotGeometry;
pub use crate::items::PlotImage;
pub use crate::items::PlotItem;
pub use crate::items::PlotItemBase;
pub use crate::items::PlotPoint;
pub use crate::items::PlotPoints;
pub use crate::items::Points;
pub use crate::items::Polygon;
pub use crate::items::Text;
pub use crate::items::VLine;
pub use crate::legend::ColorConflictHandling;
pub use crate::legend::Corner;
pub use crate::legend::Legend;
pub use crate::memory::PlotMemory;
pub use crate::plot::Plot;
pub use crate::plot_ui::PlotUi;
pub use crate::transform::PlotBounds;
pub use crate::transform::PlotTransform;

type LabelFormatterFn<'a> = dyn Fn(&str, &PlotPoint) -> String + 'a;

/// Optional label formatter function for customizing hover labels.
pub type LabelFormatter<'a> = Option<Box<LabelFormatterFn<'a>>>;

type GridSpacerFn<'a> = dyn Fn(GridInput) -> Vec<GridMark> + 'a;
type GridSpacer<'a> = Box<GridSpacerFn<'a>>;

type CoordinatesFormatterFn<'a> = dyn Fn(&PlotPoint, &PlotBounds) -> String + 'a;

/// Specifies the coordinates formatting when passed to
/// [`Plot::coordinates_formatter`].
pub struct CoordinatesFormatter<'a> {
    function: Box<CoordinatesFormatterFn<'a>>,
}

impl<'a> CoordinatesFormatter<'a> {
    /// Create a new formatter based on the pointer coordinate and the plot
    /// bounds.
    pub fn new(function: impl Fn(&PlotPoint, &PlotBounds) -> String + 'a) -> Self {
        Self {
            function: Box::new(function),
        }
    }

    /// Show a fixed number of decimal places.
    pub fn with_decimals(num_decimals: usize) -> Self {
        Self {
            function: Box::new(move |value, _| format!("x: {:.d$}\ny: {:.d$}", value.x, value.y, d = num_decimals)),
        }
    }

    fn format(&self, value: &PlotPoint, bounds: &PlotBounds) -> String {
        (self.function)(value, bounds)
    }
}

impl Default for CoordinatesFormatter<'_> {
    fn default() -> Self {
        Self::with_decimals(3)
    }
}

// ----------------------------------------------------------------------------

/// Indicates a vertical or horizontal cursor line in plot coordinates.
#[derive(Copy, Clone, PartialEq)]
pub enum Cursor {
    /// Horizontal cursor line at the given y-coordinate.
    Horizontal {
        /// Y-coordinate of the horizontal cursor line.
        y: f64,
    },

    /// Vertical cursor line at the given x-coordinate.
    Vertical {
        /// X-coordinate of the vertical cursor line.
        x: f64,
    },
}

/// Contains the cursors drawn for a plot widget in a single frame.
#[derive(PartialEq, Clone)]
struct PlotFrameCursors {
    id: Id,
    cursors: Vec<Cursor>,
}

#[derive(Default, Clone)]
struct CursorLinkGroups(HashMap<Id, Vec<PlotFrameCursors>>);

#[derive(Clone)]
struct LinkedBounds {
    bounds: PlotBounds,
    auto_bounds: Vec2b,
}

#[derive(Default, Clone)]
struct BoundsLinkGroups(HashMap<Id, LinkedBounds>);

// ----------------------------------------------------------------------------

/// What [`Plot::show`] returns.
pub struct PlotResponse<R> {
    /// What the user closure returned.
    pub inner: R,

    /// The response of the plot.
    pub response: Response,

    /// The transform between screen coordinates and plot coordinates.
    pub transform: PlotTransform,

    /// The id of a currently hovered item if any.
    ///
    /// This is `None` if either no item was hovered.
    /// A plot item can be hovered either by hovering its representation in the
    /// plot (line, marker, etc.) or by hovering the item in the legend.
    pub hovered_plot_item: Option<Id>,
}

// ----------------------------------------------------------------------------

/// User-requested modifications to the plot bounds. We collect them in the plot
/// build function to later apply them at the right time, as other modifications
/// need to happen first.
enum BoundsModification {
    SetX(RangeInclusive<f64>),
    SetY(RangeInclusive<f64>),
    Translate(Vec2),
    AutoBounds(Vec2b),
    Zoom(Vec2, PlotPoint),
}

// ----------------------------------------------------------------------------
// Grid

/// Input for "grid spacer" functions.
///
/// See [`Plot::x_grid_spacer()`] and [`Plot::y_grid_spacer()`].
pub struct GridInput {
    /// Min/max of the visible data range (the values at the two edges of the
    /// plot, for the current axis).
    pub bounds: (f64, f64),

    /// Recommended (but not required) lower-bound on the step size returned by
    /// custom grid spacers.
    ///
    /// Computed as the ratio between the diagram's bounds (in plot coordinates)
    /// and the viewport (in frame/window coordinates), scaled up to
    /// represent the minimal possible step.
    ///
    /// Always positive.
    pub base_step_size: f64,
}

/// One mark (horizontal or vertical line) in the background grid of a plot.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GridMark {
    /// X or Y value in the plot.
    pub value: f64,

    /// The (approximate) distance to the next value of same thickness.
    ///
    /// Determines how thick the grid line is painted. It's not important that
    /// `step_size` matches the difference between two `value`s precisely,
    /// but rather that grid marks of same thickness have same `step_size`.
    /// For example, months can have a different number of days, but
    /// consistently using a `step_size` of 30 days is a valid approximation.
    pub step_size: f64,
}

/// Recursively splits the grid into `base` subdivisions (e.g. 100, 10, 1).
///
/// The logarithmic base, expressing how many times each grid unit is
/// subdivided. 10 is a typical value, others are possible though.
pub fn log_grid_spacer(log_base: i64) -> GridSpacer<'static> {
    let log_base = log_base as f64;
    let step_sizes = move |input: GridInput| -> Vec<GridMark> {
        // handle degenerate cases
        if input.base_step_size.abs() < f64::EPSILON {
            return Vec::new();
        }

        // The distance between two of the thinnest grid lines is "rounded" up
        // to the next-bigger power of base
        let smallest_visible_unit = next_power(input.base_step_size, log_base);

        let step_sizes = [
            smallest_visible_unit,
            smallest_visible_unit * log_base,
            smallest_visible_unit * log_base * log_base,
        ];

        generate_marks(step_sizes, input.bounds)
    };

    Box::new(step_sizes)
}

/// Splits the grid into uniform-sized spacings (e.g. 100, 25, 1).
///
/// This function should return 3 positive step sizes, designating where the
/// lines in the grid are drawn. Lines are thicker for larger step sizes.
/// Ordering of returned value is irrelevant.
///
/// Why only 3 step sizes? Three is the number of different line thicknesses
/// that egui typically uses in the grid. Ideally, those 3 are not hardcoded
/// values, but depend on the visible range (accessible through `GridInput`).
pub fn uniform_grid_spacer<'a>(spacer: impl Fn(GridInput) -> [f64; 3] + 'a) -> GridSpacer<'a> {
    let get_marks = move |input: GridInput| -> Vec<GridMark> {
        let bounds = input.bounds;
        let step_sizes = spacer(input);
        generate_marks(step_sizes, bounds)
    };

    Box::new(get_marks)
}

// ----------------------------------------------------------------------------

/// Returns next bigger power in given base
/// e.g.
/// ```ignore
/// use egui_plot::next_power;
/// assert_eq!(next_power(0.01, 10.0), 0.01);
/// assert_eq!(next_power(0.02, 10.0), 0.1);
/// assert_eq!(next_power(0.2,  10.0), 1);
/// ```
fn next_power(value: f64, base: f64) -> f64 {
    debug_assert_ne!(value, 0.0, "Bad input"); // can be negative (typical for Y axis)
    base.powi(value.abs().log(base).ceil() as i32)
}

/// Fill in all values between [min, max] which are a multiple of `step_size`
fn generate_marks(step_sizes: [f64; 3], bounds: (f64, f64)) -> Vec<GridMark> {
    let mut steps = vec![];
    fill_marks_between(&mut steps, step_sizes[0], bounds);
    fill_marks_between(&mut steps, step_sizes[1], bounds);
    fill_marks_between(&mut steps, step_sizes[2], bounds);

    // Remove duplicates:
    // This can happen because we have overlapping steps, e.g.:
    // step_size[0] =   10  =>  [-10, 0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100,
    // 110, 120] step_size[1] =  100  =>  [     0,
    // 100          ] step_size[2] = 1000  =>  [     0
    // ]

    steps.sort_by(|a, b| cmp_f64(a.value, b.value));

    let min_step = step_sizes.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let eps = 0.1 * min_step; // avoid putting two ticks too closely together

    let mut deduplicated: Vec<GridMark> = Vec::with_capacity(steps.len());
    for step in steps {
        if let Some(last) = deduplicated.last_mut() {
            if (last.value - step.value).abs() < eps {
                // Keep the one with the largest step size
                if last.step_size < step.step_size {
                    *last = step;
                }
                continue;
            }
        }
        deduplicated.push(step);
    }

    deduplicated
}

#[test]
fn test_generate_marks() {
    fn approx_eq(a: &GridMark, b: &GridMark) -> bool {
        (a.value - b.value).abs() < 1e-10 && a.step_size == b.step_size
    }

    let gm = |value, step_size| GridMark { value, step_size };

    let marks = generate_marks([0.01, 0.1, 1.0], (2.855, 3.015));
    let expected = vec![
        gm(2.86, 0.01),
        gm(2.87, 0.01),
        gm(2.88, 0.01),
        gm(2.89, 0.01),
        gm(2.90, 0.1),
        gm(2.91, 0.01),
        gm(2.92, 0.01),
        gm(2.93, 0.01),
        gm(2.94, 0.01),
        gm(2.95, 0.01),
        gm(2.96, 0.01),
        gm(2.97, 0.01),
        gm(2.98, 0.01),
        gm(2.99, 0.01),
        gm(3.00, 1.),
        gm(3.01, 0.01),
    ];

    let mut problem = if marks.len() == expected.len() {
        None
    } else {
        Some(format!(
            "Different lengths: got {}, expected {}",
            marks.len(),
            expected.len()
        ))
    };

    for (i, (a, b)) in marks.iter().zip(&expected).enumerate() {
        if !approx_eq(a, b) {
            problem = Some(format!("Mismatch at index {i}: {a:?} != {b:?}"));
            break;
        }
    }

    if let Some(problem) = problem {
        panic!("Test failed: {problem}. Got: {marks:#?}, expected: {expected:#?}");
    }
}

fn cmp_f64(a: f64, b: f64) -> Ordering {
    match a.partial_cmp(&b) {
        Some(ord) => ord,
        None => a.is_nan().cmp(&b.is_nan()),
    }
}

/// Fill in all values between [min, max] which are a multiple of `step_size`
fn fill_marks_between(out: &mut Vec<GridMark>, step_size: f64, (min, max): (f64, f64)) {
    debug_assert!(min <= max, "Bad plot bounds: min: {min}, max: {max}");
    let first = (min / step_size).ceil() as i64;
    let last = (max / step_size).ceil() as i64;

    let marks_iter = (first..last).map(|i| {
        let value = (i as f64) * step_size;
        GridMark { value, step_size }
    });
    out.extend(marks_iter);
}

/// Helper for formatting a number so that we always show at least a few
/// decimals, unless it is an integer, in which case we never show any decimals.
pub fn format_number(number: f64, num_decimals: usize) -> String {
    let is_integral = number as i64 as f64 == number;
    if is_integral {
        // perfect integer - show it as such:
        format!("{number:.0}")
    } else {
        // make sure we tell the user it is not an integer by always showing a decimal
        // or two:
        format!("{:.*}", num_decimals.at_least(1), number)
    }
}

/// Determine a color from a 0-1 strength value.
pub fn color_from_strength(ui: &Ui, strength: f32) -> Color32 {
    let base_color = ui.visuals().text_color();
    base_color.gamma_multiply(strength.sqrt())
}
