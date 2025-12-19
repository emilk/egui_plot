use eframe::egui;
use eframe::egui::Response;
use egui_plot::Legend;
use egui_plot::Line;
use egui_plot::Plot;
use egui_plot::PlotPoints;

pub struct LegendSortExample {
    insert_order: bool,
    graph: Vec<[f64; 2]>,
    graph2: Vec<[f64; 2]>,
    graph3: Vec<[f64; 2]>,
}

impl Default for LegendSortExample {
    fn default() -> Self {
        Self {
            insert_order: false,
            graph: vec![[0.0, 1.0], [2.0, 3.0], [3.0, 2.0]],
            graph2: vec![[0.0, 2.0], [2.0, 4.0], [3.0, 3.0]],
            graph3: vec![[0.0, 3.0], [2.0, 5.0], [3.0, 4.0]],
        }
    }
}

impl LegendSortExample {
    pub fn show_plot(&self, ui: &mut egui::Ui) -> Response {
        Plot::new("My Plot")
            .legend(Legend::default().follow_insertion_order(self.insert_order))
            .show(ui, |plot_ui| {
                plot_ui.line(Line::new("3rd Curve", PlotPoints::from(self.graph3.clone())));
                plot_ui.line(Line::new("1st Curve", PlotPoints::from(self.graph.clone())));
                plot_ui.line(Line::new("2nd Curve", PlotPoints::from(self.graph2.clone())));
            })
            .response
    }

    pub fn show_controls(&mut self, ui: &mut egui::Ui) -> Response {
        ui.horizontal(|ui| {
            ui.label("If checked the legend will follow the order as the curves are inserted");
            ui.checkbox(&mut self.insert_order, "Insert order");
        })
        .response
    }
}
