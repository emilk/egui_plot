use eframe::egui;
use eframe::egui::Response;
use egui_plot::Line;
use egui_plot::Plot;
use egui_plot::PlotPoint;
use examples_utils::random_walk;

const NUM_WALKS: usize = 10;
const DEFAULT_VALUES_PER_WALK: usize = 20_000;

pub struct PerformanceLinesDemo {
    values_per_walk: usize,
    walks: Vec<Vec<PlotPoint>>,
}

impl Default for PerformanceLinesDemo {
    fn default() -> Self {
        Self {
            values_per_walk: DEFAULT_VALUES_PER_WALK,
            walks: make_walks(DEFAULT_VALUES_PER_WALK),
        }
    }
}

fn make_walks(values_per_walk: usize) -> Vec<Vec<PlotPoint>> {
    (0..NUM_WALKS)
        .map(|walk_index| random_walk(walk_index as u64 + 1, values_per_walk))
        .collect()
}

impl PerformanceLinesDemo {
    pub fn show_plot(&self, ui: &mut egui::Ui) -> Response {
        Plot::new("performance_lines_demo")
            .show(ui, |plot_ui| {
                for (walk_index, walk) in self.walks.iter().enumerate() {
                    plot_ui.line(Line::new(format!("Walk {walk_index}"), walk.as_slice()));
                }
            })
            .response
    }

    pub fn show_controls(&mut self, ui: &mut egui::Ui) -> Response {
        ui.request_repaint(); // Continuous repaint for FPS counter
        let fps = (1.0 / ui.input(|i| i.stable_dt)).round();

        ui.horizontal(|ui| {
            ui.label("Values per walk:");
            if ui
                .add(
                    egui::DragValue::new(&mut self.values_per_walk)
                        .speed(100)
                        .range(1..=10_000_000),
                )
                .changed()
            {
                self.walks = make_walks(self.values_per_walk);
            }

            ui.label(format!("FPS: {fps}"));
        });

        ui.label(format!("{NUM_WALKS} random walks are shown."));
        ui.response()
    }
}
