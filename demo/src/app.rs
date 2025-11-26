use eframe::egui;
use egui::Color32;
use egui::RichText;
use egui::special_emojis::GITHUB;
use examples_utils::PlotExample;

const GITHUB_URL: &str = "https://github.com/emilk/egui_plot";

pub struct DemoGallery {
    /// All instantiated examples.
    examples: Vec<Box<dyn PlotExample>>,

    /// Index of the currently selected example in `examples`.
    current_example: Option<usize>,

    /// Cached thumbnail textures, aligned with `examples` by index.
    thumbnail_textures: Vec<egui::TextureHandle>,
}

impl eframe::App for DemoGallery {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.ui(ctx);
    }
}

impl DemoGallery {
    const NUM_COLS: usize = 3;

    pub fn new(ctx: &egui::Context) -> Self {
        let examples = Self::create_examples();
        let thumbnail_textures = Self::load_thumbnails(ctx, &examples);

        Self {
            examples,
            current_example: None,
            thumbnail_textures,
        }
    }

    fn ui(&mut self, ctx: &egui::Context) {
        Self::top_bar(ctx);
        self.examples_panel(ctx);
        if let Some(index) = self.current_example {
            self.info_panel(ctx, index);
        }
        self.central_panel(ctx);
    }

    fn top_bar(ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                egui::widgets::global_theme_preference_buttons(ui);

                ui.add_space(16.0);
                ui.hyperlink_to(format!("{GITHUB} egui_plot on GitHub"), GITHUB_URL);
            });
        });
    }

    fn examples_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("examples_panel")
            .resizable(true)
            .exact_width(210.0 * Self::NUM_COLS as f32)
            .show(ctx, |ui| {
                ui.heading("Examples");
                ui.separator();

                let num_examples = self.examples.len();
                egui::Grid::new("examples_grid").show(ui, |ui| {
                    for index in 0..num_examples {
                        self.make_cell(index, ui);
                        if (index + 1) % Self::NUM_COLS == 0 {
                            ui.end_row();
                        }
                    }
                });
            });
    }

    fn info_panel(&mut self, ctx: &egui::Context, index: usize) {
        egui::SidePanel::right("info_panel")
            .resizable(true)
            .exact_width(350.0)
            .show(ctx, |ui| {
                let example = &mut self.examples[index];
                ui.label(RichText::new(example.title()).heading());
                ui.separator();

                ui.label(RichText::new(example.description()).line_height(Some(20.0)));
                ui.separator();

                ui.hyperlink_to(
                    format!("{GITHUB} Source code of `{}`", example.name()),
                    format!("{GITHUB_URL}/tree/main/examples/{}", example.name()),
                );
                ui.separator();

                ui.label("Tags:");
                egui_chip::ChipEditBuilder::new(",")
                    .expect("failed to create ChipEditBuilder")
                    .texts(example.tags())
                    .chip_size(Some([80.0, 20.0]))
                    .chip_colors(Color32::WHITE, Color32::BLACK)
                    .widget_colors(Color32::TRANSPARENT, Color32::TRANSPARENT)
                    .build()
                    .show(ui);
            });
    }

    fn central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(index) = self.current_example {
                self.examples[index].show_ui(ui);
            } else {
                ui.vertical_centered(|ui| {
                    ui.heading("Select an example from the left panel");
                });
            }
        });
    }

    fn create_examples() -> Vec<Box<dyn PlotExample>> {
        vec![
            Box::new(borrow_points::BorrowPointsExample::default()),
            Box::new(box_plot::BoxPlotExample::default()),
            Box::new(custom_axes::CustomAxesExample::default()),
            Box::new(custom_plot_manipulation::CustomPlotManipulationExample::default()),
            Box::new(histogram::HistogramExample::default()),
            Box::new(interaction::InteractionExample::default()),
            Box::new(items::ItemsExample::default()),
            Box::new(legend::LegendExample::default()),
            Box::new(legend_sort::LegendSortExample::default()),
            Box::new(lines::LineExample::default()),
            Box::new(linked_axes::LinkedAxesExample::default()),
            Box::new(markers::MarkerDemo::default()),
            Box::new(save_plot::SavePlotExample::default()),
            Box::new(stacked_bar::StackedBarExample::default()),
        ]
    }

    fn load_thumbnails(ctx: &egui::Context, examples: &[Box<dyn PlotExample>]) -> Vec<egui::TextureHandle> {
        let mut thumbnail_textures = Vec::with_capacity(examples.len());

        for example in examples {
            let image = image::load_from_memory(example.thumbnail_bytes())
                .unwrap_or_else(|_| image::DynamicImage::new_rgba8(1, 1));
            let image = image.to_rgba8();
            let size = [image.width() as usize, image.height() as usize];
            let color_image = egui::ColorImage::from_rgba_unmultiplied(size, image.as_raw());

            let texture = ctx.load_texture(example.name(), color_image, egui::TextureOptions::default());
            thumbnail_textures.push(texture);
        }

        thumbnail_textures
    }

    fn make_cell(&mut self, index: usize, ui: &mut egui::Ui) {
        let is_selected = self.current_example == Some(index);

        let button = {
            let texture = &self.thumbnail_textures[index];
            let image = egui::Image::new((texture.id(), egui::vec2(192.0, 192.0)));
            let mut button = egui::Button::image(image);

            if is_selected {
                button = button.fill(ui.visuals().selection.bg_fill);
            }

            ui.add(button)
        }
        .on_hover_text_at_pointer(format!(
            "Click to select `{}`\nTags: {}",
            self.examples[index].title(),
            self.examples[index].tags().join(", ")
        ));

        if button.clicked() {
            self.current_example = Some(index);
        }
    }
}

// review: tags, resize contents
