use eframe::egui;
use egui::Color32;
use egui::Response;
use egui_plot::Line;
use egui_plot::PinOptions;
use egui_plot::Plot;
use egui_plot::TooltipOptions;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PinsWithTooltipExample {
    n_points: usize,
}

impl Default for PinsWithTooltipExample {
    fn default() -> Self {
        Self { n_points: 100 }
    }
}

impl PinsWithTooltipExample {
    pub fn show_controls(&mut self, ui: &mut egui::Ui) -> Response {
        ui.horizontal(|ui| {
            ui.label("Number of points:");
            ui.add(egui::DragValue::new(&mut self.n_points).speed(10).range(10..=500));
        });
        ui.label("Hover to see tooltip. Press P to pin, U to unpin, Delete to clear.")
    }

    pub fn show_plot(&self, ui: &mut egui::Ui) -> Response {
        let x: Vec<f64> = (0..self.n_points).map(|i| i as f64 * 0.1).collect();
        let f1: Vec<f64> = x.iter().map(|&t| t.sin()).collect();
        let f2: Vec<f64> = x.iter().map(|&t| (t * 0.6 + 0.8).sin() * 0.8 + 0.2).collect();

        Plot::new("pins_with_tooltip_demo")
            .show(ui, |plot_ui| {
                let s1: Vec<[f64; 2]> = x.iter().zip(f1.iter()).map(|(&x, &y)| [x, y]).collect();
                let s2: Vec<[f64; 2]> = x.iter().zip(f2.iter()).map(|(&x, &y)| [x, y]).collect();

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

                // Collect hits once, share between both components
                let hits = plot_ui.collect_hits(50.0);

                // Show pins (draws rails, markers, handles P/U/Del)
                plot_ui.show_pins_with_hits(&PinOptions::default(), &hits);

                // Show tooltip (draws guide, band, tooltip popup)
                plot_ui.show_tooltip_with_hits(&TooltipOptions::default(), &hits);
            })
            .response
    }
}

