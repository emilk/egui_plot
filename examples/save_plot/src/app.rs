use eframe::egui;
use eframe::egui::Response;
use egui_plot::Legend;
use egui_plot::Line;
use egui_plot::Plot;
use egui_plot::PlotPoints;

#[derive(Default)]
pub struct SavePlotExample {
    plot_rect: Option<egui::Rect>,
}

impl SavePlotExample {
    pub fn show_plot(&mut self, ui: &mut egui::Ui) -> Response {
        let my_plot = Plot::new("My Plot").legend(Legend::default());

        // let's create a dummy line in the plot
        let graph: Vec<[f64; 2]> = vec![[0.0, 1.0], [2.0, 3.0], [3.0, 2.0]];
        let inner = my_plot.show(ui, |plot_ui| {
            plot_ui.line(Line::new("curve", PlotPoints::from(graph)));
        });
        // Remember the position of the plot
        self.plot_rect = Some(inner.response.rect);

        #[cfg(not(target_arch = "wasm32"))]
        {
            // Check for returned screenshot:
            let ctx = ui.ctx();
            let screenshot = ctx.input(|i| {
                for event in &i.raw.events {
                    if let egui::Event::Screenshot { image, .. } = event {
                        return Some(image.clone());
                    }
                }
                None
            });
            if let (Some(screenshot), Some(plot_location)) = (screenshot, self.plot_rect) {
                if let Some(mut path) = rfd::FileDialog::new().save_file() {
                    path.set_extension("png");

                    // for a full size application, we should put this in a different thread,
                    // so that the GUI doesn't lag during saving

                    let pixels_per_point = ctx.pixels_per_point();
                    let plot = screenshot.region(&plot_location, Some(pixels_per_point));
                    // save the plot to png
                    let result = image::save_buffer(
                        &path,
                        plot.as_raw(),
                        plot.width() as u32,
                        plot.height() as u32,
                        image::ColorType::Rgba8,
                    );
                    match result {
                        Ok(()) => eprintln!("Image saved to {}", path.display()),
                        Err(err) => eprintln!("Failed to save image to {}: {err}", path.display()),
                    }
                }
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            eprintln!("File saving is not supported on WASM targets");
        }

        inner.response
    }

    #[expect(clippy::unused_self)]
    pub fn show_controls(&self, ui: &mut egui::Ui) -> Response {
        let response = ui.button("Save Plot");
        if response.clicked() {
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
        }
        response
    }
}
