#![expect(clippy::print_stderr)]
#![doc = include_str!("../README.md")]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::SavePlotExample;

impl PlotExample for SavePlotExample {
    fn name(&self) -> &'static str {
        "save_plot"
    }

    fn title(&self) -> &'static str {
        "Saving plot"
    }

    fn description(&self) -> &'static str {
        "This example demonstrates how to save a plot as a PNG image file. It shows how to capture a screenshot of the plot using egui's screenshot functionality, extract the plot region, and save it to disk using the image crate. This is useful for exporting visualizations or generating plot images programmatically."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["export"]
    }

    fn thumbnail_bytes(&self) -> &'static [u8] {
        include_bytes!("../screenshot_thumb.png")
    }

    fn show_ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        Self::ui(self, ui)
    }
}
