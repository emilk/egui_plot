#![doc = include_str!("../README.md")]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::CustomTooltipExample;

impl PlotExample for CustomTooltipExample {
    fn name(&self) -> &'static str {
        "custom_tooltip"
    }

    fn title(&self) -> &'static str {
        "Custom Tooltip Demo"
    }

    fn description(&self) -> &'static str {
        "This example demonstrates custom tooltip UI with different x-sampling across series. Shows how to use show_tooltip_across_series_with() for full control over tooltip rendering."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["tooltip", "custom", "series", "pins"]
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

