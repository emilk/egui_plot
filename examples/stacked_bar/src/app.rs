use eframe::egui;
use eframe::egui::Response;
use eframe::egui::ScrollArea;
use egui_plot::Bar;
use egui_plot::BarChart;
use egui_plot::Legend;
use egui_plot::Plot;

pub struct StackedBarExample {
    vertical: bool,
}

impl Default for StackedBarExample {
    fn default() -> Self {
        Self { vertical: true }
    }
}

impl StackedBarExample {
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
        let mut chart1 = BarChart::new(
            "chart1",
            vec![
                Bar::new(0.5, 1.0).name("Day 1"),
                Bar::new(1.5, 3.0).name("Day 2"),
                Bar::new(2.5, 1.0).name("Day 3"),
                Bar::new(3.5, 2.0).name("Day 4"),
                Bar::new(4.5, 4.0).name("Day 5"),
            ],
        )
        .width(0.7)
        .name("Set 1");

        let mut chart2 = BarChart::new(
            "chart2",
            vec![
                Bar::new(0.5, 1.0),
                Bar::new(1.5, 1.5),
                Bar::new(2.5, 0.1),
                Bar::new(3.5, 0.7),
                Bar::new(4.5, 0.8),
            ],
        )
        .width(0.7)
        .name("Set 2")
        .stack_on(&[&chart1]);

        let mut chart3 = BarChart::new(
            "chart3",
            vec![
                Bar::new(0.5, -0.5),
                Bar::new(1.5, 1.0),
                Bar::new(2.5, 0.5),
                Bar::new(3.5, -1.0),
                Bar::new(4.5, 0.3),
            ],
        )
        .width(0.7)
        .name("Set 3")
        .stack_on(&[&chart1, &chart2]);

        let mut chart4 = BarChart::new(
            "chart4",
            vec![
                Bar::new(0.5, 0.5),
                Bar::new(1.5, 1.0),
                Bar::new(2.5, 0.5),
                Bar::new(3.5, -0.5),
                Bar::new(4.5, -0.5),
            ],
        )
        .width(0.7)
        .name("Set 4")
        .stack_on(&[&chart1, &chart2, &chart3]);

        if !self.vertical {
            chart1 = chart1.horizontal();
            chart2 = chart2.horizontal();
            chart3 = chart3.horizontal();
            chart4 = chart4.horizontal();
        }

        Plot::new("Stacked Bar Chart Demo")
            .legend(Legend::default())
            .data_aspect(1.0)
            .show(ui, |plot_ui| {
                plot_ui.bar_chart(chart1);
                plot_ui.bar_chart(chart2);
                plot_ui.bar_chart(chart3);
                plot_ui.bar_chart(chart4);
            })
            .response
    }
}
