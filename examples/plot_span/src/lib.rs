#![doc = include_str!("../README.md")]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::PlotSpanDemo;

impl PlotExample for PlotSpanDemo {
    fn name(&self) -> &'static str {
        "plot_span"
    }

    fn title(&self) -> &'static str {
        "Plot Span Demo"
    }

    fn description(&self) -> &'static str {
        "This example demonstrates how to add spans to a plot. Spans are shaded regions that can highlight ranges on either the X or Y axis."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["span", "annotation"]
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
