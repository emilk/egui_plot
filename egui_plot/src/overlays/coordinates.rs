use crate::bounds::PlotBounds;
use crate::data::PlotPoint;

type CoordinatesFormatterFn<'a> = dyn Fn(&PlotPoint, &PlotBounds) -> String + 'a;

/// Specifies the coordinates formatting when passed to
/// [`crate::Plot::coordinates_formatter`].
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

    pub(crate) fn format(&self, value: &PlotPoint, bounds: &PlotBounds) -> String {
        (self.function)(value, bounds)
    }
}

impl Default for CoordinatesFormatter<'_> {
    fn default() -> Self {
        Self::with_decimals(3)
    }
}
