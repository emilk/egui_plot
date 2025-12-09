use eframe::egui;
use egui::Color32;
use egui::Response;
use egui_plot::Line;
use egui_plot::Plot;
use egui_plot::TooltipOptions;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TooltipExample {
    n_points: usize,
}

impl Default for TooltipExample {
    fn default() -> Self {
        Self { n_points: 20 }
    }
}

impl TooltipExample {
    pub fn show_controls(&mut self, ui: &mut egui::Ui) -> Response {
        ui.horizontal(|ui| {
            ui.label("Number of points:");
            ui.add(egui::DragValue::new(&mut self.n_points).speed(1).range(10..=200));
        });
        ui.label("Hover the plot to see nearest points per series.")
    }

    pub fn show_plot(&self, ui: &mut egui::Ui) -> Response {
        let x1: Vec<f64> = (0..self.n_points).map(|i| i as f64 * 0.1).collect();
        let x2: Vec<f64> = (0..self.n_points / 2).map(|i| i as f64 * 0.2).collect();
        let f1: Vec<f64> = x1.iter().map(|&t| t.sin()).collect();
        let f2: Vec<f64> = x2.iter().map(|&t| (t * 0.6 + 0.8).sin() * 0.8 + 0.2).collect();

        Plot::new("tooltip_demo")
            .show(ui, |plot_ui| {
                let s1: Vec<[f64; 2]> = x1.iter().zip(f1.iter()).map(|(&x, &y)| [x, y]).collect();
                let s2: Vec<[f64; 2]> = x2.iter().zip(f2.iter()).map(|(&x, &y)| [x, y]).collect();

                plot_ui.line(
                    Line::new("sin(x)", s1)
                        .color(Color32::from_rgb(120, 220, 120))
                        .width(2.0),
                );
                plot_ui.line(
                    Line::new("shifted sin", s2)
                        .color(Color32::from_rgb(120, 160, 255))
                        .width(2.0),
                );

                // Tooltip only - no pins
                plot_ui.tooltip(&TooltipOptions::default());
            })
            .response
    }
}
