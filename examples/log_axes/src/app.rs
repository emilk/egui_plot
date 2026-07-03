
use eframe::egui;
use eframe::egui::Response;
use egui::NumExt as _;
use egui_plot::AxisScale;
use egui_plot::Legend;
use egui_plot::Line;
use egui_plot::LineStyle;
use egui_plot::Plot;
use egui_plot::PlotPoints;

#[derive(Clone, Copy, PartialEq)]
pub struct LogScaleExample {
    invert_x: bool,
    invert_y: bool,
    log_x: bool,
    log_y: bool,
}

impl Default for LogScaleExample {
    fn default() -> Self {
        Self {
            invert_x: false,
            invert_y: false,
            log_x: false,
            log_y: false,
        }
    }
}

impl LogScaleExample {
    pub fn show_controls(&mut self, ui: &mut egui::Ui) -> Response {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.checkbox(&mut self.invert_x, "Invert X axis");
                ui.checkbox(&mut self.invert_y, "Invert Y axis");
                ui.checkbox(&mut self.log_x, "Log X axis");
                ui.checkbox(&mut self.log_y, "Log Y axis");
            });
        })
        .response
    }

    fn squared_line(&self) -> Line<'_> {
        Line::new("exponential", PlotPoints::from_explicit_callback(|x| 10.0_f64.powf(x), 0.0..10.0, 512))
            .color(egui::Color32::from_rgb(100, 150, 250))
            .style(LineStyle::Solid)
    }

    pub fn show_plot(&mut self, ui: &mut egui::Ui) -> Response {
        let mut plot = Plot::new("lines_demo")
            .legend(Legend::default().title("Lines"))
            .invert_x(self.invert_x)
            .invert_y(self.invert_y)
            .x_scale(if self.log_x { AxisScale::Log10 } else { AxisScale::Linear })
            .y_scale(if self.log_y { AxisScale::Log10 } else { AxisScale::Linear });
        plot.show(ui, |plot_ui| {
            plot_ui.line(self.squared_line());
        })
        .response
    }
}
