use std::f64::consts::PI;

use eframe::egui;
use eframe::egui::Response;
use egui_plot::FilledArea;
use egui_plot::Legend;
use egui_plot::Line;
use egui_plot::Plot;
use egui_plot::PlotPoints;

pub struct FilledAreaExample {
    delta_lower: f64,
    delta_upper: f64,
    num_points: usize,
}

impl Default for FilledAreaExample {
    fn default() -> Self {
        Self {
            delta_lower: 0.5,
            delta_upper: 0.5,
            num_points: 100,
        }
    }
}

impl FilledAreaExample {
    pub fn show_controls(&mut self, ui: &mut egui::Ui) -> Response {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Lower bound offset:");
                ui.add(
                    egui::Slider::new(&mut self.delta_lower, 0.0..=2.0)
                        .text("δ lower")
                        .step_by(0.1),
                );
            });
            ui.vertical(|ui| {
                ui.label("Upper bound offset:");
                ui.add(
                    egui::Slider::new(&mut self.delta_upper, 0.0..=2.0)
                        .text("δ upper")
                        .step_by(0.1),
                );
            });
            ui.vertical(|ui| {
                ui.label("Number of points:");
                ui.add(egui::Slider::new(&mut self.num_points, 10..=500).text("points"));
            });
        })
        .response
    }

    pub fn show_plot(&self, ui: &mut egui::Ui) -> Response {
        // Generate x values
        let xs: Vec<f64> = (0..self.num_points)
            .map(|i| i as f64 * 4.0 * PI / self.num_points as f64)
            .collect();

        // Generate sin(x) and bounds
        let ys: Vec<f64> = xs.iter().map(|&x| x.sin()).collect();
        let ys_min: Vec<f64> = ys.iter().map(|&y| y - self.delta_lower).collect();
        let ys_max: Vec<f64> = ys.iter().map(|&y| y + self.delta_upper).collect();

        // Create the center line
        let sin_line = Line::new(
            "sin(x)",
            xs.iter()
                .zip(ys.iter())
                .map(|(&x, &y)| [x, y])
                .collect::<PlotPoints<'_>>(),
        )
        .color(egui::Color32::from_rgb(200, 100, 100));

        // Create the filled area
        let filled_area = FilledArea::new("sin(x) +/- deltas", &xs, &ys_min, &ys_max)
            .fill_color(egui::Color32::from_rgba_unmultiplied(100, 200, 100, 50));

        Plot::new("Filled Area Demo")
            .legend(Legend::default())
            .show(ui, |plot_ui| {
                plot_ui.add(filled_area);
                plot_ui.line(sin_line);
            })
            .response
    }
}
