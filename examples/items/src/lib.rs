#![doc = include_str!("../README.md")]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::ItemsExample;

impl PlotExample for ItemsExample {
    fn name(&self) -> &'static str {
        "items"
    }

    fn title(&self) -> &'static str {
        "Items Demo"
    }

    fn description(&self) -> &'static str {
        "This example demonstrates the various plot items available in `egui_plot`, including lines, polygons, points, arrows, text, images, and horizontal/vertical lines. It showcases the different visual elements you can add to a plot."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["items", "lines", "polygons", "arrows", "text"]
    }

    fn thumbnail_bytes(&self) -> &'static [u8] {
        include_bytes!("../screenshot_thumb.png")
    }

    fn show_ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        Self::ui(self, ui)
    }
}
