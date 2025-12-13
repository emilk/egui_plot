use std::f64::consts::TAU;
use std::sync::Arc;

use eframe::egui;
use eframe::egui::Checkbox;
use eframe::egui::ComboBox;
use eframe::egui::Pos2;
use eframe::egui::Response;
use eframe::egui::TextWrapMode;
use egui::NumExt as _;
use egui_plot::CoordinatesFormatter;
use egui_plot::Corner;
use egui_plot::Legend;
use egui_plot::Line;
use egui_plot::LineStyle;
use egui_plot::Plot;
use egui_plot::PlotPoints;
use egui_plot::default_label_formatter;

#[derive(Clone, Copy, PartialEq)]
pub struct LineExample {
    animate: bool,
    time: f64,
    circle_radius: f64,
    circle_center: Pos2,
    square: bool,
    proportional: bool,
    coordinates: bool,
    show_axes: bool,
    show_grid: bool,
    show_crosshair: bool,
    show_labels: bool,
    line_style: LineStyle,
    gradient: bool,
    gradient_fill: bool,
    invert_x: bool,
    invert_y: bool,
}

impl Default for LineExample {
    fn default() -> Self {
        Self {
            animate: !cfg!(debug_assertions),
            time: 0.0,
            circle_radius: 1.5,
            circle_center: Pos2::ZERO,
            square: false,
            proportional: true,
            coordinates: true,
            show_axes: true,
            show_grid: true,
            show_crosshair: true,
            show_labels: true,
            line_style: LineStyle::Solid,
            gradient: false,
            gradient_fill: false,
            invert_x: false,
            invert_y: false,
        }
    }
}

impl LineExample {
    pub fn show_controls(&mut self, ui: &mut egui::Ui) -> Response {
        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Circle:");
                    ui.add(
                        egui::DragValue::new(&mut self.circle_radius)
                            .speed(0.1)
                            .range(0.0..=f64::INFINITY)
                            .prefix("r: "),
                    );
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut self.circle_center.x).speed(0.1).prefix("x: "));
                        ui.add(egui::DragValue::new(&mut self.circle_center.y).speed(1.0).prefix("y: "));
                    });
                })
            });
            ui.vertical(|ui| {
                ui.checkbox(&mut self.show_axes, "Show axes");
                ui.checkbox(&mut self.show_grid, "Show grid");
                ui.checkbox(&mut self.show_crosshair, "Show crosshair");
                ui.checkbox(&mut self.coordinates, "Show coordinates on hover")
                    .on_hover_text("Can take a custom formatting function.");
                ui.checkbox(&mut self.show_labels, "Show hover labels")
                    .on_hover_text("Show labels when hovering over data points.");
            });
            ui.vertical(|ui| {
                ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
                ui.checkbox(&mut self.animate, "Animate");
                ui.checkbox(&mut self.square, "Square view")
                    .on_hover_text("Always keep the viewport square.");
                ui.checkbox(&mut self.proportional, "Proportional data axes")
                    .on_hover_text("Tick are the same size on both axes.");
                ComboBox::from_label("Line style")
                    .selected_text(self.line_style.to_string())
                    .show_ui(ui, |ui| {
                        for style in [
                            LineStyle::Solid,
                            LineStyle::dashed_dense(),
                            LineStyle::dashed_loose(),
                            LineStyle::dotted_dense(),
                            LineStyle::dotted_loose(),
                        ] {
                            ui.selectable_value(&mut self.line_style, style, style.to_string());
                        }
                    });
            });
            ui.vertical(|ui| {
                ui.checkbox(&mut self.gradient, "Gradient line");
                ui.add_enabled(self.gradient, Checkbox::new(&mut self.gradient_fill, "Gradient fill"));
            });
            ui.vertical(|ui| {
                ui.checkbox(&mut self.invert_x, "Invert X axis");
                ui.checkbox(&mut self.invert_y, "Invert Y axis");
            });
        })
        .response
    }

    fn circle(&self) -> Line<'_> {
        let n = 512;
        let points: PlotPoints<'_> = (0..=n)
            .map(|i| {
                let t = egui::remap(i as f64, 0.0..=(n as f64), 0.0..=TAU);
                [
                    self.circle_radius * t.cos() + self.circle_center.x as f64,
                    self.circle_radius * t.sin() + self.circle_center.y as f64,
                ]
            })
            .collect();
        Line::new("circle", points)
            .color(egui::Color32::from_rgb(100, 200, 100))
            .style(self.line_style)
    }

    fn sin(&self) -> Line<'_> {
        Line::new(
            "wave",
            PlotPoints::from_explicit_callback(move |x| 0.5 * (2.0 * x).sin() * self.time.sin(), .., 512),
        )
        .color(egui::Color32::from_rgb(200, 100, 100))
        .style(self.line_style)
    }

    fn thingy(&self) -> Line<'_> {
        let mut line = Line::new(
            "parametric",
            PlotPoints::from_parametric_callback(
                move |t| ((2.0 * t + self.time).sin(), (3. * t).sin()),
                0.0..=TAU,
                256,
            ),
        )
        .style(self.line_style);
        if self.gradient {
            line = line.gradient_color(
                Arc::new(|p| egui::Color32::BLUE.lerp_to_gamma(egui::Color32::ORANGE, p.x.abs().clamp(0., 1.) as f32)),
                self.gradient_fill,
            );
            if self.gradient_fill {
                line = line.fill(0.);
            }
        } else {
            line = line.color(egui::Color32::from_rgb(100, 150, 250));
        }
        line
    }

    pub fn show_plot(&mut self, ui: &mut egui::Ui) -> Response {
        if self.animate {
            ui.ctx().request_repaint();
            self.time += ui.input(|i| i.unstable_dt).at_most(1.0 / 30.0) as f64;
        }

        let mut plot = Plot::new("lines_demo")
            .legend(Legend::default().title("Lines"))
            .show_axes(self.show_axes)
            .show_grid(self.show_grid)
            .show_crosshair(self.show_crosshair)
            .invert_x(self.invert_x)
            .invert_y(self.invert_y);
        if self.square {
            plot = plot.view_aspect(1.0);
        }
        if self.proportional {
            plot = plot.data_aspect(1.0);
        }
        if self.coordinates {
            plot = plot.coordinates_formatter(Corner::LeftBottom, CoordinatesFormatter::default());
        }
        if self.show_labels {
            plot = plot.label_formatter(default_label_formatter);
        }
        plot.show(ui, |plot_ui| {
            plot_ui.line(self.circle());
            plot_ui.line(self.sin());
            plot_ui.line(self.thingy());
        })
        .response
    }
}
