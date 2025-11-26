#![doc = include_str!("../README.md")]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::LegendExample;

impl PlotExample for LegendExample {
    fn name(&self) -> &'static str {
        "legend"
    }

    fn title(&self) -> &'static str {
        "Legend Demo"
    }

    fn description(&self) -> &'static str {
        "This example demonstrates how to customize plot legends. It shows how to configure legend position, text style, and background opacity, with multiple lines displayed in the legend."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["legend"]
    }

    fn thumbnail_bytes(&self) -> &'static [u8] {
        include_bytes!("../screenshot_thumb.png")
    }

    fn show_ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        Self::ui(self, ui)
    }
}
