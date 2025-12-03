use ahash::HashMap;
use egui::Id;

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
pub(crate) struct PlotFrameCursors {
    pub(crate) id: Id,
    pub(crate) cursors: Vec<Cursor>,
}

#[derive(Default, Clone)]
pub(crate) struct CursorLinkGroups(pub(crate) HashMap<Id, Vec<PlotFrameCursors>>);
