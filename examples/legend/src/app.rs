use eframe::egui;
use eframe::egui::Response;
use egui_plot::Corner;
use egui_plot::Legend;
use egui_plot::Line;
use egui_plot::Plot;
use egui_plot::PlotPoints;

#[derive(Default)]
pub struct LegendExample {
    config: Legend,
}

impl LegendExample {
    fn line_with_slope<'a>(slope: f64) -> Line<'a> {
        Line::new(
            "line with slope",
            PlotPoints::from_explicit_callback(move |x| slope * x, .., 100),
        )
    }

    fn sin<'a>() -> Line<'a> {
        Line::new("sin(x)", PlotPoints::from_explicit_callback(move |x| x.sin(), .., 100))
    }

    fn cos<'a>() -> Line<'a> {
        Line::new("cos(x)", PlotPoints::from_explicit_callback(move |x| x.cos(), .., 100))
    }

    pub fn show_plot(&mut self, ui: &mut egui::Ui) -> Response {
        let Self { config } = self;
        let legend_plot = Plot::new("legend_demo").legend(config.clone()).data_aspect(1.0);
        legend_plot
            .show(ui, |plot_ui| {
                plot_ui.line(Self::line_with_slope(0.5).name("lines"));
                plot_ui.line(Self::line_with_slope(1.0).name("lines"));
                plot_ui.line(Self::line_with_slope(2.0).name("lines"));
                plot_ui.line(Self::sin().name("sin(x)"));
                plot_ui.line(Self::cos().name("cos(x)"));
            })
            .response
    }

    pub fn show_controls(&mut self, ui: &mut egui::Ui) -> Response {
        let Self { config } = self;
        egui::Grid::new("settings")
            .show(ui, |ui| {
                ui.label("Text style:");
                ui.horizontal(|ui| {
                    let all_text_styles = ui.style().text_styles();
                    for style in all_text_styles {
                        ui.selectable_value(&mut config.text_style, style.clone(), style.to_string());
                    }
                });
                ui.end_row();

                ui.label("Position:");
                ui.horizontal(|ui| {
                    Corner::all().for_each(|position| {
                        ui.selectable_value(&mut config.position, position, format!("{position:?}"));
                    });
                });
                ui.end_row();

                ui.label("Opacity:");
                ui.add(
                    egui::DragValue::new(&mut config.background_alpha)
                        .speed(0.02)
                        .range(0.0..=1.0),
                );
                ui.end_row();
            })
            .response
    }
}
