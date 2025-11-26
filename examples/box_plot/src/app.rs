use eframe::egui;
use eframe::egui::Response;
use eframe::egui::ScrollArea;
use egui_plot::BoxElem;
use egui_plot::BoxPlot;
use egui_plot::BoxSpread;
use egui_plot::Legend;
use egui_plot::Plot;

#[derive(Default)]
pub struct BoxPlotExample {
    horizontal: bool,
}

impl BoxPlotExample {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> Response {
        ScrollArea::horizontal().show(ui, |ui| self.options_ui(ui));
        self.show_plot(ui)
    }

    fn options_ui(&mut self, ui: &mut egui::Ui) -> Response {
        ui.horizontal(|ui| {
            ui.label("Orientation:");
            ui.selectable_value(&mut self.horizontal, false, "Vertical");
            ui.selectable_value(&mut self.horizontal, true, "Horizontal");
        })
        .response
    }

    fn show_plot(&self, ui: &mut egui::Ui) -> Response {
        let yellow = egui::Color32::from_rgb(248, 252, 168);
        let mut box1 = BoxPlot::new(
            "Experiment A",
            vec![
                BoxElem::new(0.5, BoxSpread::new(1.5, 2.2, 2.5, 2.6, 3.1)).name("Day 1"),
                BoxElem::new(2.5, BoxSpread::new(0.4, 1.0, 1.1, 1.4, 2.1)).name("Day 2"),
                BoxElem::new(4.5, BoxSpread::new(1.7, 2.0, 2.2, 2.5, 2.9)).name("Day 3"),
            ],
        );

        let mut box2 = BoxPlot::new(
            "Experiment B",
            vec![
                BoxElem::new(1.0, BoxSpread::new(0.2, 0.5, 1.0, 2.0, 2.7)).name("Day 1"),
                BoxElem::new(3.0, BoxSpread::new(1.5, 1.7, 2.1, 2.9, 3.3))
                    .name("Day 2: interesting")
                    .stroke(egui::Stroke::new(1.5, yellow))
                    .fill(yellow.linear_multiply(0.2)),
                BoxElem::new(5.0, BoxSpread::new(1.3, 2.0, 2.3, 2.9, 4.0)).name("Day 3"),
            ],
        );

        let mut box3 = BoxPlot::new(
            "Experiment C",
            vec![
                BoxElem::new(1.5, BoxSpread::new(2.1, 2.2, 2.6, 2.8, 3.0)).name("Day 1"),
                BoxElem::new(3.5, BoxSpread::new(1.3, 1.5, 1.9, 2.2, 2.4)).name("Day 2"),
                BoxElem::new(5.5, BoxSpread::new(0.2, 0.4, 1.0, 1.3, 1.5)).name("Day 3"),
            ],
        );

        if self.horizontal {
            box1 = box1.horizontal();
            box2 = box2.horizontal();
            box3 = box3.horizontal();
        }

        Plot::new("Box Plot Demo")
            .legend(Legend::default())
            .allow_zoom(egui::Vec2b::new(false, false))
            .allow_drag(egui::Vec2b::new(false, false))
            .allow_scroll(egui::Vec2b::new(false, false))
            .show(ui, |plot_ui| {
                plot_ui.box_plot(box1);
                plot_ui.box_plot(box2);
                plot_ui.box_plot(box3);
            })
            .response
    }
}
