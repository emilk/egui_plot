#![doc = include_str!("../README.md")]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::StackedBarExample;

impl PlotExample for StackedBarExample {
    fn name(&self) -> &'static str {
        "stacked_bar"
    }

    fn title(&self) -> &'static str {
        "Stacked Bar Demo"
    }

    fn description(&self) -> &'static str {
        "This example demonstrates how to create stacked bar charts. It shows multiple bar chart series stacked on top of each other, with customizable orientation."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["bar_chart"]
    }

    fn thumbnail_bytes(&self) -> &'static [u8] {
        include_bytes!("../screenshot_thumb.png")
    }

    fn show_ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        Self::ui(self, ui)
    }
}
