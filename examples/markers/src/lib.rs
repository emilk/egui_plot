#![doc = include_str!("../README.md")]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::MarkerDemo;

impl PlotExample for MarkerDemo {
    fn name(&self) -> &'static str {
        "markers"
    }

    fn title(&self) -> &'static str {
        "Marker Demo"
    }

    fn description(&self) -> &'static str {
        "This example demonstrates the different marker shapes available for point plots. It shows all available marker types with customizable fill, radius, and color options."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["markers", "points"]
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
