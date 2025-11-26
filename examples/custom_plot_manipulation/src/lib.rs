#![doc = include_str!("../README.md")]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::CustomPlotManipulationExample;

impl PlotExample for CustomPlotManipulationExample {
    fn name(&self) -> &'static str {
        "custom_plot_manipulation"
    }

    fn title(&self) -> &'static str {
        "Custom Plot Manipulation"
    }

    fn description(&self) -> &'static str {
        "This example demonstrates how to implement custom plot manipulation controls using raw input events. It shows how to create alternative pan and zoom behaviors, such as inverting the default Ctrl key behavior, customizing zoom and scroll speeds, and locking axes. This is useful for building specialized interaction patterns that differ from the default `egui_plot` controls."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["interaction", "controls"]
    }

    fn thumbnail_bytes(&self) -> &'static [u8] {
        include_bytes!("../screenshot_thumb.png")
    }

    fn show_ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        Self::ui(self, ui)
    }
}
