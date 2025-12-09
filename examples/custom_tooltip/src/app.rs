use std::f64::consts::PI;

use eframe::egui;
use egui::Color32;
use egui::Response;
use egui::RichText;
use egui_plot::HitPoint;
use egui_plot::Line;
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
        ui.label("This demo shows a custom tooltip UI with mismatched x-sampling across series.")
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

                // Custom tooltip UI
                plot_ui.show_tooltip_custom(&TooltipOptions::default(), |ui, hits: &[HitPoint]| {
                    ui.strong("Custom Tooltip");
                    ui.separator();

                    if hits.is_empty() {
                        ui.weak("No data points nearby");
                        return;
                    }

                    egui::Grid::new("custom_tooltip_grid")
                        .num_columns(3)
                        .spacing([12.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            ui.strong("Series");
                            ui.strong("X");
                            ui.strong("Y");
                            ui.end_row();

                            for h in hits {
                                ui.label(RichText::new(&h.series_name).color(h.color));
                                ui.monospace(format!("{:.4}", h.value.x));
                                ui.monospace(format!("{:.4}", h.value.y));
                                ui.end_row();
                            }
                        });

                    ui.add_space(4.0);
                    ui.weak(format!("Showing {} series", hits.len()));
                });
            })
            .response
    }
}
