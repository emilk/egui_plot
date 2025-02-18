use std::f64::consts::TAU;
use std::ops::RangeInclusive;

use egui::{
    remap, vec2, Color32, ComboBox, NumExt, Pos2, Response, ScrollArea, Stroke, TextWrapMode,
    Vec2b, WidgetInfo, WidgetType,
};

use egui_plot::{
    Arrows, AxisHints, Bar, BarChart, BoxElem, BoxPlot, BoxSpread, CoordinatesFormatter, Corner,
    GridInput, GridMark, HLine, Legend, Line, LineStyle, MarkerShape, Plot, PlotImage, PlotPoint,
    PlotPoints, PlotResponse, Points, Polygon, Text, VLine,
};

// ----------------------------------------------------------------------------

#[derive(PartialEq, Eq, serde::Deserialize, serde::Serialize)]
enum Panel {
    Lines,
    Markers,
    Legend,
    Charts,
    Items,
    Interaction,
    CustomAxes,
    LinkedAxes,
}

impl Default for Panel {
    fn default() -> Self {
        Self::Lines
    }
}

// ----------------------------------------------------------------------------

#[derive(Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PlotDemo {
    line_demo: LineDemo,
    marker_demo: MarkerDemo,
    legend_demo: LegendDemo,
    charts_demo: ChartsDemo,
    items_demo: ItemsDemo,
    interaction_demo: InteractionDemo,
    custom_axes_demo: CustomAxesDemo,
    linked_axes_demo: LinkedAxesDemo,
    open_panel: Panel,
}

impl PlotDemo {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            egui::reset_button(ui, self, "Reset");
            ui.collapsing("Instructions", |ui| {
                ui.label("Pan by dragging, or scroll (+ shift = horizontal).");
                ui.label("Box zooming: Right click to zoom in and zoom out using a selection.");
                if cfg!(target_arch = "wasm32") {
                    ui.label("Zoom with ctrl / ⌘ + pointer wheel, or with pinch gesture.");
                } else if cfg!(target_os = "macos") {
                    ui.label("Zoom with ctrl / ⌘ + scroll.");
                } else {
                    ui.label("Zoom with ctrl + scroll.");
                }
                ui.label("Reset view with double-click.");
            });
            ui.add(crate::egui_github_link_file!());
        });
        ui.separator();
        ui.horizontal_wrapped(|ui| {
            // We give the ui a label so we can easily enumerate all demos in the tests
            // The actual accessibility benefit is questionable considering the plot itself isn't
            // accessible at all
            let container_response = ui.response();
            container_response
                .widget_info(|| WidgetInfo::labeled(WidgetType::RadioGroup, true, "Select Demo"));

            // TODO(lucasmerlin): The parent ui should ideally be automatically set as AccessKit parent
            // or at least, with an opt in via UiBuilder, making this much more readable
            // See https://github.com/emilk/egui/issues/5674
            ui.ctx()
                .clone()
                .with_accessibility_parent(container_response.id, || {
                    ui.selectable_value(&mut self.open_panel, Panel::Lines, "Lines");
                    ui.selectable_value(&mut self.open_panel, Panel::Markers, "Markers");
                    ui.selectable_value(&mut self.open_panel, Panel::Legend, "Legend");
                    ui.selectable_value(&mut self.open_panel, Panel::Charts, "Charts");
                    ui.selectable_value(&mut self.open_panel, Panel::Items, "Items");
                    ui.selectable_value(&mut self.open_panel, Panel::Interaction, "Interaction");
                    ui.selectable_value(&mut self.open_panel, Panel::CustomAxes, "Custom Axes");
                    ui.selectable_value(&mut self.open_panel, Panel::LinkedAxes, "Linked Axes");
                });
        });
        ui.separator();

        match self.open_panel {
            Panel::Lines => {
                self.line_demo.ui(ui);
            }
            Panel::Markers => {
                self.marker_demo.ui(ui);
            }
            Panel::Legend => {
                self.legend_demo.ui(ui);
            }
            Panel::Charts => {
                self.charts_demo.ui(ui);
            }
            Panel::Items => {
                self.items_demo.ui(ui);
            }
            Panel::Interaction => {
                self.interaction_demo.ui(ui);
            }
            Panel::CustomAxes => {
                self.custom_axes_demo.ui(ui);
            }
            Panel::LinkedAxes => {
                self.linked_axes_demo.ui(ui);
            }
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Copy, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
struct LineDemo {
    animate: bool,
    time: f64,
    circle_radius: f64,
    circle_center: Pos2,
    square: bool,
    proportional: bool,
    coordinates: bool,
    show_axes: bool,
    show_grid: bool,
    line_style: LineStyle,
}

impl Default for LineDemo {
    fn default() -> Self {
        Self {
            animate: !cfg!(debug_assertions),
            time: 0.0,
            circle_radius: 1.5,
            circle_center: Pos2::new(0.0, 0.0),
            square: false,
            proportional: true,
            coordinates: true,
            show_axes: true,
            show_grid: true,
            line_style: LineStyle::Solid,
        }
    }
}

impl LineDemo {
    fn options_ui(&mut self, ui: &mut egui::Ui) {
        let Self {
            animate,
            time: _,
            circle_radius,
            circle_center,
            square,
            proportional,
            coordinates,
            show_axes,
            show_grid,
            line_style,
        } = self;

        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Circle:");
                    ui.add(
                        egui::DragValue::new(circle_radius)
                            .speed(0.1)
                            .range(0.0..=f64::INFINITY)
                            .prefix("r: "),
                    );
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::DragValue::new(&mut circle_center.x)
                                .speed(0.1)
                                .prefix("x: "),
                        );
                        ui.add(
                            egui::DragValue::new(&mut circle_center.y)
                                .speed(1.0)
                                .prefix("y: "),
                        );
                    });
                });
            });

            ui.vertical(|ui| {
                ui.checkbox(show_axes, "Show axes");
                ui.checkbox(show_grid, "Show grid");
                ui.checkbox(coordinates, "Show coordinates on hover")
                    .on_hover_text("Can take a custom formatting function.");
            });

            ui.vertical(|ui| {
                ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
                ui.checkbox(animate, "Animate");
                ui.checkbox(square, "Square view")
                    .on_hover_text("Always keep the viewport square.");
                ui.checkbox(proportional, "Proportional data axes")
                    .on_hover_text("Tick are the same size on both axes.");

                ComboBox::from_label("Line style")
                    .selected_text(line_style.to_string())
                    .show_ui(ui, |ui| {
                        for style in &[
                            LineStyle::Solid,
                            LineStyle::dashed_dense(),
                            LineStyle::dashed_loose(),
                            LineStyle::dotted_dense(),
                            LineStyle::dotted_loose(),
                        ] {
                            ui.selectable_value(line_style, *style, style.to_string());
                        }
                    });
            });
        });
    }

    fn circle<'a>(&self) -> Line<'a> {
        let n = 512;
        let circle_points: PlotPoints<'_> = (0..=n)
            .map(|i| {
                let t = remap(i as f64, 0.0..=(n as f64), 0.0..=TAU);
                let r = self.circle_radius;
                [
                    r * t.cos() + self.circle_center.x as f64,
                    r * t.sin() + self.circle_center.y as f64,
                ]
            })
            .collect();
        Line::new("circle", circle_points)
            .color(Color32::from_rgb(100, 200, 100))
            .style(self.line_style)
    }

    fn sin<'a>(&self) -> Line<'a> {
        let time = self.time;
        Line::new(
            "wave",
            PlotPoints::from_explicit_callback(
                move |x| 0.5 * (2.0 * x).sin() * time.sin(),
                ..,
                512,
            ),
        )
        .color(Color32::from_rgb(200, 100, 100))
        .style(self.line_style)
    }

    fn thingy<'a>(&self) -> Line<'a> {
        let time = self.time;
        Line::new(
            "x = sin(2t), y = sin(3t)",
            PlotPoints::from_parametric_callback(
                move |t| ((2.0 * t + time).sin(), (3.0 * t).sin()),
                0.0..=TAU,
                256,
            ),
        )
        .color(Color32::from_rgb(100, 150, 250))
        .style(self.line_style)
    }
}

impl LineDemo {
    fn ui(&mut self, ui: &mut egui::Ui) -> Response {
        ScrollArea::horizontal().show(ui, |ui| {
            self.options_ui(ui);
        });

        if self.animate {
            ui.ctx().request_repaint();
            self.time += ui.input(|i| i.unstable_dt).at_most(1.0 / 30.0) as f64;
        };
        let mut plot = Plot::new("lines_demo")
            .legend(Legend::default())
            .show_axes(self.show_axes)
            .show_grid(self.show_grid);
        if self.square {
            plot = plot.view_aspect(1.0);
        }
        if self.proportional {
            plot = plot.data_aspect(1.0);
        }
        if self.coordinates {
            plot = plot.coordinates_formatter(Corner::LeftBottom, CoordinatesFormatter::default());
        }
        plot.show(ui, |plot_ui| {
            plot_ui.line(self.circle());
            plot_ui.line(self.sin());
            plot_ui.line(self.thingy());
        })
        .response
    }
}

// ----------------------------------------------------------------------------

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
struct MarkerDemo {
    fill_markers: bool,
    marker_radius: f32,
    automatic_colors: bool,
    marker_color: Color32,
}

impl Default for MarkerDemo {
    fn default() -> Self {
        Self {
            fill_markers: true,
            marker_radius: 5.0,
            automatic_colors: true,
            marker_color: Color32::GREEN,
        }
    }
}

impl MarkerDemo {
    fn markers<'a>(&self) -> Vec<Points<'a>> {
        MarkerShape::all()
            .enumerate()
            .map(|(i, marker)| {
                let y_offset = i as f64 * 0.5 + 1.0;
                let mut points = Points::new(
                    "marker",
                    vec![
                        [1.0, 0.0 + y_offset],
                        [2.0, 0.5 + y_offset],
                        [3.0, 0.0 + y_offset],
                        [4.0, 0.5 + y_offset],
                        [5.0, 0.0 + y_offset],
                        [6.0, 0.5 + y_offset],
                    ],
                )
                .name(format!("{marker:?}"))
                .filled(self.fill_markers)
                .radius(self.marker_radius)
                .shape(marker);

                if !self.automatic_colors {
                    points = points.color(self.marker_color);
                }

                points
            })
            .collect()
    }

    fn ui(&mut self, ui: &mut egui::Ui) -> Response {
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.fill_markers, "Fill");
            ui.add(
                egui::DragValue::new(&mut self.marker_radius)
                    .speed(0.1)
                    .range(0.0..=f64::INFINITY)
                    .prefix("Radius: "),
            );
            ui.checkbox(&mut self.automatic_colors, "Automatic colors");
            if !self.automatic_colors {
                ui.color_edit_button_srgba(&mut self.marker_color);
            }
        });

        let markers_plot = Plot::new("markers_demo")
            .data_aspect(1.0)
            .legend(Legend::default());
        markers_plot
            .show(ui, |plot_ui| {
                for marker in self.markers() {
                    plot_ui.points(marker);
                }
            })
            .response
    }
}

// ----------------------------------------------------------------------------

#[derive(Default, PartialEq, serde::Deserialize, serde::Serialize)]
struct LegendDemo {
    config: Legend,
}

impl LegendDemo {
    fn line_with_slope<'a>(slope: f64) -> Line<'a> {
        Line::new(
            "line with slope",
            PlotPoints::from_explicit_callback(move |x| slope * x, .., 100),
        )
    }

    fn sin<'a>() -> Line<'a> {
        Line::new(
            "sin(x)",
            PlotPoints::from_explicit_callback(move |x| x.sin(), .., 100),
        )
    }

    fn cos<'a>() -> Line<'a> {
        Line::new(
            "cos(x)",
            PlotPoints::from_explicit_callback(move |x| x.cos(), .., 100),
        )
    }

    fn ui(&mut self, ui: &mut egui::Ui) -> Response {
        ScrollArea::horizontal().show(ui, |ui| {
            self.settings_ui(ui);
        });

        let Self { config } = self;
        let legend_plot = Plot::new("legend_demo")
            .legend(config.clone())
            .data_aspect(1.0);
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

    fn settings_ui(&mut self, ui: &mut egui::Ui) {
        let Self { config } = self;
        egui::Grid::new("settings").show(ui, |ui| {
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
        });
    }
}

// ----------------------------------------------------------------------------

#[derive(Default, PartialEq, serde::Deserialize, serde::Serialize)]
struct CustomAxesDemo {}

impl CustomAxesDemo {
    const MINS_PER_DAY: f64 = 24.0 * 60.0;
    const MINS_PER_H: f64 = 60.0;

    fn logistic_fn<'a>() -> Line<'a> {
        fn days(min: f64) -> f64 {
            CustomAxesDemo::MINS_PER_DAY * min
        }

        let values = PlotPoints::from_explicit_callback(
            move |x| 1.0 / (1.0 + (-2.5 * (x / Self::MINS_PER_DAY - 2.0)).exp()),
            days(0.0)..days(5.0),
            100,
        );
        Line::new("logistic fn", values)
    }

    #[allow(clippy::needless_pass_by_value)]
    fn x_grid(input: GridInput) -> Vec<GridMark> {
        // Note: this always fills all possible marks. For optimization, `input.bounds`
        // could be used to decide when the low-interval grids (minutes) should be added.

        let mut marks = vec![];

        let (min, max) = input.bounds;
        let min = min.floor() as i32;
        let max = max.ceil() as i32;

        for i in min..=max {
            let step_size = if i % Self::MINS_PER_DAY as i32 == 0 {
                // 1 day
                Self::MINS_PER_DAY
            } else if i % Self::MINS_PER_H as i32 == 0 {
                // 1 hour
                Self::MINS_PER_H
            } else if i % 5 == 0 {
                // 5min
                5.0
            } else {
                // skip grids below 5min
                continue;
            };

            marks.push(GridMark {
                value: i as f64,
                step_size,
            });
        }

        marks
    }

    #[allow(clippy::unused_self)]
    fn ui(&self, ui: &mut egui::Ui) -> Response {
        const MINS_PER_DAY: f64 = CustomAxesDemo::MINS_PER_DAY;
        const MINS_PER_H: f64 = CustomAxesDemo::MINS_PER_H;

        fn day(x: f64) -> f64 {
            (x / MINS_PER_DAY).floor()
        }

        fn hour(x: f64) -> f64 {
            (x.rem_euclid(MINS_PER_DAY) / MINS_PER_H).floor()
        }

        fn minute(x: f64) -> f64 {
            x.rem_euclid(MINS_PER_H).floor()
        }

        fn percent(y: f64) -> f64 {
            100.0 * y
        }

        let time_formatter = |mark: GridMark, _range: &RangeInclusive<f64>| {
            let minutes = mark.value;
            if !(0.0..5.0 * MINS_PER_DAY).contains(&minutes) {
                // No labels outside value bounds
                String::new()
            } else if is_approx_integer(minutes / MINS_PER_DAY) {
                // Days
                format!("Day {}", day(minutes))
            } else {
                // Hours and minutes
                format!("{h}:{m:02}", h = hour(minutes), m = minute(minutes))
            }
        };

        let percentage_formatter = |mark: GridMark, _range: &RangeInclusive<f64>| {
            let percent = 100.0 * mark.value;
            if is_approx_zero(percent) {
                String::new() // skip zero
            } else if is_approx_integer(percent) {
                // Display only integer percentages
                format!("{percent:.0}%")
            } else {
                String::new()
            }
        };

        let label_fmt = |_s: &str, val: &PlotPoint| {
            format!(
                "Day {d}, {h}:{m:02}\n{p:.2}%",
                d = day(val.x),
                h = hour(val.x),
                m = minute(val.x),
                p = percent(val.y)
            )
        };

        ui.label("Zoom in on the X-axis to see hours and minutes");

        let x_axes = vec![
            AxisHints::new_x()
                .label("Time")
                .formatter(time_formatter)
                .placement(egui_plot::VPlacement::Top),
            AxisHints::new_x().label("Time").formatter(time_formatter),
            AxisHints::new_x().label("Value"),
        ];
        let y_axes = vec![
            AxisHints::new_y()
                .label("Percent")
                .formatter(percentage_formatter),
            AxisHints::new_y()
                .label("Absolute")
                .placement(egui_plot::HPlacement::Right),
        ];
        Plot::new("custom_axes")
            .data_aspect(2.0 * MINS_PER_DAY as f32)
            .custom_x_axes(x_axes)
            .custom_y_axes(y_axes)
            .x_grid_spacer(Self::x_grid)
            .label_formatter(label_fmt)
            .show(ui, |plot_ui| {
                plot_ui.line(Self::logistic_fn());
            })
            .response
    }
}

// ----------------------------------------------------------------------------

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
struct LinkedAxesDemo {
    link_axis: Vec2b,
    link_cursor: Vec2b,
}

impl Default for LinkedAxesDemo {
    fn default() -> Self {
        Self {
            link_axis: Vec2b::new(true, true),
            link_cursor: Vec2b::new(true, true),
        }
    }
}

impl LinkedAxesDemo {
    fn line_with_slope<'a>(slope: f64) -> Line<'a> {
        Line::new(
            "line with slope",
            PlotPoints::from_explicit_callback(move |x| slope * x, .., 100),
        )
    }

    fn sin<'a>() -> Line<'a> {
        Line::new(
            "sin(x)",
            PlotPoints::from_explicit_callback(move |x| x.sin(), .., 100),
        )
    }

    fn cos<'a>() -> Line<'a> {
        Line::new(
            "cos(x)",
            PlotPoints::from_explicit_callback(move |x| x.cos(), .., 100),
        )
    }

    fn configure_plot(plot_ui: &mut egui_plot::PlotUi<'_>) {
        plot_ui.line(Self::line_with_slope(0.5));
        plot_ui.line(Self::line_with_slope(1.0));
        plot_ui.line(Self::line_with_slope(2.0));
        plot_ui.line(Self::sin());
        plot_ui.line(Self::cos());
    }

    fn ui(&mut self, ui: &mut egui::Ui) -> Response {
        ui.horizontal(|ui| {
            ui.label("Linked axes:");
            ui.checkbox(&mut self.link_axis.x, "X");
            ui.checkbox(&mut self.link_axis.y, "Y");
        });
        ui.horizontal(|ui| {
            ui.label("Linked cursors:");
            ui.checkbox(&mut self.link_cursor.x, "X");
            ui.checkbox(&mut self.link_cursor.y, "Y");
        });

        ScrollArea::horizontal()
            .show(ui, |ui| self.plots_ui(ui))
            .inner
    }

    fn plots_ui(&self, ui: &mut egui::Ui) -> Response {
        ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
        let link_group_id = ui.id().with("linked_demo");
        ui.horizontal(|ui| {
            Plot::new("left-top")
                .data_aspect(1.0)
                .width(250.0)
                .height(250.0)
                .link_axis(link_group_id, self.link_axis)
                .link_cursor(link_group_id, self.link_cursor)
                .show(ui, Self::configure_plot);
            Plot::new("right-top")
                .data_aspect(2.0)
                .width(150.0)
                .height(250.0)
                .y_axis_label("y")
                .y_axis_position(egui_plot::HPlacement::Right)
                .link_axis(link_group_id, self.link_axis)
                .link_cursor(link_group_id, self.link_cursor)
                .show(ui, Self::configure_plot);
        });
        Plot::new("left-bottom")
            .data_aspect(0.5)
            .width(250.0)
            .height(150.0)
            .x_axis_label("x")
            .link_axis(link_group_id, self.link_axis)
            .link_cursor(link_group_id, self.link_cursor)
            .show(ui, Self::configure_plot)
            .response
    }
}

// ----------------------------------------------------------------------------

#[derive(Default, PartialEq, serde::Deserialize, serde::Serialize)]
struct ItemsDemo {
    #[serde(skip)]
    texture: Option<egui::TextureHandle>,
}

impl ItemsDemo {
    fn ui(&mut self, ui: &mut egui::Ui) -> Response {
        let n = 100;
        let mut sin_values: Vec<_> = (0..=n)
            .map(|i| remap(i as f64, 0.0..=n as f64, -TAU..=TAU))
            .map(|i| [i, i.sin()])
            .collect();

        let line = Line::new("sin(x)", sin_values.split_off(n / 2)).fill(-1.5);
        let polygon = Polygon::new(
            "polygon",
            PlotPoints::from_parametric_callback(
                |t| (4.0 * t.sin() + 2.0 * t.cos(), 4.0 * t.cos() + 2.0 * t.sin()),
                0.0..TAU,
                100,
            ),
        );
        let points = Points::new("sin(x)", sin_values).stems(-1.5).radius(1.0);

        let arrows = {
            let pos_radius = 8.0;
            let tip_radius = 7.0;
            let arrow_origins = PlotPoints::from_parametric_callback(
                |t| (pos_radius * t.sin(), pos_radius * t.cos()),
                0.0..TAU,
                36,
            );
            let arrow_tips = PlotPoints::from_parametric_callback(
                |t| (tip_radius * t.sin(), tip_radius * t.cos()),
                0.0..TAU,
                36,
            );
            Arrows::new("arrows", arrow_origins, arrow_tips)
        };

        let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
            ui.ctx()
                .load_texture("plot_demo", egui::ColorImage::example(), Default::default())
        });
        let image = PlotImage::new(
            "image",
            texture,
            PlotPoint::new(0.0, 10.0),
            5.0 * vec2(texture.aspect_ratio(), 1.0),
        );

        let plot = Plot::new("items_demo")
            .legend(Legend::default().position(Corner::RightBottom))
            .show_x(false)
            .show_y(false)
            .data_aspect(1.0);
        plot.show(ui, |plot_ui| {
            plot_ui.hline(HLine::new("Lines horizontal", 9.0));
            plot_ui.hline(HLine::new("Lines horizontal", -9.0));
            plot_ui.vline(VLine::new("Lines vertical", 9.0));
            plot_ui.vline(VLine::new("Lines vertical", -9.0));
            plot_ui.line(line.name("Line with fill").id("line_with_fill"));
            plot_ui.polygon(polygon.name("Convex polygon").id("convex_polygon"));
            plot_ui.points(points.name("Points with stems").id("points_with_stems"));
            plot_ui.text(Text::new("Text", PlotPoint::new(-3.0, -3.0), "wow").id("text0"));
            plot_ui.text(Text::new("Text", PlotPoint::new(-2.0, 2.5), "so graph").id("text1"));
            plot_ui.text(Text::new("Text", PlotPoint::new(3.0, 3.0), "much color").id("text2"));
            plot_ui.text(Text::new("Text", PlotPoint::new(2.5, -2.0), "such plot").id("text3"));
            plot_ui.image(image.name("Image"));
            plot_ui.arrows(arrows.name("Arrows"));
        })
        .response
    }
}

// ----------------------------------------------------------------------------

#[derive(Default, PartialEq, serde::Deserialize, serde::Serialize)]
struct InteractionDemo {}

impl InteractionDemo {
    #[allow(clippy::unused_self)]
    fn ui(&self, ui: &mut egui::Ui) -> Response {
        let id = ui.make_persistent_id("interaction_demo");

        // This demonstrates how to read info about the plot _before_ showing it:
        let plot_memory = egui_plot::PlotMemory::load(ui.ctx(), id);
        if let Some(plot_memory) = plot_memory {
            let bounds = plot_memory.bounds();
            ui.label(format!(
                "plot bounds: min: {:.02?}, max: {:.02?}",
                bounds.min(),
                bounds.max()
            ));
        }

        let plot = Plot::new("interaction_demo").id(id).height(300.0);

        let PlotResponse {
            response,
            inner: (screen_pos, pointer_coordinate, pointer_coordinate_drag_delta, bounds, hovered),
            hovered_plot_item,
            ..
        } = plot.show(ui, |plot_ui| {
            plot_ui.line(
                Line::new(
                    "sin",
                    PlotPoints::from_explicit_callback(move |x| x.sin(), .., 100),
                )
                .color(Color32::RED),
            );
            plot_ui.line(
                Line::new(
                    "cos",
                    PlotPoints::from_explicit_callback(move |x| x.cos(), .., 100),
                )
                .color(Color32::BLUE),
            );

            (
                plot_ui.screen_from_plot(PlotPoint::new(0.0, 0.0)),
                plot_ui.pointer_coordinate(),
                plot_ui.pointer_coordinate_drag_delta(),
                plot_ui.plot_bounds(),
                plot_ui.response().hovered(),
            )
        });

        ui.label(format!(
            "plot bounds: min: {:.02?}, max: {:.02?}",
            bounds.min(),
            bounds.max()
        ));
        ui.label(format!(
            "origin in screen coordinates: x: {:.02}, y: {:.02}",
            screen_pos.x, screen_pos.y
        ));
        ui.label(format!("plot hovered: {hovered}"));
        let coordinate_text = if let Some(coordinate) = pointer_coordinate {
            format!("x: {:.02}, y: {:.02}", coordinate.x, coordinate.y)
        } else {
            "None".to_owned()
        };
        ui.label(format!("pointer coordinate: {coordinate_text}"));
        let coordinate_text = format!(
            "x: {:.02}, y: {:.02}",
            pointer_coordinate_drag_delta.x, pointer_coordinate_drag_delta.y
        );
        ui.label(format!("pointer coordinate drag delta: {coordinate_text}"));

        let hovered_item = if hovered_plot_item == Some(egui::Id::new("sin")) {
            "red sin"
        } else if hovered_plot_item == Some(egui::Id::new("cos")) {
            "blue cos"
        } else {
            "none"
        };
        ui.label(format!("hovered plot item: {hovered_item}"));

        response
    }
}

// ----------------------------------------------------------------------------

#[derive(PartialEq, Eq, serde::Deserialize, serde::Serialize)]
enum Chart {
    GaussBars,
    StackedBars,
    BoxPlot,
}

impl Default for Chart {
    fn default() -> Self {
        Self::GaussBars
    }
}

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
struct ChartsDemo {
    chart: Chart,
    vertical: bool,
    allow_zoom: Vec2b,
    allow_drag: Vec2b,
    allow_scroll: Vec2b,
}

impl Default for ChartsDemo {
    fn default() -> Self {
        Self {
            vertical: true,
            chart: Chart::default(),
            allow_zoom: true.into(),
            allow_drag: true.into(),
            allow_scroll: true.into(),
        }
    }
}

impl ChartsDemo {
    fn ui(&mut self, ui: &mut egui::Ui) -> Response {
        ScrollArea::horizontal().show(ui, |ui| {
            self.options_ui(ui);
        });
        match self.chart {
            Chart::GaussBars => self.bar_gauss(ui),
            Chart::StackedBars => self.bar_stacked(ui),
            Chart::BoxPlot => self.box_plot(ui),
        }
    }

    fn options_ui(&mut self, ui: &mut egui::Ui) -> Response {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Type:");
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.chart, Chart::GaussBars, "Histogram");
                    ui.selectable_value(&mut self.chart, Chart::StackedBars, "Stacked Bar Chart");
                    ui.selectable_value(&mut self.chart, Chart::BoxPlot, "Box Plot");
                });
                ui.label("Orientation:");
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.vertical, true, "Vertical");
                    ui.selectable_value(&mut self.vertical, false, "Horizontal");
                });
            });
            ui.vertical(|ui| {
                ui.group(|ui| {
                    ui.add_enabled_ui(self.chart != Chart::StackedBars, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Allow zoom:");
                            ui.checkbox(&mut self.allow_zoom.x, "X");
                            ui.checkbox(&mut self.allow_zoom.y, "Y");
                        });
                    });
                    ui.horizontal(|ui| {
                        ui.label("Allow drag:");
                        ui.checkbox(&mut self.allow_drag.x, "X");
                        ui.checkbox(&mut self.allow_drag.y, "Y");
                    });
                    ui.horizontal(|ui| {
                        ui.label("Allow scroll:");
                        ui.checkbox(&mut self.allow_scroll.x, "X");
                        ui.checkbox(&mut self.allow_scroll.y, "Y");
                    });
                });
            });
        })
        .response
    }

    fn bar_gauss(&self, ui: &mut egui::Ui) -> Response {
        let mut chart = BarChart::new(
            "Normal Distribution",
            (-395..=395)
                .step_by(10)
                .map(|x| x as f64 * 0.01)
                .map(|x| {
                    (
                        x,
                        (-x * x / 2.0).exp() / (2.0 * std::f64::consts::PI).sqrt(),
                    )
                })
                // The 10 factor here is purely for a nice 1:1 aspect ratio
                .map(|(x, f)| Bar::new(x, f * 10.0).width(0.1))
                .collect(),
        )
        .color(Color32::LIGHT_BLUE);

        if !self.vertical {
            chart = chart.horizontal();
        }

        Plot::new("Normal Distribution Demo")
            .legend(Legend::default())
            .clamp_grid(true)
            .allow_zoom(self.allow_zoom)
            .allow_drag(self.allow_drag)
            .allow_scroll(self.allow_scroll)
            .show(ui, |plot_ui| plot_ui.bar_chart(chart))
            .response
    }

    fn bar_stacked(&self, ui: &mut egui::Ui) -> Response {
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
            .allow_drag(self.allow_drag)
            .show(ui, |plot_ui| {
                plot_ui.bar_chart(chart1);
                plot_ui.bar_chart(chart2);
                plot_ui.bar_chart(chart3);
                plot_ui.bar_chart(chart4);
            })
            .response
    }

    fn box_plot(&self, ui: &mut egui::Ui) -> Response {
        let yellow = Color32::from_rgb(248, 252, 168);
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
                    .stroke(Stroke::new(1.5, yellow))
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

        if !self.vertical {
            box1 = box1.horizontal();
            box2 = box2.horizontal();
            box3 = box3.horizontal();
        }

        Plot::new("Box Plot Demo")
            .legend(Legend::default())
            .allow_zoom(self.allow_zoom)
            .allow_drag(self.allow_drag)
            .show(ui, |plot_ui| {
                plot_ui.box_plot(box1);
                plot_ui.box_plot(box2);
                plot_ui.box_plot(box3);
            })
            .response
    }
}

fn is_approx_zero(val: f64) -> bool {
    val.abs() < 1e-6
}

fn is_approx_integer(val: f64) -> bool {
    val.fract().abs() < 1e-6
}
