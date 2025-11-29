#![doc = include_str!("../README.md")]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::BorrowPointsExample;

impl PlotExample for BorrowPointsExample {
    fn name(&self) -> &'static str {
        "borrow_points"
    }

    fn title(&self) -> &'static str {
        "Example of borrowing points"
    }

    fn description(&self) -> &'static str {
        "This example demonstrates how to borrow points instead of cloning them when creating plot lines. It shows how to use `PlotPoints::Borrowed` to avoid unnecessary allocations, which is useful for performance-critical applications or when you want to reuse the same data across multiple frames without copying."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["performance"]
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
        ui.scope(|_ui| {}).response
    }
}
