use eframe::egui;
use egui::Color32;
use egui::Pos2;
use egui::Response;
use egui_plot::HitPoint;
use egui_plot::Line;
use egui_plot::PinOptions;
use egui_plot::PinnedPoints;
use egui_plot::Plot;
use egui_plot::PlotPoint;
use egui_plot::init_pins;

const PLOT_ID: &str = "pins_demo";

pub struct PinsExample {
    n_points: usize,
    pins_initialized: bool,
}

impl Default for PinsExample {
    fn default() -> Self {
        Self {
            n_points: 100,
            pins_initialized: false,
        }
    }
}

impl Clone for PinsExample {
    fn clone(&self) -> Self {
        Self {
            n_points: self.n_points,
            pins_initialized: self.pins_initialized,
        }
    }
}

impl PartialEq for PinsExample {
    fn eq(&self, other: &Self) -> bool {
        self.n_points == other.n_points
    }
}

impl Eq for PinsExample {}

impl PinsExample {
    pub fn show_controls(&mut self, ui: &mut egui::Ui) -> Response {
        ui.horizontal(|ui| {
            ui.label("Number of points:");
            ui.add(egui::DragValue::new(&mut self.n_points).speed(10).range(10..=500));
        });
        ui.label("This demo starts with 3 pre-existing pins at x = 1.0, 3.0, and 5.0");
        ui.label("Press P to pin, U to unpin last, Delete to clear all pins.")
    }

    /// Create pre-existing pins with hit points for the two series
    fn create_initial_pins() -> Vec<PinnedPoints> {
        let pin_positions: [f64; 3] = [1.0, 3.0, 5.0];

        pin_positions
            .iter()
            .map(|&x| {
                let y1 = x.sin();
                let y2 = (x * 0.6 + 0.8).sin() * 0.8 + 0.2;

                PinnedPoints {
                    hits: vec![
                        HitPoint {
                            series_name: "sin(x)".to_owned(),
                            color: Color32::from_rgb(120, 220, 120),
                            value: PlotPoint::new(x, y1),
                            screen_pos: Pos2::ZERO,
                            screen_dx: 0.0,
                            screen_dy: 0.0,
                            is_highlighted: false,
                        },
                        HitPoint {
                            series_name: "shifted sin".to_owned(),
                            color: Color32::from_rgb(120, 160, 255),
                            value: PlotPoint::new(x, y2),
                            screen_pos: Pos2::ZERO,
                            screen_dx: 0.0,
                            screen_dy: 0.0,
                            is_highlighted: false,
                        },
                    ],
                    plot_x: x,
                }
            })
            .collect()
    }

    pub fn show_plot(&mut self, ui: &mut egui::Ui) -> Response {
        // Initialize pins on first frame
        if !self.pins_initialized {
            let pins = Self::create_initial_pins();
            init_pins(ui.ctx(), PLOT_ID, pins);
            self.pins_initialized = true;
        }

        let x: Vec<f64> = (0..self.n_points).map(|i| i as f64 * 0.1).collect();
        let f1: Vec<f64> = x.iter().map(|&t| t.sin()).collect();
        let f2: Vec<f64> = x.iter().map(|&t| (t * 0.6 + 0.8).sin() * 0.8 + 0.2).collect();

        Plot::new(PLOT_ID)
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

                // Pins only - no tooltip
                plot_ui.show_pins(&PinOptions::default());
            })
            .response
    }
}
