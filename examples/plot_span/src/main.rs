#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![expect(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::{
    egui::{self, Align2, Color32},
    epaint::Hsva,
};
use egui_plot::{Legend, Line, Plot, PlotPoints, Span};

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App with a plot",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp {}))),
    )
}

#[derive(Default)]
struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            Plot::new("My Plot").legend(Legend::default()).show(ui, |plot_ui| {
                let span = Span::new("Span 1", -10.0..=-5.0)
                    .border_style(egui_plot::LineStyle::Dashed { length: 50.0 })
                    .border_width(3.0);
                plot_ui.span(span);

                let span = Span::new("Span 2", 0.0..=1.0);
                plot_ui.span(span);

                let span = Span::new("Span 3", 5.0..=6.0).axis(egui_plot::Axis::Y);
                plot_ui.span(span);

                let color4: Color32 = Hsva::new(0.1, 0.85, 0.5, 0.15).into();
                let span4 = Span::new("Span 4", 5.0..=5.5)
                    .border_width(0.0)
                    .fill(color4)
                    .label_align(Align2::LEFT_BOTTOM);
                plot_ui.span(span4.clone());

                let color5: Color32 = Hsva::new(0.3, 0.85, 0.5, 0.15).into();
                let span5 = Span::new("Span 5", 5.5..=6.5)
                    .border_width(0.0)
                    .fill(color5)
                    .label_align(Align2::LEFT_BOTTOM);
                plot_ui.span(span5.clone());

                let span = span4.clone().range(6.5..=8.0);
                plot_ui.span(span);

                let span = span5.clone().range(8.0..=10.0);
                plot_ui.span(span);

                let span = Span::new("Infinite span", 10.0..=f64::INFINITY);
                plot_ui.span(span);

                let sine_points = PlotPoints::from_explicit_callback(|x| x.sin(), .., 5000);
                let sine_line = Line::new("Sine", sine_points).name("Sine");

                plot_ui.line(sine_line);
            });
        });
    }
}
