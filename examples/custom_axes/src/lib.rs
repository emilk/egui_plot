#![doc = include_str!("../README.md")]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::CustomAxesExample;

impl PlotExample for CustomAxesExample {
    fn name(&self) -> &'static str {
        "custom_axes"
    }

    fn title(&self) -> &'static str {
        "Custom Axes Demo"
    }

    fn description(&self) -> &'static str {
        "This example demonstrates how to create custom axes with custom formatters and grid spacers. It shows a logistic function with time-based X-axis formatting (days, hours, minutes) and percentage-based Y-axis formatting, demonstrating how to create domain-specific axis labels."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["axes"]
    }

    fn thumbnail_bytes(&self) -> &'static [u8] {
        include_bytes!("../screenshot_thumb.png")
    }

    fn show_ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        Self::ui(self, ui)
    }
}
