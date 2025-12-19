#![doc = include_str!("../README.md")]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::PerformanceDemo;

impl PlotExample for PerformanceDemo {
    fn name(&self) -> &'static str {
        "performance"
    }

    fn title(&self) -> &'static str {
        "Performance Demo"
    }

    fn description(&self) -> &'static str {
        "This example demonstrates plotting performance with a large number of markers. Use the controls to adjust the number of markers and observe rendering performance."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["performance", "markers"]
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
