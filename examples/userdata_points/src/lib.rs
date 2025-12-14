#![doc = include_str!("../README.md")]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::UserdataPointsExample;

impl PlotExample for UserdataPointsExample {
    fn name(&self) -> &'static str {
        "userdata_points"
    }

    fn title(&self) -> &'static str {
        "Example of Userdata Points"
    }

    fn description(&self) -> &'static str {
        "This demo shows how to attach custom data to plot items and display it in tooltips."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["performance"]
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
        ui.scope(|_ui| {}).response
    }
}
