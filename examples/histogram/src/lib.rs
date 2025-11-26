#![doc = include_str!("../README.md")]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::HistogramExample;

impl PlotExample for HistogramExample {
    fn name(&self) -> &'static str {
        "histogram"
    }

    fn title(&self) -> &'static str {
        "Histogram Demo"
    }

    fn description(&self) -> &'static str {
        "This example demonstrates how to create histograms using bar charts. It displays a normal distribution with customizable orientation, zoom, drag, and scroll controls."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["histogram", "bar_chart"]
    }

    fn thumbnail_bytes(&self) -> &'static [u8] {
        include_bytes!("../screenshot_thumb.png")
    }

    fn show_ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        Self::ui(self, ui)
    }
}
