use eframe::egui;
use eframe::egui::Response;
use eframe::egui::ScrollArea;
use egui_plot::Bar;
use egui_plot::BarChart;
use egui_plot::Legend;
use egui_plot::Plot;

pub struct HistogramExample {
    vertical: bool,
}

impl Default for HistogramExample {
    fn default() -> Self {
        Self { vertical: true }
    }
}

impl HistogramExample {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> Response {
        ScrollArea::horizontal().show(ui, |ui| self.options_ui(ui));
        self.show_plot(ui)
    }

    fn options_ui(&mut self, ui: &mut egui::Ui) -> Response {
        ui.horizontal(|ui| {
            ui.label("Orientation:");
            ui.selectable_value(&mut self.vertical, true, "Vertical");
            ui.selectable_value(&mut self.vertical, false, "Horizontal");
        })
        .response
    }

    fn show_plot(&self, ui: &mut egui::Ui) -> Response {
        let mut chart = BarChart::new(
            "Normal Distribution",
            (-395..=395)
                .step_by(10)
                .map(|x| x as f64 * 0.01)
                .map(|x| (x, (-x * x / 2.0).exp() / (2.0 * std::f64::consts::PI).sqrt()))
                .map(|(x, f)| Bar::new(x, f * 10.0).width(0.1))
                .collect(),
        )
        .color(egui::Color32::LIGHT_BLUE);

        if !self.vertical {
            chart = chart.horizontal();
        }

        Plot::new("Normal Distribution Demo")
            .legend(Legend::default())
            .clamp_grid(true)
            .allow_zoom(egui::Vec2b::new(true, true))
            .allow_drag(egui::Vec2b::new(true, true))
            .allow_scroll(egui::Vec2b::new(true, true))
            .show(ui, |plot_ui| plot_ui.bar_chart(chart))
            .response
    }
}
