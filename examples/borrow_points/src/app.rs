use eframe::egui;
use eframe::egui::Response;
use egui_plot::Legend;
use egui_plot::Line;
use egui_plot::Plot;
use egui_plot::PlotPoint;
use egui_plot::PlotPoints;

pub struct BorrowPointsExample {
    points: Vec<PlotPoint>,
}

impl Default for BorrowPointsExample {
    fn default() -> Self {
        let points: Vec<[f64; 2]> = vec![[0.0, 1.0], [2.0, 3.0], [3.0, 2.0]];
        let points = points.iter().map(|p| PlotPoint::new(p[0], p[1])).collect();
        Self { points }
    }
}

impl BorrowPointsExample {
    pub fn show_plot(&self, ui: &mut egui::Ui) -> Response {
        Plot::new("My Plot")
            .legend(Legend::default())
            .show(ui, |plot_ui| {
                plot_ui.line(Line::new("curve", PlotPoints::Borrowed(&self.points)).name("curve"));
            })
            .response
    }

    #[expect(clippy::unused_self)]
    pub fn show_controls(&self, ui: &mut egui::Ui) -> Response {
        ui.scope(|_ui| {}).response
    }
}
