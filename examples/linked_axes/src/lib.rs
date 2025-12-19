#![doc = include_str!("../README.md")]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::LinkedAxesExample;

impl PlotExample for LinkedAxesExample {
    fn name(&self) -> &'static str {
        "linked_axes"
    }

    fn title(&self) -> &'static str {
        "Linked Axes Example"
    }

    fn description(&self) -> &'static str {
        "This example demonstrates how to link axes and cursors across multiple plots. When you zoom, pan, or move the cursor in one plot, the linked plots will synchronize their view, useful for comparing data across different visualizations."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["axes", "cursor"]
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
