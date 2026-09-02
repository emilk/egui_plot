#![cfg_attr(doc, doc = include_str!("../README.md"))]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::PerformanceLinesDemo;

impl PlotExample for PerformanceLinesDemo {
    fn name(&self) -> &'static str {
        "performance_lines"
    }

    fn title(&self) -> &'static str {
        "Performance Lines Demo"
    }

    fn description(&self) -> &'static str {
        "This example demonstrates plotting performance with 10 random walks. Use the controls to adjust the number of values in each walk and observe rendering performance."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["lines", "performance"]
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
