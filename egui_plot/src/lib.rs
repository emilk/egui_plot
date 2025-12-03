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
mod overlays;

use egui::Id;

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
pub use overlays::legend::ColorConflictHandling;
pub use crate::placement::Corner;
pub use overlays::legend::Legend;
pub use crate::memory::PlotMemory;
pub use crate::plot::Plot;
pub use crate::plot::PlotResponse;
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
pub use crate::overlays::CoordinatesFormatter;
