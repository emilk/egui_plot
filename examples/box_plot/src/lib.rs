#![doc = include_str!("../README.md")]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::BoxPlotExample;

impl PlotExample for BoxPlotExample {
    fn name(&self) -> &'static str {
        "box_plot"
    }

    fn title(&self) -> &'static str {
        "Box Plot Demo"
    }

    fn description(&self) -> &'static str {
        "This example demonstrates how to create box plots (box-and-whisker plots) with customizable orientation. It shows multiple box plots with different experiments and days, allowing you to visualize statistical distributions."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["box_plot", "color", "legend"]
    }

    fn thumbnail_bytes(&self) -> &'static [u8] {
        include_bytes!("../screenshot_thumb.png")
    }

    fn show_ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        Self::ui(self, ui)
    }
}
