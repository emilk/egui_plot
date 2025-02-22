#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoint, PlotPoints};

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([350.0, 200.0]),
        ..Default::default()
    };

    let points: Vec<[f64; 2]> = vec![[0.0, 1.0], [2.0, 3.0], [3.0, 2.0]];

    let points: Vec<PlotPoint> = points.iter().map(|p| PlotPoint::new(p[0], p[1])).collect();
    eframe::run_native(
        "My egui App with a plot",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp { points }))),
    )
}

#[derive(Default)]
struct MyApp {
    points: Vec<PlotPoint>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            Plot::new("My Plot")
                .legend(Legend::default())
                .show(ui, |plot_ui| {
                    plot_ui
                        .line(Line::new("curve", PlotPoints::Borrowed(&self.points)).name("curve"));
                });
        });
    }
}
