#![doc = include_str!("../README.md")]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::PinsExample;

impl PlotExample for PinsExample {
    fn name(&self) -> &'static str {
        "pins"
    }

    fn title(&self) -> &'static str {
        "Pins Demo"
    }

    fn description(&self) -> &'static str {
        "Pins-only example for marking positions. Starts with pre-existing pins. Press P to add, U to remove, Delete to clear."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["pins", "markers", "comparison"]
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
