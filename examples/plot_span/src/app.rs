use eframe::egui::{self, Align2, Color32, Response};
use eframe::epaint::Hsva;
use egui_plot::{Corner, Legend, Line, Plot, PlotPoints, Span};

#[derive(Default)]
pub struct PlotSpanDemo {
    show_legend: bool,
}

impl PlotSpanDemo {
    pub fn show_controls(&mut self, ui: &mut egui::Ui) -> Response {
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.show_legend, "Show legend");
        })
        .response
    }

    pub fn show_plot(&mut self, ui: &mut egui::Ui) -> Response {
        let mut plot = Plot::new("Span Demo");
        if self.show_legend {
            plot = plot.legend(Legend::default().position(Corner::LeftBottom));
        }

        plot.show(ui, |plot_ui| {
            let span = Span::new("Span 1", -10.0..=-5.0)
                .border_style(egui_plot::LineStyle::Dashed { length: 50.0 })
                .border_width(3.0);
            plot_ui.span(span);

            let span = Span::new("Span 2", 0.0..=1.0);
            plot_ui.span(span);

            let span = Span::new("Span 3", 5.0..=6.0).axis(egui_plot::Axis::Y);
            plot_ui.span(span);

            let color4: Color32 = Hsva::new(0.1, 0.85, 0.5, 0.15).into();
            let span4 = Span::new("Span 4", 5.0..=5.5)
                .border_width(0.0)
                .fill(color4)
                .label_align(Align2::LEFT_BOTTOM);
            plot_ui.span(span4.clone());

            let color5: Color32 = Hsva::new(0.3, 0.85, 0.5, 0.15).into();
            let span5 = Span::new("Span 5", 5.5..=6.5)
                .border_width(0.0)
                .fill(color5)
                .label_align(Align2::LEFT_BOTTOM);
            plot_ui.span(span5.clone());

            let span = span4.clone().range(6.5..=8.0);
            plot_ui.span(span);

            let span = span5.clone().range(8.0..=10.0);
            plot_ui.span(span);

            let span = Span::new("Infinite span", 10.0..=f64::INFINITY);
            plot_ui.span(span);

            let sine_points = PlotPoints::from_explicit_callback(|x| x.sin(), .., 5000);
            let sine_line = Line::new("Sine", sine_points).name("Sine");

            plot_ui.line(sine_line);
        })
        .response
    }
}

