#![doc = include_str!("../README.md")]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::PinsWithTooltipExample;

impl PlotExample for PinsWithTooltipExample {
    fn name(&self) -> &'static str {
        "pins_with_tooltip"
    }

    fn title(&self) -> &'static str {
        "Pins + Tooltip Combined"
    }

    fn description(&self) -> &'static str {
        "This example shows both pins and tooltip used together. Hover to see the tooltip, press P to pin. Both components share the same hit collection for efficiency."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["pins", "tooltip", "combined", "comparison"]
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

