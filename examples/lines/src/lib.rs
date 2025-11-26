#![doc = include_str!("../README.md")]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::LineExample;

impl PlotExample for LineExample {
    fn name(&self) -> &'static str {
        "lines"
    }

    fn title(&self) -> &'static str {
        "Line Demo"
    }

    fn description(&self) -> &'static str {
        "This example demonstrates various line plotting features including animated lines, different line styles (solid, dashed, dotted), gradients, fills, axis inversion, and coordinate display. It shows a comprehensive set of line customization options."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["lines", "animation", "styling"]
    }

    fn thumbnail_bytes(&self) -> &'static [u8] {
        include_bytes!("../screenshot_thumb.png")
    }

    fn show_ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        Self::ui(self, ui)
    }
}
