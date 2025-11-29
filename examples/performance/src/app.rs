use eframe::egui;
use eframe::egui::Response;
use egui_plot::MarkerShape;
use egui_plot::Plot;
use egui_plot::Points;

/// Simple LCG pseudo-random number generator. Returns a value in [0.0, 1.0].
fn rng(state: &mut u64) -> f64 {
    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *state = x;
    (x as f64) / (u64::MAX as f64)
}

fn make_markers(target_count: usize) -> Vec<[f64; 2]> {
    let mut state = 42u64;
    (0..target_count).map(|_| [rng(&mut state), rng(&mut state)]).collect()
}

pub struct PerformanceDemo {
    target_count: usize,
    marker_radius: f32,
    markers: Vec<[f64; 2]>,
    marker_shape: MarkerShape,
}

impl Default for PerformanceDemo {
    fn default() -> Self {
        Self {
            target_count: 100,
            marker_radius: 1.0,
            markers: make_markers(100),
            marker_shape: MarkerShape::Circle,
        }
    }
}

impl PerformanceDemo {
    pub fn show_plot(&self, ui: &mut egui::Ui) -> Response {
        Plot::new("performance_demo")
            .data_aspect(1.0)
            .show(ui, |plot_ui| {
                plot_ui.points(
                    Points::new("markers", self.markers.clone())
                        .radius(self.marker_radius)
                        .shape(self.marker_shape)
                        .filled(true),
                );
            })
            .response
    }

    pub fn show_controls(&mut self, ui: &mut egui::Ui) -> Response {
        ui.ctx().request_repaint(); // Continuous repaint for FPS counter
        let fps = (1.0 / ui.ctx().input(|i| i.stable_dt)).round();

        ui.horizontal(|ui| {
            ui.label("Markers:");
            if ui
                .add(
                    egui::DragValue::new(&mut self.target_count)
                        .speed(100)
                        .range(100..=10_000_000),
                )
                .changed()
            {
                self.markers = make_markers(self.target_count);
            }

            ui.label("Radius:");
            ui.add(
                egui::DragValue::new(&mut self.marker_radius)
                    .speed(0.1)
                    .range(0.5..=5.0),
            );

            ui.label("Shape:");
            egui::ComboBox::from_id_salt("marker_shape")
                .selected_text(format!("{:?}", self.marker_shape))
                .show_ui(ui, |ui| {
                    for shape in MarkerShape::all() {
                        ui.selectable_value(&mut self.marker_shape, shape, format!("{shape:?}"));
                    }
                });

            ui.label(format!("FPS: {fps}"));
        });

        ui.label("Note: Less than 100k markers should work fine, beyond that may cause issues.");
        ui.response()
    }
}
