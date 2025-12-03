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
mod colors;
mod items;
mod legend;
mod math;
mod memory;
mod plot;
mod plot_ui;
mod rect_elem;
mod transform;
mod utils;
mod values;
mod placement;
mod bounds;
mod grid;
mod label;
mod cursor;

use egui::Color32;
use egui::Id;
use egui::Response;
use egui::Ui;

pub use crate::axis::Axis;
pub use crate::axis::AxisHints;
pub use placement::HPlacement;
pub use placement::Placement;
pub use placement::VPlacement;
pub use crate::items::Arrows;
pub use crate::items::Bar;
pub use crate::items::BarChart;
pub use crate::items::BoxElem;
pub use crate::items::BoxPlot;
pub use crate::items::BoxSpread;
pub use crate::values::ClosestElem;
pub use crate::items::HLine;
pub use crate::items::Heatmap;
pub use crate::items::Line;
pub use crate::values::LineStyle;
pub use crate::values::MarkerShape;
pub use crate::values::Orientation;
pub use crate::items::PlotConfig;
pub use crate::values::PlotGeometry;
pub use crate::items::PlotImage;
pub use crate::items::PlotItem;
pub use crate::items::PlotItemBase;
pub use crate::values::PlotPoint;
pub use crate::values::PlotPoints;
pub use crate::items::Points;
pub use crate::items::Polygon;
pub use crate::items::Span;
pub use crate::items::Text;
pub use crate::items::VLine;
pub use crate::legend::ColorConflictHandling;
pub use crate::placement::Corner;
pub use crate::legend::Legend;
pub use crate::memory::PlotMemory;
pub use crate::plot::Plot;
pub use crate::plot_ui::PlotUi;
pub use crate::cursor::Cursor;
pub(crate) use crate::cursor::PlotFrameCursors;
pub(crate) use crate::cursor::CursorLinkGroups;
pub use crate::grid::GridInput;
pub use crate::grid::GridMark;
pub use crate::grid::GridSpacer;
pub use crate::grid::log_grid_spacer;
pub use crate::grid::uniform_grid_spacer;
pub use bounds::PlotBounds;
pub use crate::transform::PlotTransform;

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

/// Determine a color from a 0-1 strength value.
pub fn color_from_strength(ui: &Ui, strength: f32) -> Color32 {
    let base_color = ui.visuals().text_color();
    base_color.gamma_multiply(strength.sqrt())
}
