use emath::NumExt as _;

use crate::bounds::PlotPoint;

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

type LabelFormatterFn<'a> = dyn Fn(&HoverPosition<'_>) -> Option<String> + 'a;

/// Optional label formatter function for customizing hover labels.
pub type LabelFormatter<'a> = Box<LabelFormatterFn<'a>>;

/// Indicates the position of the cursor in a plot for hover purposes.
#[derive(Copy, Clone, PartialEq)]
pub enum HoverPosition<'a> {
    NearDataPoint {
        /// The name of the plot whose data point is nearest to the cursor
        plot_name: &'a str,

        /// The position of the nearest data point
        position: PlotPoint,

        /// The index of the nearest data point in its plot
        index: usize,
    },
    Elsewhere {
        /// The position in the plot over which the cursor hovers
        position: PlotPoint,
    },
}

/// Default label formatter that shows the x and y coordinates with 3 decimal
/// places.
#[expect(clippy::unnecessary_wraps)]
pub fn default_label_formatter(pos: &HoverPosition<'_>) -> Option<String> {
    Some(match pos {
        HoverPosition::NearDataPoint {
            plot_name,
            position,
            index: _,
        } => format!("{}\nx = {:.3}\ny = {:.3}", plot_name, position.x, position.y),
        HoverPosition::Elsewhere { position } => format!("x = {:.3}\ny = {:.3}", position.x, position.y),
    })
}
