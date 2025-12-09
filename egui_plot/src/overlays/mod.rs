//! Contains widgets that can be added to a plot at some fixed screen
//! coordinates.

mod coordinates;
mod legend;
pub(crate) mod pin;
mod tooltip;

pub use coordinates::CoordinatesFormatter;
pub use legend::ColorConflictHandling;
pub use legend::Legend;
pub use legend::LegendWidget;

// Pin exports
pub use pin::HitPoint;
pub use pin::PinOptions;
pub use pin::PinnedPoints;
pub use pin::init_pins;

// Tooltip exports
pub use tooltip::TooltipOptions;
