#![doc = include_str!("../README.md")]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::LegendSortExample;

impl PlotExample for LegendSortExample {
    fn name(&self) -> &'static str {
        "legend_sort"
    }

    fn title(&self) -> &'static str {
        "Legend Sorting"
    }

    fn description(&self) -> &'static str {
        "This example demonstrates how to control the sorting order of legend entries. It shows how to use `follow_insertion_order()` to display legend entries in the order they were added to the plot, rather than alphabetically, which is useful for maintaining a specific visual hierarchy in the legend."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["legend"]
    }

    fn thumbnail_bytes(&self) -> &'static [u8] {
        include_bytes!("../screenshot_thumb.png")
    }

    fn show_ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        Self::ui(self, ui)
    }
}
