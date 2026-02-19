use eframe::egui;
use egui::Color32;
use egui::RichText;
use egui::ScrollArea;
use egui::TextEdit;
use egui::Vec2;
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
    // Width of a column in the thumbnails panel.
    // TODO(#193): I don't know what units this corresponds to, and should be
    // cleaned up...
    const COL_WIDTH: f32 = 128.0;

    pub fn new(ctx: &egui::Context) -> Self {
        let examples: Vec<Box<dyn PlotExample>> = vec![
            Box::new(borrow_points::BorrowPointsExample::default()),
            Box::new(box_plot::BoxPlotExample::default()),
            Box::new(custom_axes::CustomAxesExample::default()),
            Box::new(custom_plot_manipulation::CustomPlotManipulationExample::default()),
            Box::new(filled_area::FilledAreaExample::default()),
            Box::new(heatmap::HeatmapDemo::default()),
            Box::new(histogram::HistogramExample::default()),
            Box::new(interaction::InteractionExample::default()),
            Box::new(items::ItemsExample::default()),
            Box::new(legend::LegendExample::default()),
            Box::new(legend_sort::LegendSortExample::default()),
            Box::new(lines::LineExample::default()),
            Box::new(linked_axes::LinkedAxesExample::default()),
            Box::new(markers::MarkerDemo::default()),
            Box::new(performance::PerformanceDemo::default()),
            Box::new(plot_span::PlotSpanDemo::default()),
            Box::new(save_plot::SavePlotExample::default()),
            Box::new(stacked_bar::StackedBarExample::default()),
            Box::new(userdata_points::UserdataPointsExample::default()),
        ];
        let thumbnail_textures = Self::load_thumbnails(ctx, &examples);

        Self {
            examples,
            current_example: None,
            thumbnail_textures,
        }
    }

    fn ui(&mut self, ctx: &egui::Context) {
        let screen_rect = ctx.available_rect();
        let is_small_screen = screen_rect.width() < 1024.0;

        Self::top_bar(ctx);
        if !is_small_screen {
            if let Some(index) = self.current_example {
                self.info_panel(ctx, index);
            }
        }
        self.thumbnails_panel(ctx, screen_rect.width() / 3.0);
        self.demo_panel(ctx);
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

    fn thumbnails_panel(&mut self, ctx: &egui::Context, default_width: f32) {
        egui::SidePanel::left("examples_panel")
            .default_width(default_width)
            // Set min_width so the heading is well rendered.
            .min_width(100.0)
            // 3 columns + some space extra for buttons.
            // TODO(#193): get rid of "extra space" calc.
            .max_width(Self::COL_WIDTH * 3. + 30.)
            .resizable(true)
            .show(ctx, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    let available_width = ui.available_width();
                    let num_columns = 1.max((available_width / Self::COL_WIDTH).floor() as usize);
                    let scale = 1.0_f32.min(available_width / (Self::COL_WIDTH * num_columns as f32));

                    ui.heading("Examples");
                    ui.separator();

                    let num_examples = self.examples.len();
                    egui::Grid::new("examples_grid").show(ui, |ui| {
                        for index in 0..num_examples {
                            self.make_cell(index, ui, scale);
                            if (index + 1) % num_columns == 0 {
                                ui.end_row();
                            }
                        }
                    });
                });
            });
    }

    fn info_panel(&mut self, ctx: &egui::Context, index: usize) {
        egui::SidePanel::right("info_panel")
            .resizable(true)
            .default_width(600.0)
            .show(ctx, |ui| {
                let example = &mut self.examples[index];
                ui.label(RichText::new(example.title()).heading());
                ui.separator();

                ui.label(RichText::new(example.description()).line_height(Some(20.0)));

                ui.horizontal(|ui| {
                    ui.label("Tags:");
                    #[expect(clippy::expect_used, reason = "tags are non-empty strings")]
                    egui_chip::ChipEditBuilder::new(",")
                        .expect("failed to create ChipEditBuilder")
                        .texts(example.tags())
                        .chip_size(Some([80.0, 20.0]))
                        .chip_colors(Color32::WHITE, Color32::BLACK)
                        .widget_colors(Color32::TRANSPARENT, Color32::TRANSPARENT)
                        .build()
                        .show(ui);
                });
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label(format!("Code of `{}` see also on ", example.name()));
                    ui.hyperlink_to(
                        format!("{GITHUB} Github"),
                        format!("{GITHUB_URL}/tree/main/examples/{}", example.name()),
                    );
                });
                let mut source_code = String::from_utf8_lossy(example.code_bytes()).to_string();
                ScrollArea::vertical().show(ui, |ui| {
                    let text_edit = TextEdit::multiline(&mut source_code).code_editor().desired_width(600.0);
                    ui.add(text_edit);
                });
            });
    }

    fn demo_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(index) = self.current_example {
                ui.vertical(|ui| {
                    self.examples[index].show_controls(ui);
                    ui.separator();
                    self.examples[index].show_ui(ui);
                });
            } else {
                ui.vertical_centered(|ui| {
                    ui.heading("Select an example from the left panel");
                });
            }
        });
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

    fn make_cell(&mut self, index: usize, ui: &mut egui::Ui, scale: f32) {
        let is_selected = self.current_example == Some(index);

        let button = {
            let texture = &self.thumbnail_textures[index];
            // TODO(#193): I don't know what units this corresponds to, and should be
            // cleaned up.
            let image = egui::Image::new((texture.id(), Vec2::splat(110.0 * scale)));
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
