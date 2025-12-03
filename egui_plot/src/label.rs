use emath::NumExt as _;

use crate::PlotPoint;

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

type LabelFormatterFn<'a> = dyn Fn(&str, &PlotPoint) -> String + 'a;
/// Optional label formatter function for customizing hover labels.
pub type LabelFormatter<'a> = Option<Box<LabelFormatterFn<'a>>>;
