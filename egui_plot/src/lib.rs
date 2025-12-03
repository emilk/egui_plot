//! Simple plotting library for [`egui`](https://github.com/emilk/egui).
//!
//! Check out [`Plot`] for how to get started.
//!
//! [**Looking for maintainer!**](https://github.com/emilk/egui/issues/4705)
//!
//! ## Feature flags
#![cfg_attr(feature = "document-features", doc = document_features::document_features!())]
//!

mod aesthetics;
mod axis;
mod bounds;
mod colors;
mod cursor;
mod data;
mod grid;
mod items;
mod label;
mod math;
mod memory;
mod overlays;
mod placement;
mod plot;
mod plot_ui;
mod rect_elem;
mod transform;
mod utils;

pub use bounds::PlotBounds;
use egui::Id;
pub use overlays::ColorConflictHandling;
pub use overlays::Legend;
pub use placement::HPlacement;
pub use placement::Placement;
pub use placement::VPlacement;

pub use crate::aesthetics::LineStyle;
pub use crate::aesthetics::MarkerShape;
pub use crate::aesthetics::Orientation;
pub use crate::axis::Axis;
pub use crate::axis::AxisHints;
pub use crate::colors::color_from_strength;
pub use crate::cursor::Cursor;
pub use crate::data::PlotPoint;
pub use crate::data::PlotPoints;
pub use crate::grid::GridInput;
pub use crate::grid::GridMark;
pub use crate::grid::log_grid_spacer;
pub use crate::grid::uniform_grid_spacer;
pub use crate::items::Arrows;
pub use crate::items::Bar;
pub use crate::items::BarChart;
pub use crate::items::BoxElem;
pub use crate::items::BoxPlot;
pub use crate::items::BoxSpread;
pub use crate::items::ClosestElem;
pub use crate::items::HLine;
pub use crate::items::Heatmap;
pub use crate::items::Line;
pub use crate::items::PlotConfig;
pub use crate::items::PlotGeometry;
pub use crate::items::PlotImage;
pub use crate::items::PlotItem;
pub use crate::items::PlotItemBase;
pub use crate::items::Points;
pub use crate::items::Polygon;
pub use crate::items::Span;
pub use crate::items::Text;
pub use crate::items::VLine;
pub use crate::label::LabelFormatter;
pub use crate::label::format_number;
pub use crate::memory::PlotMemory;
pub use crate::overlays::CoordinatesFormatter;
pub use crate::placement::Corner;
pub use crate::plot::Plot;
pub use crate::plot::PlotResponse;
pub use crate::plot_ui::PlotUi;
pub use crate::transform::PlotTransform;
