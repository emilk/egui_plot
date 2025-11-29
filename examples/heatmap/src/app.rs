use eframe::egui;
use eframe::egui::Color32;
use eframe::egui::Response;
use eframe::egui::vec2;
use egui_plot::Legend;
use egui_plot::Plot;

pub const TURBO_COLORMAP: [Color32; 10] = [
    Color32::from_rgb(48, 18, 59),
    Color32::from_rgb(35, 106, 141),
    Color32::from_rgb(30, 160, 140),
    Color32::from_rgb(88, 200, 98),
    Color32::from_rgb(164, 223, 39),
    Color32::from_rgb(228, 223, 14),
    Color32::from_rgb(250, 187, 13),
    Color32::from_rgb(246, 135, 8),
    Color32::from_rgb(213, 68, 2),
    Color32::from_rgb(122, 4, 2),
];

pub struct HeatmapDemo {
    tick: f64,
    animate: bool,
    show_labels: bool,
    palette: Vec<Color32>,
    rows: usize,
    cols: usize,
}

impl Default for HeatmapDemo {
    fn default() -> Self {
        Self {
            tick: 0.0,
            animate: false,
            show_labels: true,
            palette: TURBO_COLORMAP.to_vec(),
            rows: 10,
            cols: 15,
        }
    }
}

impl HeatmapDemo {
    pub fn show_controls(&mut self, ui: &mut egui::Ui) -> Response {
        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.checkbox(&mut self.animate, "Animate");
                    if self.animate {
                        ui.ctx().request_repaint();
                        self.tick += 1.0;
                    }
                    ui.checkbox(&mut self.show_labels, "Show labels");
                });
            });
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.add(egui::Slider::new(&mut self.rows, 1..=100).text("Rows"));
                    ui.add(egui::Slider::new(&mut self.cols, 1..=100).text("Columns"));
                });
            });
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.add_enabled_ui(self.palette.len() > 1, |ui| {
                        if ui.button("Pop color").clicked() {
                            self.palette.pop();
                        }
                    });
                    if ui.button("Push color").clicked() {
                        self.palette.push(*self.palette.last().expect("Palette is empty"));
                    }
                });
                ui.horizontal(|ui| {
                    for color in &mut self.palette {
                        ui.color_edit_button_srgba(color);
                    }
                })
            })
        })
        .response
    }

    #[expect(clippy::needless_pass_by_ref_mut)]
    pub fn show_plot(&mut self, ui: &mut egui::Ui) -> Response {
        let mut values = Vec::new();
        for y in 0..self.rows {
            for x in 0..self.cols {
                let y = y as f64;
                let x = x as f64;
                let cols = self.cols as f64;
                let rows = self.rows as f64;
                values.push(((x + self.tick) / rows).sin() + ((y + self.tick) / cols).cos());
            }
        }

        let heatmap = egui_plot::Heatmap::<128>::new(values, self.cols)
            .expect("Failed to create heatmap")
            .palette(&self.palette)
            .highlight(true)
            .show_labels(self.show_labels);

        Plot::new("Heatmap Demo")
            .legend(Legend::default())
            .allow_zoom(false)
            .allow_scroll(false)
            .allow_drag(false)
            .allow_axis_zoom_drag(false)
            .allow_boxed_zoom(false)
            .set_margin_fraction(vec2(0.0, 0.0))
            .show(ui, |plot_ui| {
                plot_ui.heatmap(heatmap);
            })
            .response
    }
}
