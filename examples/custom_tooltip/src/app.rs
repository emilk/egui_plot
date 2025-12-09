use std::f64::consts::PI;

use eframe::egui;
use egui::Color32;
use egui::Response;
use egui::RichText;
use egui_plot::HitPoint;
use egui_plot::Line;
use egui_plot::PinnedPoints;
use egui_plot::Plot;
use egui_plot::TooltipOptions;

#[derive(Clone, PartialEq, Eq)]
pub struct CustomTooltipExample {
    n_series1: usize,
    n_series2: usize,
}

impl Default for CustomTooltipExample {
    fn default() -> Self {
        Self {
            n_series1: 100,
            n_series2: 300,
        }
    }
}

impl CustomTooltipExample {
    pub fn show_controls(&mut self, ui: &mut egui::Ui) -> Response {
        ui.horizontal(|ui| {
            ui.label("Series 1 points:");
            ui.add(egui::DragValue::new(&mut self.n_series1).speed(5).range(10..=500));
            ui.label("Series 2 points:");
            ui.add(egui::DragValue::new(&mut self.n_series2).speed(5).range(10..=500));
        });
        ui.label("This demo shows mismatched x-sampling across series.");
        ui.label("Press P to pin, U to unpin last, Delete to clear all pins.")
    }

    pub fn show_plot(&self, ui: &mut egui::Ui) -> Response {
        let t_min = 0.0;
        let t_max = 4.0 * PI;

        let linspace = |k: usize| -> Vec<f64> {
            if k <= 1 {
                return vec![t_min];
            }
            let step = (t_max - t_min) / (k as f64 - 1.0);
            (0..k).map(|i| t_min + step * (i as f64)).collect()
        };

        let x1 = linspace(self.n_series1);
        let f1: Vec<f64> = x1.iter().map(|&t| t.sin()).collect();

        let x2 = linspace(self.n_series2);
        let f2: Vec<f64> = x2.iter().map(|&t| (t * 0.6 + 0.8).sin() * 0.8 + 0.2).collect();

        Plot::new("custom_tooltip_demo")
            .show(ui, |plot_ui| {
                let s1: Vec<[f64; 2]> = x1.iter().zip(f1.iter()).map(|(&x, &y)| [x, y]).collect();
                let s2: Vec<[f64; 2]> = x2.iter().zip(f2.iter()).map(|(&x, &y)| [x, y]).collect();

                plot_ui.line(
                    Line::new(format!("f1 (n={})", self.n_series1), s1)
                        .color(Color32::from_rgb(120, 220, 120))
                        .width(2.0),
                );
                plot_ui.line(
                    Line::new(format!("f2 (n={})", self.n_series2), s2)
                        .color(Color32::from_rgb(120, 160, 255))
                        .width(2.0),
                );

                plot_ui.show_tooltip_across_series_with(
                    &TooltipOptions::default(),
                    |ui, _hits: &[HitPoint], pins: &[PinnedPoints]| {
                        ui.strong("Pinned snapshots");
                        if pins.is_empty() {
                            ui.weak("No pins yet. Hover and press P to pin, U to unpin last, Delete to clear.");
                            return;
                        }

                        for (k, snap) in pins.iter().enumerate() {
                            egui::CollapsingHeader::new(format!("Pin #{k}"))
                                .default_open(false)
                                .show(ui, |ui| {
                                    egui::Grid::new(format!("pin_grid_{k}"))
                                        .num_columns(4)
                                        .spacing([8.0, 2.0])
                                        .striped(true)
                                        .show(ui, |ui| {
                                            ui.weak("");
                                            ui.weak("series");
                                            ui.weak("x");
                                            ui.weak("y");
                                            ui.end_row();

                                            for h in &snap.hits {
                                                ui.label(RichText::new("●").color(h.color));
                                                ui.monospace(&h.series_name);
                                                ui.monospace(format!("{:.6}", h.value.x));
                                                ui.monospace(format!("{:.6}", h.value.y));
                                                ui.end_row();
                                            }
                                        });
                                });
                        }

                        ui.add_space(6.0);
                        ui.weak("Hotkeys: P = pin current, U = unpin last, Delete = clear all");
                    },
                );
            })
            .response
    }
}
