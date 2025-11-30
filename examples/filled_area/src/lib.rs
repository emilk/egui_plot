#![doc = include_str!("../README.md")]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::FilledAreaExample;

impl PlotExample for FilledAreaExample {
    fn name(&self) -> &'static str {
        "filled_area"
    }

    fn title(&self) -> &'static str {
        "Filled Area Demo"
    }

    fn description(&self) -> &'static str {
        "This example demonstrates how to create filled areas between two lines. It shows a sine wave with an adjustable confidence band around it, useful for visualizing uncertainty, ranges, and confidence intervals."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["filled_area", "confidence_interval", "range"]
    }

    fn thumbnail_bytes(&self) -> &'static [u8] {
        include_bytes!("../screenshot_thumb.png")
    }

    fn code_bytes(&self) -> &'static [u8] {
        include_bytes!("./app.rs")
    }

    fn show_ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        self.show_plot(ui)
    }

    fn show_controls(&mut self, ui: &mut egui::Ui) -> egui::Response {
        self.show_controls(ui)
    }
}
