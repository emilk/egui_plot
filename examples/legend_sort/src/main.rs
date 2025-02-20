#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoints};

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([350.0, 200.0]),
        ..Default::default()
    };
    let graph: Vec<[f64; 2]> = vec![[0.0, 1.0], [2.0, 3.0], [3.0, 2.0]];
    let graph2: Vec<[f64; 2]> = vec![[0.0, 2.0], [2.0, 4.0], [3.0, 3.0]];
    let graph3: Vec<[f64; 2]> = vec![[0.0, 3.0], [2.0, 5.0], [3.0, 4.0]];

    eframe::run_native(
        "My egui App with a plot",
        options,
        Box::new(|_cc| {
            Ok(Box::new(MyApp {
                insert_order: false,
                graph,
                graph2,
                graph3,
            }))
        }),
    )
}

#[derive(Default)]
struct MyApp {
    insert_order: bool,
    graph: Vec<[f64; 2]>,
    graph2: Vec<[f64; 2]>,
    graph3: Vec<[f64; 2]>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("If checked the legend will follow the order as the curves are inserted");
            ui.checkbox(&mut self.insert_order, "Insert order");

            Plot::new("My Plot")
                .legend(Legend::default().follow_insertion_order(self.insert_order))
                .show(ui, |plot_ui| {
                    plot_ui.line(Line::new(
                        "3rd Curve",
                        PlotPoints::from(self.graph3.clone()),
                    ));
                    plot_ui.line(Line::new("1st Curve", PlotPoints::from(self.graph.clone())));
                    plot_ui.line(Line::new(
                        "2nd Curve",
                        PlotPoints::from(self.graph2.clone()),
                    ));
                });
            // Remember the position of the plot
        });
    }
}
