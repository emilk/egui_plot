use eframe::egui;
use eframe::egui::DragValue;
use eframe::egui::Event;
use eframe::egui::Response;
use eframe::egui::Vec2;
use egui_plot::Legend;
use egui_plot::Line;
use egui_plot::PlotPoints;

pub struct CustomPlotManipulationExample {
    lock_x: bool,
    lock_y: bool,
    ctrl_to_zoom: bool,
    shift_to_horizontal: bool,
    zoom_speed: f32,
    scroll_speed: f32,
}

impl Default for CustomPlotManipulationExample {
    fn default() -> Self {
        Self {
            lock_x: false,
            lock_y: false,
            ctrl_to_zoom: false,
            shift_to_horizontal: false,
            zoom_speed: 1.0,
            scroll_speed: 1.0,
        }
    }
}

impl CustomPlotManipulationExample {
    pub fn show_plot(&self, ui: &mut egui::Ui) -> Response {
        let (scroll, pointer_down, modifiers) = ui.ctx().input(|i| {
            let scroll = i.events.iter().find_map(|e| match e {
                Event::MouseWheel {
                    unit: _,
                    delta,
                    modifiers: _,
                } => Some(*delta),
                _ => None,
            });
            (scroll, i.pointer.primary_down(), i.modifiers)
        });

        egui_plot::Plot::new("plot")
            .allow_zoom(false)
            .allow_drag(false)
            .allow_scroll(false)
            .invert_x(false)
            .invert_y(true)
            .legend(Legend::default())
            .show(ui, |plot_ui| {
                if let Some(mut scroll) = scroll {
                    if modifiers.ctrl == self.ctrl_to_zoom {
                        scroll = Vec2::splat(scroll.x + scroll.y);
                        let mut zoom_factor = Vec2::from([
                            (scroll.x * self.zoom_speed / 10.0).exp(),
                            (scroll.y * self.zoom_speed / 10.0).exp(),
                        ]);
                        if self.lock_x {
                            zoom_factor.x = 1.0;
                        }
                        if self.lock_y {
                            zoom_factor.y = 1.0;
                        }
                        plot_ui.zoom_bounds_around_hovered(zoom_factor);
                    } else {
                        if modifiers.shift == self.shift_to_horizontal {
                            scroll = Vec2::new(scroll.y, scroll.x);
                        }
                        if self.lock_x {
                            scroll.x = 0.0;
                        }
                        if self.lock_y {
                            scroll.y = 0.0;
                        }
                        let delta_pos = self.scroll_speed * scroll;
                        plot_ui.translate_bounds(delta_pos);
                    }
                }
                if plot_ui.response().hovered() && pointer_down {
                    let mut pointer_translate = -plot_ui.pointer_coordinate_drag_delta();
                    if self.lock_x {
                        pointer_translate.x = 0.0;
                    }
                    if self.lock_y {
                        pointer_translate.y = 0.0;
                    }
                    plot_ui.translate_bounds(pointer_translate);
                }

                let sine_points = PlotPoints::from_explicit_callback(|x| x.sin(), .., 5000);
                let sine_line = Line::new("Sine", sine_points).name("Sine");

                plot_ui.line(sine_line);
            })
            .response
    }

    pub fn show_controls(&mut self, ui: &mut egui::Ui) -> Response {
        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.vertical(|ui| {
                ui.checkbox(&mut self.lock_x, "Lock x axis")
                    .on_hover_text("Check to keep the X axis fixed, i.e., pan and zoom will only affect the Y axis");
                ui.checkbox(&mut self.lock_y, "Lock y axis")
                    .on_hover_text("Check to keep the Y axis fixed, i.e., pan and zoom will only affect the X axis");
                });
            });
            ui.group(|ui| {
                ui.vertical(|ui| {
                ui.checkbox(&mut self.ctrl_to_zoom, "Ctrl to zoom").on_hover_text(
                    "If unchecked, the behavior of the Ctrl key is inverted compared to the default controls\ni.e., scrolling the mouse without pressing any keys zooms the plot",
                );
                ui.checkbox(&mut self.shift_to_horizontal, "Shift for horizontal scroll")
                    .on_hover_text(
                        "If unchecked, the behavior of the shift key is inverted compared to the default controls\ni.e., hold to scroll vertically, release to scroll horizontally",
                    );
                });
            });
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.add(DragValue::new(&mut self.zoom_speed).range(0.1..=2.0).speed(0.1));
                        ui.label("Zoom speed")
                            .on_hover_text("How fast to zoom in and out with the mouse wheel");
                    });
                    ui.horizontal(|ui| {
                        ui.add(DragValue::new(&mut self.scroll_speed).range(0.1..=100.0).speed(0.1));
                        ui.label("Scroll speed")
                            .on_hover_text("How fast to pan with the mouse wheel");
                    })
                });
            });
        })
        .response
    }
}
