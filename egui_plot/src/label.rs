use egui::Id;
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

type LabelFormatterFn<'a> = dyn Fn(&str, &PlotPoint, Option<(Id, usize)>) -> String + 'a;

/// Optional label formatter function for customizing hover labels.
///
/// The formatter receives the item name, the hovered point, and an optional
/// `(Id, index)` for the hovered plot item. The `Id` matches the item `id()`,
/// and `index` is the point index within that item. The argument is `None`
/// when the cursor isn't hovering a concrete plot item.
pub type LabelFormatter<'a> = Box<LabelFormatterFn<'a>>;

/// Default label formatter that shows the x and y coordinates with 3 decimal
/// places.
pub fn default_label_formatter(name: &str, value: &PlotPoint, _id_index: Option<(Id, usize)>) -> String {
    let prefix = if name.is_empty() {
        String::new()
    } else {
        format!("{name}\n")
    };
    format!("{}x = {:.3}\ny = {:.3}", prefix, value.x, value.y)
}
