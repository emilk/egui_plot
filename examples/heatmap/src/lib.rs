#![doc = include_str!("../README.md")]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::HeatmapDemo;

impl PlotExample for HeatmapDemo {
    fn name(&self) -> &'static str {
        "heatmap"
    }

    fn title(&self) -> &'static str {
        "Heatmap Demo"
    }

    fn description(&self) -> &'static str {
        "This example demonstrates how to create animated heatmaps with customizable color palettes, dimensions, and labels. It visualizes a 2D grid of values using color gradients."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["heatmap", "color", "grid", "visualization"]
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
