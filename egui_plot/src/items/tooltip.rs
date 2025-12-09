//! Multi-series tooltip & pinning overlays.
//!
//! This module implements a tooltip for `egui_plot` time series.
//!
//! Given a mouse position, we find the closest x-points of each series,
//! and if they are closer than some radius, we display the tooltip.
//! Additionally, these points can be "pinned" to inspect and compare
//! their values across pins, without the need to move the mouse back-and-forth.
//!
//! The pin snapshots store plot-space values (x,y) and the pinned plot-x,
//! so they remain correct across zoom/pan and are redrawn each frame.
//!
//! # Quick start
//!
//! ```ignore
//! Plot::new("my_plot").show(ui, |plot_ui| {
//!     // Default tooltip (simple table UI):
//!     plot_ui.show_tooltip_with_options(&TooltipOptions::default());
//! });
//! ```
//!
//! # Custom UI
//!
//! ```ignore
//! Plot::new("my_plot").show(ui, |plot_ui| {
//!     let opts = TooltipOptions::default()
//!         .highlight_lines_distance(Some(30.0))
//!         .show_pins_panel(true);
//!     plot_ui.show_tooltip_across_series_with(&opts, |ui, hits, pins| {
//!         ui.strong("My custom tooltip");
//!         for h in hits {
//!             ui.label(format!("{}: x={:.3}, y={:.3}", h.series_name, h.value.x, h.value.y));
//!         }
//!         ui.label(format!("{} pins", pins.len()));
//!     });
//! });
//! ```
//!
//! ## Notes
//! - Pins are stored in **egui temp memory**.
//!   They are **not persisted** across application restarts.
//! - Series highlighting currently matches by **series name**. Prefer unique names.

use egui::{Align2, Area, Color32, Frame, Grid, Id, Key, Order, Pos2, Rect, RichText, Stroke, TextStyle};

use crate::axis::PlotTransform;
use crate::bounds::PlotPoint;
use crate::items::PlotGeometry;
use crate::plot::PlotUi;

/// One selected anchor per series, found inside the vertical band.
///
/// Built once per frame for all participating series. Each row stores:
/// - the **series name** (used for display and highlight matching),
/// - **display color** (used for markers),
/// - the picked **plot value** `(x,y)`,
/// - its **screen position** (for drawing),
/// - screen distances to the pointer for sorting and highlighting.
#[derive(Clone, Debug)]
pub struct HitPoint {
    /// Series display name (should be unique/stable; used for highlight matching).
    pub series_name: String,
    /// Marker color used when drawing the on-canvas anchor.
    pub color: Color32,
    /// Picked plot-space value `(x, y)` for this series.
    pub value: PlotPoint,
    /// Screen-space position where the marker is drawn.
    pub screen_pos: Pos2,
    /// Horizontal distance in pixels from the pointer. Used for sorting.
    pub screen_dx: f32,
    /// Vertical distance in pixels from the pointer.
    pub screen_dy: f32,
    /// Whether this hit point is within the highlight distance threshold.
    pub is_highlighted: bool,
}

impl HitPoint {
    /// Manhattan distance from the pointer (|dx| + |dy|).
    #[inline]
    pub fn manhattan_distance(&self) -> f32 {
        self.screen_dx + self.screen_dy
    }
}

/// A pinned selection: the full set of `HitPoint`s plus the exact plot-space X.
///
/// Pins are created by pressing **`P`** while hovering the plot; they are kept
/// in egui *temp* memory and redrawn every frame (rails + markers). Press **`U`**
/// to remove the last pin, or **`Delete`** to clear all.
#[derive(Clone, Debug, Default)]
pub struct PinnedPoints {
    /// Cloned hits from the moment the pin was taken (plot-space values).
    pub hits: Vec<HitPoint>,
    /// The pinned plot-space X used to draw the vertical "pin rail".
    pub plot_x: f64,
}

/// Visual/behavioral settings for the band tooltip.
///
/// Use [`TooltipOptions::default()`] and adjust via builder-ish methods.
#[derive(Clone)]
pub struct TooltipOptions {
    /// Fill the vertical band region for visual feedback.
    pub draw_band_fill: bool,

    /// Draw a 1D guide line at the current pointer X.
    pub draw_vertical_guide: bool,

    /// Color for the band fill (typically a faint translucent blue).
    pub band_fill: Color32,

    /// Stroke for the vertical guide line.
    pub guide_stroke: Stroke,

    /// Show markers at hit points on each series (similar to pin markers).
    pub show_markers: bool,

    /// Radius of the on-canvas hit markers (in pixels).
    pub marker_radius: f32,

    /// If `Some(distance)`, highlight lines whose nearest point is within
    /// `distance` pixels (Manhattan distance: |dx| + |dy|) from the crosshair.
    /// If `None`, no lines are highlighted.
    pub highlight_lines_distance: Option<f32>,

    /// Show a small panel listing the current pins at the top-right.
    pub show_pins_panel: bool,

    /// Half-width of the vertical selection, in screen pixels.
    pub radius_px: f32,

    /// Horizontal gap between the vertical ruler and the tooltip (in pixels).
    /// The tooltip is offset in the direction with more available space.
    pub tooltip_horizontal_gap: f32,

    /// Vertical gap between the anchor point and the tooltip (in pixels).
    pub tooltip_vertical_gap: f32,
}

impl Default for TooltipOptions {
    fn default() -> Self {
        Self {
            draw_band_fill: true,
            draw_vertical_guide: true,
            band_fill: Color32::from_rgba_unmultiplied(120, 160, 255, 24),
            guide_stroke: Stroke::new(1.0, Color32::WHITE),
            show_markers: true,
            marker_radius: 5.0,
            highlight_lines_distance: Some(50.0),
            show_pins_panel: true,
            radius_px: 50.0,
            tooltip_horizontal_gap: 10.0,
            tooltip_vertical_gap: 10.0,
        }
    }
}

impl TooltipOptions {
    /// Set the maximum Manhattan distance (|dx| + |dy|, in pixels) from the crosshair
    /// at which lines will be highlighted. Pass `None` to disable highlighting.
    ///
    /// Only lines whose nearest point is within this distance from the
    /// crosshair will be visually emphasized.
    #[inline]
    pub fn highlight_lines_distance(mut self, distance: Option<f32>) -> Self {
        self.highlight_lines_distance = distance;
        self
    }

    /// Toggle whether to show markers at hit points on each series.
    /// These markers move with the crosshair, similar to pin markers.
    #[inline]
    pub fn show_markers(mut self, on: bool) -> Self {
        self.show_markers = on;
        self
    }

    /// Toggle whether to display the floating pins panel in the plot corner.
    #[inline]
    pub fn show_pins_panel(mut self, on: bool) -> Self {
        self.show_pins_panel = on;
        self
    }

    /// Set the horizontal gap between the vertical ruler and the tooltip.
    #[inline]
    pub fn tooltip_horizontal_gap(mut self, gap: f32) -> Self {
        self.tooltip_horizontal_gap = gap;
        self
    }

    /// Set the vertical gap between the anchor and the tooltip.
    #[inline]
    pub fn tooltip_vertical_gap(mut self, gap: f32) -> Self {
        self.tooltip_vertical_gap = gap;
        self
    }

    /// Set both horizontal and vertical tooltip gaps at once.
    #[inline]
    pub fn tooltip_gap(mut self, horizontal: f32, vertical: f32) -> Self {
        self.tooltip_horizontal_gap = horizontal;
        self.tooltip_vertical_gap = vertical;
        self
    }
}

/// Temp-memory storage for pins
/// Derive a memory key (sub-`Id`) for pins based on the plot's `Id`.
///
/// Pins are scoped **per plot** so multiple plots don't share a pin list.
fn pins_mem_id(base: Id) -> Id {
    base.with("band_pins_mem")
}

/// Load the pin list for this plot from **egui temp memory**.
///
/// Returns `Vec::new()` if nothing is stored. Pins are not persisted
/// across app restarts.
fn load_pins(ctx: &egui::Context, base: Id) -> Vec<PinnedPoints> {
    ctx.data(|d| d.get_temp::<Vec<PinnedPoints>>(pins_mem_id(base)))
        .unwrap_or_default()
}

/// Save (replace) the pin list for this plot in **egui temp memory**.
///
/// This overwrites the previously stored list for the same plot.
fn save_pins(ctx: &egui::Context, base: Id, v: Vec<PinnedPoints>) {
    ctx.data_mut(|d| d.insert_temp(pins_mem_id(base), v));
}

impl PlotUi<'_> {
    /// Default UI with custom options
    pub fn show_tooltip_with_options(&mut self, options: &TooltipOptions) {
        self.show_tooltip_across_series_with(options, default_tooltip_ui);
    }
    /// Provide options and a closure to build the **tooltip body UI**.
    ///
    /// - `options`: visual behavior knobs (band fill, markers, guide, etc).
    /// - `ui_builder`: called each frame to render the tooltip contents.
    ///   Receives:
    ///   - `&[HitPoint]`: per-series closest samples near the pointer X (this frame),
    ///   - `&[PinnedPoints]`: previously pinned snapshots.
    ///
    /// The overlay (band, markers, rails) and highlighting are handled by this
    /// function; the closure only draws the *tooltip* content (table, custom UI).
    pub fn show_tooltip_across_series_with(
        &mut self,

        options: &TooltipOptions,
        ui_builder: impl FnOnce(&mut egui::Ui, &[HitPoint], &[PinnedPoints]),
    ) {
        let ctx = self.ctx().clone();
        let visuals = ctx.style().visuals.clone();
        let transform = *self.transform();
        let frame = transform.frame();

        let mut pins = load_pins(&ctx, self.response.id);
        draw_pins_overlay(&ctx, &pins, transform, *frame, &visuals, options.marker_radius);

        if options.show_pins_panel && !pins.is_empty() {
            show_pins_panel(&ctx, *frame, &pins);
        }

        let Some(pointer_screen) = ctx.input(|i| i.pointer.latest_pos()) else {
            return;
        };

        let r = options.radius_px;
        let band_min_x = (pointer_screen.x - r).max(frame.left());
        let band_max_x = (pointer_screen.x + r).min(frame.right());
        if band_max_x <= band_min_x {
            return;
        }

        let mut hits: Vec<HitPoint> = Vec::new();

        for item in &self.items {
            if !item.allow_hover() {
                continue;
            }

            let base_color = {
                let c = item.color();
                if c == Color32::TRANSPARENT {
                    visuals.text_color()
                } else {
                    c
                }
            };

            let (mut best_ix, mut best_dx, mut best_dy, mut best_pos) = (None, f32::INFINITY, 0.0f32, Pos2::ZERO);

            match item.geometry() {
                PlotGeometry::Points(points) => {
                    for (ix, v) in points.iter().enumerate() {
                        let p = transform.position_from_point(v);
                        if p.x < band_min_x || p.x > band_max_x {
                            continue;
                        }
                        let dx = (p.x - pointer_screen.x).abs();
                        if dx < best_dx {
                            best_ix = Some(ix);
                            best_dx = dx;
                            best_dy = (p.y - pointer_screen.y).abs();
                            best_pos = p;
                        }
                    }
                }
                PlotGeometry::Rects | PlotGeometry::None => {}
            }

            if let Some(ix) = best_ix {
                let value = match item.geometry() {
                    PlotGeometry::Points(points) => points[ix],
                    _ => continue,
                };
                hits.push(HitPoint {
                    series_name: item.name().to_owned(),
                    color: base_color,
                    value,
                    screen_pos: best_pos,
                    screen_dx: best_dx,
                    screen_dy: best_dy,
                    is_highlighted: false, // Will be set below based on distance
                });
            }
        }

        if hits.is_empty() {
            if self.response.hovered() {
                let mut pins_changed = false;
                ctx.input(|i| {
                    if i.key_pressed(Key::U) {
                        pins.pop();
                        pins_changed = true;
                    }
                    if i.key_pressed(Key::Delete) {
                        pins.clear();
                        pins_changed = true;
                    }
                });
                if pins_changed {
                    save_pins(&ctx, self.response.id, pins);
                }
            }
            return;
        }

        // Sort by series name for stable ordering in the tooltip
        hits.sort_by(|a, b| a.series_name.cmp(&b.series_name));

        if let Some(highlight_distance) = options.highlight_lines_distance {
            // Mark hits within the highlight distance and highlight their corresponding lines
            for hit in &mut hits {
                if hit.manhattan_distance() <= highlight_distance {
                    hit.is_highlighted = true;
                }
            }
            let names_within_distance: ahash::AHashSet<&str> = hits
                .iter()
                .filter(|h| h.is_highlighted)
                .map(|h| h.series_name.as_str())
                .collect();
            for item in &mut self.items {
                if names_within_distance.contains(item.name()) {
                    item.highlight();
                }
            }
        }

        if self.response.hovered() {
            let mut pins_changed = false;
            ctx.input(|i| {
                if i.key_pressed(Key::P) {
                    let pointer_plot = transform.value_from_position(pointer_screen);
                    pins.push(PinnedPoints {
                        hits: hits.clone(),
                        plot_x: pointer_plot.x,
                    });
                    pins_changed = true;
                }
                if i.key_pressed(Key::U) {
                    pins.pop();
                    pins_changed = true;
                }
                if i.key_pressed(Key::Delete) {
                    pins.clear();
                    pins_changed = true;
                }
            });
            if pins_changed {
                save_pins(&ctx, self.response.id, pins.clone());
            }
        }

        {
            let painter = egui::Painter::new(ctx.clone(), self.response.layer_id, *frame);

            if options.draw_band_fill {
                let band_rect = Rect::from_min_max(
                    Pos2::new(band_min_x, frame.top()),
                    Pos2::new(band_max_x, frame.bottom()),
                );
                painter.rect_filled(band_rect, 0.0, options.band_fill);
            }
            if options.draw_vertical_guide {
                painter.line_segment(
                    [
                        Pos2::new(pointer_screen.x, frame.top()),
                        Pos2::new(pointer_screen.x, frame.bottom()),
                    ],
                    options.guide_stroke,
                );
            }
        }

        // Draw markers on foreground layer so they appear above plot content
        if options.show_markers {
            let marker_painter = egui::Painter::new(
                ctx.clone(),
                egui::LayerId::new(egui::Order::Foreground, egui::Id::new("tooltip_markers")),
                *frame,
            );
            for h in &hits {
                marker_painter.circle_filled(h.screen_pos, options.marker_radius, h.color);
                marker_painter.circle_stroke(
                    h.screen_pos,
                    options.marker_radius,
                    Stroke::new(1.0, visuals.window_stroke().color),
                );
            }
        }

        // Calculate tooltip anchor position with configurable gaps.
        // Offset horizontally away from the vertical ruler, in the direction with more space.
        let frame_center_x = frame.center().x;
        let horizontal_offset = if pointer_screen.x < frame_center_x {
            // Pointer is on the left half → place tooltip to the right
            options.tooltip_horizontal_gap
        } else {
            // Pointer is on the right half → place tooltip to the left
            -options.tooltip_horizontal_gap
        };
        let tooltip_anchor = Pos2::new(pointer_screen.x + horizontal_offset, pointer_screen.y);

        let mut tooltip = egui::Tooltip::always_open(
            ctx.clone(),
            self.response.layer_id,
            self.response.id.with("band_tooltip"),
            egui::PopupAnchor::Position(tooltip_anchor),
        );
        let tooltip_width = ctx.style().spacing.tooltip_width;
        tooltip.popup = tooltip.popup.width(tooltip_width);

        tooltip.gap(options.tooltip_vertical_gap).show(|ui| {
            ui.set_max_width(tooltip_width);
            ui_builder(ui, &hits, &pins);
        });
    }
}

/// Draws **all pin overlays**: a vertical rail per pin and markers at each pinned point.
///
/// Pins are stored in plot-space; this function transforms them back to screen
fn draw_pins_overlay(
    ctx: &egui::Context,
    pins: &[PinnedPoints],
    transform: PlotTransform,
    frame: Rect,
    visuals: &egui::style::Visuals,
    marker_radius: f32,
) {
    if pins.is_empty() {
        return;
    }
    let painter = egui::Painter::new(
        ctx.clone(),
        egui::LayerId::new(egui::Order::Foreground, egui::Id::new("pins_overlay")),
        frame,
    );

    let rail = Stroke::new(1.5, Color32::from_rgb(255, 200, 64));
    let label_font = TextStyle::Small.resolve(&ctx.style());

    for (k, group) in pins.iter().enumerate() {
        let x = transform.position_from_point(&PlotPoint::new(group.plot_x, 0.0)).x;
        painter.line_segment([Pos2::new(x, frame.top()), Pos2::new(x, frame.bottom())], rail);

        let label = format!("{}", k + 1);
        let tx = x.clamp(frame.left() + 12.0, frame.right() - 12.0);
        painter.text(
            Pos2::new(tx, frame.top() + 4.0),
            Align2::CENTER_TOP,
            label,
            label_font.clone(),
            visuals.strong_text_color(),
        );

        let outline = Stroke::new(1.5, visuals.strong_text_color());
        for h in &group.hits {
            let p = transform.position_from_point(&h.value);
            painter.circle_filled(p, marker_radius + 0.5, h.color);
            painter.circle_stroke(p, marker_radius + 0.5, outline);
        }
    }
}

/// Shows a small floating **Pins panel** in the top-right of the plot frame.
///
/// This is a *display-only* panel (not interactive), listing all pins and
/// their captured series rows. It helps the user review pinned values without
/// having to hover the plot again.
fn show_pins_panel(ctx: &egui::Context, frame: Rect, pins: &[PinnedPoints]) {
    let panel_id = Id::new("egui_plot_pins_panel");
    let panel_pos = Pos2::new(frame.right() - 240.0, frame.top() + 8.0);

    Area::new(panel_id)
        .order(Order::Foreground)
        .fixed_pos(panel_pos)
        .movable(false)
        .interactable(false)
        .show(ctx, |ui| {
            let mut f = Frame::window(ui.style())
                .fill(ui.style().visuals.extreme_bg_color)
                .stroke(ui.style().visuals.window_stroke());

            f.corner_radius = ui.style().visuals.window_corner_radius;
            f.show(ui, |ui| {
                ui.set_width(232.0);
                ui.strong(format!("Pins ({})", pins.len()));
                ui.separator();

                for (k, snap) in pins.iter().enumerate() {
                    egui::CollapsingHeader::new(format!("Pin #{}", k + 1))
                        .default_open(false)
                        .show(ui, |ui| {
                            egui::Grid::new(format!("pin_grid_{k}"))
                                .num_columns(4)
                                .spacing([6.0, 2.0])
                                .striped(true)
                                .show(ui, |ui| {
                                    ui.weak("");
                                    ui.weak("series");
                                    ui.weak("x");
                                    ui.weak("y");
                                    ui.end_row();
                                    for h in &snap.hits {
                                        ui.label(RichText::new("●").color(h.color));
                                        ui.monospace(&h.series_name);
                                        ui.monospace(format!("{:.6}", h.value.x));
                                        ui.monospace(format!("{:.6}", h.value.y));
                                        ui.end_row();
                                    }
                                });
                        });
                }

                if pins.is_empty() {
                    ui.weak("No pins yet. Hover plot and press P.");
                } else {
                    ui.add_space(6.0);
                    ui.weak("Hotkeys: P=pin, U=unpin, Delete=clear");
                }
            });
        });
}

/// Default tooltip content: a compact table with a row per hit (series).
fn default_tooltip_ui(ui: &mut egui::Ui, hits: &[HitPoint], pins: &[PinnedPoints]) {
    let x_dec = 3usize;
    let y_dec = 3usize;

    Grid::new(Id::new("egui_plot_band_tooltip_table"))
        .num_columns(3)
        .spacing([8.0, 2.0])
        .striped(true)
        .show(ui, |ui| {
            ui.weak("");
            ui.weak("x");
            ui.weak("y");
            ui.end_row();
            for h in hits {
                // Highlight the row if it's within the highlight distance
                if h.is_highlighted {
                    ui.label(RichText::new(&h.series_name).color(h.color).strong());
                    ui.strong(format!("{:.x_dec$}", h.value.x));
                    ui.strong(format!("{:.y_dec$}", h.value.y));
                } else {
                    ui.label(RichText::new(&h.series_name).color(h.color));
                    ui.label(format!("{:.x_dec$}", h.value.x));
                    ui.label(format!("{:.y_dec$}", h.value.y));
                }
                ui.end_row();
            }
        });

    if !pins.is_empty() {
        ui.add_space(6.0);
        ui.separator();
        ui.weak(format!("Pinned groups: {}  (P pin • U unpin • Del clear)", pins.len()));
    }
}
