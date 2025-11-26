#![doc = include_str!("../README.md")]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::InteractionExample;

impl PlotExample for InteractionExample {
    fn name(&self) -> &'static str {
        "interaction"
    }

    fn title(&self) -> &'static str {
        "Interaction Demo"
    }

    fn description(&self) -> &'static str {
        "This example demonstrates how to interact with plots programmatically. It shows how to access plot bounds, pointer coordinates, drag deltas, and detect hovered plot items, providing a foundation for building interactive plot applications."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["interaction"]
    }

    fn thumbnail_bytes(&self) -> &'static [u8] {
        include_bytes!("../screenshot_thumb.png")
    }

    fn show_ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        Self::ui(self, ui)
    }
}
