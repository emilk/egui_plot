//! Pin overlay for marking and comparing values across the plot.
//!
//! Pins allow users to "pin" positions on the plot to compare values
//! at multiple X positions. Pins are stored in egui temp memory and persist
//! across frames (but not across application restarts).
//!
//! ## Usage
//!
//! ```ignore
//! Plot::new("my_plot").show(ui, |plot_ui| {
//!     plot_ui.line(Line::new("series", data));
//!
//!     // Show pins (handles P/U/Delete keys, draws rails & markers)
//!     plot_ui.show_pins(&PinOptions::default());
//! });
//! ```
//!
//! ## Hotkeys
//! - **P**: Pin current position
//! - **U**: Unpin (remove last pin)
//! - **Delete**: Clear all pins

use egui::{Align2, Area, Color32, Frame, Id, Key, Order, Pos2, Rect, RichText, Stroke, TextStyle};

use crate::axis::PlotTransform;
use crate::bounds::PlotPoint;
use crate::items::PlotGeometry;
use crate::plot::PlotUi;

/// One selected anchor per series, found inside the vertical band.
///
/// Built once per frame for all participating series.
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
    /// Horizontal distance in pixels from the pointer.
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
/// Pins are created by pressing **`P`** while hovering the plot.
#[derive(Clone, Debug, Default)]
pub struct PinnedPoints {
    /// Cloned hits from the moment the pin was taken (plot-space values).
    pub hits: Vec<HitPoint>,
    /// The pinned plot-space X used to draw the vertical "pin rail".
    pub plot_x: f64,
}

/// Visual/behavioral settings for pins.
#[derive(Clone)]
pub struct PinOptions {
    /// Stroke for the vertical pin rail.
    pub rail_stroke: Stroke,

    /// Radius of pin markers (in pixels).
    pub marker_radius: f32,

    /// Show a floating panel listing all pins in the corner.
    pub show_panel: bool,

    /// Half-width of the vertical selection band, in screen pixels.
    pub radius_px: f32,
}

impl Default for PinOptions {
    fn default() -> Self {
        Self {
            rail_stroke: Stroke::new(1.5, Color32::from_rgb(255, 200, 64)),
            marker_radius: 5.0,
            show_panel: true,
            radius_px: 50.0,
        }
    }
}

impl PinOptions {
    /// Set the stroke for pin rails.
    #[inline]
    pub fn rail_stroke(mut self, stroke: Stroke) -> Self {
        self.rail_stroke = stroke;
        self
    }

    /// Set the radius of pin markers.
    #[inline]
    pub fn marker_radius(mut self, radius: f32) -> Self {
        self.marker_radius = radius;
        self
    }

    /// Toggle whether to show the floating pins panel.
    #[inline]
    pub fn show_panel(mut self, on: bool) -> Self {
        self.show_panel = on;
        self
    }

    /// Set the selection band radius.
    #[inline]
    pub fn radius_px(mut self, radius: f32) -> Self {
        self.radius_px = radius;
        self
    }
}

// ============================================================================
// Pin memory functions
// ============================================================================

fn pins_mem_id(base: Id) -> Id {
    base.with("band_pins_mem")
}

pub(crate) fn load_pins(ctx: &egui::Context, base: Id) -> Vec<PinnedPoints> {
    ctx.data(|d| d.get_temp::<Vec<PinnedPoints>>(pins_mem_id(base)))
        .unwrap_or_default()
}

pub(crate) fn save_pins(ctx: &egui::Context, base: Id, v: Vec<PinnedPoints>) {
    ctx.data_mut(|d| d.insert_temp(pins_mem_id(base), v));
}

/// Initialize pins for a plot by its ID.
///
/// Call this once (e.g., on first frame) to set up pre-existing pins.
/// The `plot_id` should match the ID used in `Plot::new(plot_id)`.
pub fn init_pins(ctx: &egui::Context, plot_id: impl std::hash::Hash, pins: Vec<PinnedPoints>) {
    let id = Id::new(plot_id);
    ctx.data_mut(|d| d.insert_temp(pins_mem_id(id), pins));
}

// ============================================================================
// Pin drawing functions
// ============================================================================

/// Draws all pin overlays: vertical rails and markers at each pinned point.
fn draw_pins_overlay(
    ctx: &egui::Context,
    pins: &[PinnedPoints],
    transform: PlotTransform,
    frame: Rect,
    visuals: &egui::style::Visuals,
    options: &PinOptions,
) {
    if pins.is_empty() {
        return;
    }
    let painter = egui::Painter::new(
        ctx.clone(),
        egui::LayerId::new(Order::Foreground, Id::new("pins_overlay")),
        frame,
    );

    let label_font = TextStyle::Small.resolve(&ctx.style());

    for (k, group) in pins.iter().enumerate() {
        let x = transform.position_from_point(&PlotPoint::new(group.plot_x, 0.0)).x;
        painter.line_segment(
            [Pos2::new(x, frame.top()), Pos2::new(x, frame.bottom())],
            options.rail_stroke,
        );

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
            painter.circle_filled(p, options.marker_radius + 0.5, h.color);
            painter.circle_stroke(p, options.marker_radius + 0.5, outline);
        }
    }
}

/// Shows a floating Pins panel in the top-right of the plot frame.
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

// ============================================================================
// PlotUi methods for pins
// ============================================================================

impl PlotUi<'_> {
    /// Collect hit points from all series near the current pointer position.
    ///
    /// Returns an empty vector if the pointer is not over the plot.
    /// The `radius_px` parameter controls the horizontal search band width.
    pub fn collect_hits(&self, radius_px: f32) -> Vec<HitPoint> {
        let ctx = self.ctx();
        let visuals = ctx.style().visuals.clone();
        let transform = *self.transform();
        let frame = transform.frame();

        let Some(pointer_screen) = ctx.input(|i| i.pointer.latest_pos()) else {
            return Vec::new();
        };

        // Only show hits if pointer is within the plot frame
        if !frame.contains(pointer_screen) {
            return Vec::new();
        }

        let band_min_x = (pointer_screen.x - radius_px).max(frame.left());
        let band_max_x = (pointer_screen.x + radius_px).min(frame.right());
        if band_max_x <= band_min_x {
            return Vec::new();
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
                    is_highlighted: false,
                });
            }
        }

        // Sort by series name for stable ordering
        hits.sort_by(|a, b| a.series_name.cmp(&b.series_name));
        hits
    }

    /// Show pins overlay and handle pin input (P/U/Delete keys).
    ///
    /// This is a standalone pin component. Use `collect_hits()` first if you
    /// want to pin the current hover position.
    ///
    /// # Example
    /// ```ignore
    /// Plot::new("my_plot").show(ui, |plot_ui| {
    ///     plot_ui.line(Line::new("series", data));
    ///
    ///     let hits = plot_ui.collect_hits(50.0);
    ///     plot_ui.show_pins_with_hits(&PinOptions::default(), &hits);
    /// });
    /// ```
    pub fn show_pins_with_hits(&self, options: &PinOptions, hits: &[HitPoint]) {
        let ctx = self.ctx().clone();
        let visuals = ctx.style().visuals.clone();
        let transform = *self.transform();
        let frame = transform.frame();

        let mut pins = load_pins(&ctx, self.response.id);

        // Draw existing pins
        draw_pins_overlay(&ctx, &pins, transform, *frame, &visuals, options);

        // Show pins panel if enabled
        if options.show_panel && !pins.is_empty() {
            show_pins_panel(&ctx, *frame, &pins);
        }

        // Handle pin input
        if self.response.hovered() {
            let mut pins_changed = false;
            ctx.input(|i| {
                if i.key_pressed(Key::P) && !hits.is_empty() {
                    if let Some(pointer_screen) = i.pointer.latest_pos() {
                        let pointer_plot = transform.value_from_position(pointer_screen);
                        pins.push(PinnedPoints {
                            hits: hits.to_vec(),
                            plot_x: pointer_plot.x,
                        });
                        pins_changed = true;
                    }
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
                save_pins(&ctx, self.response.id, pins);
            }
        }
    }

    /// Show pins overlay using default hit collection.
    ///
    /// Convenience method that collects hits and shows pins in one call.
    pub fn show_pins(&self, options: &PinOptions) {
        let hits = self.collect_hits(options.radius_px);
        self.show_pins_with_hits(options, &hits);
    }

    /// Get the current pins for this plot.
    pub fn get_pins(&self) -> Vec<PinnedPoints> {
        load_pins(self.ctx(), self.response.id)
    }
}
