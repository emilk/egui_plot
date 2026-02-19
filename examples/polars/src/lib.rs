#![doc = include_str!("../README.md")]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::PolarsExample;

impl PlotExample for PolarsExample {
    fn name(&self) -> &'static str {
        "polars"
    }

    fn title(&self) -> &'static str {
        "Polars Demo"
    }

    fn description(&self) -> &'static str {
        "This example demonstrates how to create plots with Polars dataframes."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["polars", "dataframe"]
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
