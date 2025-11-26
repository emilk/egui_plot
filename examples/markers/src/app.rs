use eframe::egui;
use eframe::egui::Response;
use egui_plot::Legend;
use egui_plot::MarkerShape;
use egui_plot::Plot;
use egui_plot::Points;

pub struct MarkerDemo {
    fill_markers: bool,
    marker_radius: f32,
    automatic_colors: bool,
    marker_color: egui::Color32,
}

impl Default for MarkerDemo {
    fn default() -> Self {
        Self {
            fill_markers: true,
            marker_radius: 5.0,
            automatic_colors: true,
            marker_color: egui::Color32::GREEN,
        }
    }
}

impl MarkerDemo {
    fn markers<'a>(&self) -> Vec<Points<'a>> {
        MarkerShape::all()
            .enumerate()
            .map(|(i, marker)| {
                let y_offset = i as f64 * 0.5 + 1.0;
                let mut points = Points::new(
                    "marker",
                    vec![
                        [1.0, 0.0 + y_offset],
                        [2.0, 0.5 + y_offset],
                        [3.0, 0.0 + y_offset],
                        [4.0, 0.5 + y_offset],
                        [5.0, 0.0 + y_offset],
                        [6.0, 0.5 + y_offset],
                    ],
                )
                .id(format!("marker_{i}"))
                .name(format!("{marker:?}"))
                .filled(self.fill_markers)
                .radius(self.marker_radius)
                .shape(marker);

                if !self.automatic_colors {
                    points = points.color(self.marker_color);
                }

                points
            })
            .collect()
    }

    pub fn show_plot(&self, ui: &mut egui::Ui) -> Response {
        let markers_plot = Plot::new("markers_demo")
            .data_aspect(1.0)
            .legend(Legend::default().title("Markers"));
        markers_plot
            .show(ui, |plot_ui| {
                for marker in self.markers() {
                    plot_ui.points(marker);
                }
            })
            .response
    }

    pub fn show_controls(&mut self, ui: &mut egui::Ui) -> Response {
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.fill_markers, "Fill");
            ui.add(
                egui::DragValue::new(&mut self.marker_radius)
                    .speed(0.1)
                    .range(0.0..=f64::INFINITY)
                    .prefix("Radius: "),
            );
            ui.checkbox(&mut self.automatic_colors, "Automatic colors");
            if !self.automatic_colors {
                ui.color_edit_button_srgba(&mut self.marker_color);
            }
        })
        .response
    }
}
