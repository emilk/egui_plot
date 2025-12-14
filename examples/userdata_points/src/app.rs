use eframe::egui;
use eframe::egui::Response;
use egui_plot::Legend;
use egui_plot::Line;
use egui_plot::Plot;

use egui::mutex::Mutex;
use egui::{Color32, Id};
use egui_plot::{Corner, MarkerShape, Points};
use std::collections::HashMap;
use std::sync::Arc;

pub struct UserdataPointsExample {
    sine_points: Vec<DemoPoint>,
    cosine_points: Vec<DemoPoint>,
    damped_points: Vec<DemoPoint>,
}

#[derive(Clone)]
struct DemoPoint {
    x: f64,
    y: f64,
    custom_label: String,
    importance: f32,
}

impl Default for UserdataPointsExample {
    fn default() -> Self {
        // Create multiple datasets with custom metadata
        let sine_points = (0..=500)
            .map(|i| {
                let x = i as f64 / 100.0;
                DemoPoint {
                    x,
                    y: x.sin(),
                    custom_label: format!("Sine #{i}"),
                    importance: (i % 100) as f32 / 100.0,
                }
            })
            .collect::<Vec<_>>();
        let cosine_points = (0..=500)
            .map(|i| {
                let x = i as f64 / 100.0;
                DemoPoint {
                    x,
                    y: x.cos(),
                    custom_label: format!("Cosine #{i}"),
                    importance: (1.0 - (i % 100) as f32 / 100.0),
                }
            })
            .collect::<Vec<_>>();

        let damped_points = (0..=500)
            .map(|i| {
                let x = i as f64 / 100.0;
                DemoPoint {
                    x,
                    y: (-x * 0.5).exp() * (2.0 * x).sin(),
                    custom_label: format!("Damped #{i}"),
                    importance: if i % 50 == 0 { 1.0 } else { 0.3 },
                }
            })
            .collect::<Vec<_>>();
        Self {
            sine_points,
            cosine_points,
            damped_points,
        }
    }
}

impl UserdataPointsExample {
    pub fn show_plot(&self, ui: &mut egui::Ui) -> Response {
        let custom_data = Arc::new(Mutex::new(HashMap::<Id, Vec<DemoPoint>>::new()));

        let custom_data_ = custom_data.clone();
        Plot::new("Userdata Plot Demo")
            .legend(Legend::default().position(Corner::LeftTop))
            .label_formatter(move |name, value, item| {
                if let Some((id, index)) = item {
                    let lock = custom_data_.lock();
                    if let Some(points) = lock.get(&id) {
                        if let Some(point) = points.get(index) {
                            return format!(
                                "{}\nPosition: ({:.3}, {:.3})\nLabel: {}\nImportance: {:.1}%",
                                name,
                                value.x,
                                value.y,
                                point.custom_label,
                                point.importance * 100.0
                            );
                        }
                    }
                }
                format!("{}\n({:.3}, {:.3})", name, value.x, value.y)
            })
            .show(ui, |plot_ui| {
                let mut lock = custom_data.lock();

                // Sine wave with custom data
                let sine_id = Id::new("sine_wave");
                lock.insert(sine_id, self.sine_points.clone());
                plot_ui.line(
                    Line::new(
                        "sin(x)",
                        self.sine_points.iter().map(|p| [p.x, p.y]).collect::<Vec<_>>(),
                    )
                    .id(sine_id)
                    .color(Color32::from_rgb(200, 100, 100)),
                );

                // Cosine wave with custom data
                let cosine_id = Id::new("cosine_wave");
                lock.insert(cosine_id, self.cosine_points.clone());
                plot_ui.line(
                    Line::new(
                        "cos(x)",
                        self.cosine_points.iter().map(|p| [p.x, p.y]).collect::<Vec<_>>(),
                    )
                    .id(cosine_id)
                    .color(Color32::from_rgb(100, 200, 100)),
                );

                // Damped sine wave with custom data
                let damped_id = Id::new("damped_wave");
                lock.insert(damped_id, self.damped_points.clone());
                plot_ui.line(
                    Line::new(
                        "e^(-0.5x) · sin(2x)",
                        self.damped_points.iter().map(|p| [p.x, p.y]).collect::<Vec<_>>(),
                    )
                    .id(damped_id)
                    .color(Color32::from_rgb(100, 100, 200)),
                );

                // Add some points with high importance as markers
                let important_points: Vec<_> = self
                    .damped_points
                    .iter()
                    .filter(|p| p.importance > 0.9)
                    .map(|p| [p.x, p.y])
                    .collect();

                if !important_points.is_empty() {
                    plot_ui.points(
                        Points::new("Important Points", important_points)
                            .color(Color32::from_rgb(255, 150, 0))
                            .radius(4.0)
                            .shape(MarkerShape::Diamond),
                    );
                }
            })
            .response
    }

    #[expect(clippy::unused_self, reason = "required by the example template")]
    pub fn show_controls(&self, ui: &mut egui::Ui) -> Response {
        ui.scope(|_ui| {}).response
    }
}
