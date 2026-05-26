use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints, log_formatter_computer, log_formatter_engineering, log_formatter_superscript};

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Log Scale Plot Example",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(PartialEq, Default)]
enum LogFormatter {
    #[default]
    Superscript,
    Computer,
    Engineering,
}

#[derive(Default)]
struct MyApp {
    log_x: bool,
    log_y: bool,
    log_axis_formatter: LogFormatter,
}

impl eframe::App for MyApp {
    fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Log Scale Plot Example");

            ui.horizontal(|ui| {
                ui.label("This example demonstrates logarithmic axis scaling.");
            });

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.log_x, "Logarithmic X-axis");
                ui.checkbox(&mut self.log_y, "Logarithmic Y-axis");
            });

            ui.horizontal(|ui| {
                ui.label("Log-axis formatter:");
                ui.radio_value(
                    &mut self.log_axis_formatter,
                    LogFormatter::Superscript,
                    "Superscript (10², 1/10²)",
                );
                ui.radio_value(&mut self.log_axis_formatter, LogFormatter::Computer, "Computer (1e2)");
                ui.radio_value(
                    &mut self.log_axis_formatter,
                    LogFormatter::Engineering,
                    "Engineering (1K, 1M)",
                );
            });

            ui.separator();

            // Generate exponential data (y = 10^x)
            // X from 0.1 to 10 (suitable for log scale)
            let exponential: PlotPoints<'_> = (1..=100)
                .map(|i| {
                    let x = i as f64 * 0.1;
                    let y = 10_f64.powf(x);
                    [x, y]
                })
                .collect();

            // Generate power law data (y = x^3)
            // X from 1 to 100 (suitable for log scale)
            let power_law: PlotPoints<'_> = (1..=100)
                .map(|i| {
                    let x = i as f64;
                    let y = x.powi(3);
                    [x, y]
                })
                .collect();

            let mut plot = Plot::new("log_plot")
                .height(400.0)
                .legend(egui_plot::Legend::default())
                .allow_double_click_reset(true);

            if self.log_x {
                plot = plot.log_x();
                plot = match self.log_axis_formatter {
                    LogFormatter::Superscript => plot.x_axis_formatter(log_formatter_superscript()),
                    LogFormatter::Computer => plot.x_axis_formatter(log_formatter_computer()),
                    LogFormatter::Engineering => plot.x_axis_formatter(log_formatter_engineering()),
                };
            }
            if self.log_y {
                plot = plot.log_y();
                // Override default formatter based on selection
                plot = match self.log_axis_formatter {
                    LogFormatter::Superscript => plot.y_axis_formatter(log_formatter_superscript()),
                    LogFormatter::Computer => plot.y_axis_formatter(log_formatter_computer()),
                    LogFormatter::Engineering => plot.y_axis_formatter(log_formatter_engineering()),
                };
            }

            plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new("y = 10^x", exponential).color(egui::Color32::from_rgb(200, 100, 100)));
                plot_ui.line(Line::new("y = x³", power_law).color(egui::Color32::from_rgb(100, 150, 250)));
            });

            ui.separator();
            ui.label("💡 Interaction:");
            ui.label("  - Try enabling logarithmic Y-axis to see exponential data linearized");
            ui.label("  - Try enabling logarithmic X-axis to see power law relationships");
            ui.label("  - Both axes can be logarithmic simultaneously");
            ui.label("  - Double-click the plot to reset zoom to extents");
            ui.label("  - Try different formatters to see how labels are displayed");
        });
    }
}
