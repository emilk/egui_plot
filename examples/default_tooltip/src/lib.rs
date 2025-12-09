#![doc = include_str!("../README.md")]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::DefaultTooltipExample;

impl PlotExample for DefaultTooltipExample {
    fn name(&self) -> &'static str {
        "default_tooltip"
    }

    fn title(&self) -> &'static str {
        "Default Tooltip Demo"
    }

    fn description(&self) -> &'static str {
        "This example demonstrates the default tooltip feature for comparing values across multiple series. Hover over the plot to see nearest points per series. Press P to pin, U to unpin, Delete to clear pins."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["tooltip", "series", "comparison"]
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

