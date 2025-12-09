//! Multi-series tooltip overlay.
//!
//! Shows a tooltip with values from all series near the pointer position.
//!
//! ## Usage
//!
//! ```ignore
//! Plot::new("my_plot").show(ui, |plot_ui| {
//!     plot_ui.line(Line::new("series", data));
//!
//!     // Show tooltip only
//!     plot_ui.show_tooltip(&TooltipOptions::default());
//! });
//! ```
//!
//! ## Combined with Pins
//!
//! ```ignore
//! Plot::new("my_plot").show(ui, |plot_ui| {
//!     plot_ui.line(Line::new("series", data));
//!
//!     // Collect hits once, use for both
//!     let hits = plot_ui.collect_hits(50.0);
//!     plot_ui.show_pins_with_hits(&PinOptions::default(), &hits);
//!     plot_ui.show_tooltip_with_hits(&TooltipOptions::default(), &hits);
//! });
//! ```

use egui::{Color32, Grid, Id, Pos2, Rect, RichText, Stroke};

use crate::overlays::pin::HitPoint;
use crate::plot::PlotUi;

/// Visual/behavioral settings for the tooltip.
#[derive(Clone)]
pub struct TooltipOptions {
    /// Fill the vertical band region for visual feedback.
    pub draw_band_fill: bool,

    /// Draw a vertical guide line at the current pointer X.
    pub draw_vertical_guide: bool,

    /// Color for the band fill (typically a faint translucent color).
    pub band_fill: Color32,

    /// Stroke for the vertical guide line.
    pub guide_stroke: Stroke,

    /// Show markers at hit points on each series.
    pub show_markers: bool,

    /// Radius of the on-canvas hit markers (in pixels).
    pub marker_radius: f32,

    /// If `Some(distance)`, highlight lines whose nearest point is within
    /// `distance` pixels (Manhattan distance) from the crosshair.
    pub highlight_lines_distance: Option<f32>,

    /// Half-width of the vertical selection band, in screen pixels.
    pub radius_px: f32,

    /// Horizontal gap between the vertical ruler and the tooltip.
    pub tooltip_horizontal_gap: f32,

    /// Vertical gap between the anchor point and the tooltip.
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
            radius_px: 50.0,
            tooltip_horizontal_gap: 10.0,
            tooltip_vertical_gap: 10.0,
        }
    }
}

impl TooltipOptions {
    /// Set the maximum Manhattan distance for line highlighting.
    #[inline]
    pub fn highlight_lines_distance(mut self, distance: Option<f32>) -> Self {
        self.highlight_lines_distance = distance;
        self
    }

    /// Toggle whether to show markers at hit points.
    #[inline]
    pub fn show_markers(mut self, on: bool) -> Self {
        self.show_markers = on;
        self
    }

    /// Set the horizontal gap between ruler and tooltip.
    #[inline]
    pub fn tooltip_horizontal_gap(mut self, gap: f32) -> Self {
        self.tooltip_horizontal_gap = gap;
        self
    }

    /// Set the vertical gap between anchor and tooltip.
    #[inline]
    pub fn tooltip_vertical_gap(mut self, gap: f32) -> Self {
        self.tooltip_vertical_gap = gap;
        self
    }

    /// Set both horizontal and vertical tooltip gaps.
    #[inline]
    pub fn tooltip_gap(mut self, horizontal: f32, vertical: f32) -> Self {
        self.tooltip_horizontal_gap = horizontal;
        self.tooltip_vertical_gap = vertical;
        self
    }

    /// Set the selection band radius.
    #[inline]
    pub fn radius_px(mut self, radius: f32) -> Self {
        self.radius_px = radius;
        self
    }

    /// Toggle band fill drawing.
    #[inline]
    pub fn draw_band_fill(mut self, on: bool) -> Self {
        self.draw_band_fill = on;
        self
    }

    /// Toggle vertical guide drawing.
    #[inline]
    pub fn draw_vertical_guide(mut self, on: bool) -> Self {
        self.draw_vertical_guide = on;
        self
    }
}

impl PlotUi<'_> {
    /// Show tooltip with the provided hits.
    ///
    /// Use this when you've already collected hits (e.g., for sharing with pins).
    pub fn show_tooltip_with_hits(&mut self, options: &TooltipOptions, hits: &[HitPoint]) {
        self.show_tooltip_with_hits_custom(options, hits, default_tooltip_ui);
    }

    /// Show tooltip with custom UI builder and provided hits.
    pub fn show_tooltip_with_hits_custom(
        &mut self,
        options: &TooltipOptions,
        hits: &[HitPoint],
        ui_builder: impl FnOnce(&mut egui::Ui, &[HitPoint]),
    ) {
        if hits.is_empty() {
            return;
        }

        let ctx = self.ctx().clone();
        let visuals = ctx.style().visuals.clone();
        let transform = *self.transform();
        let frame = transform.frame();

        let Some(pointer_screen) = ctx.input(|i| i.pointer.latest_pos()) else {
            return;
        };

        // Apply highlighting to lines
        let mut hits_with_highlight: Vec<HitPoint> = hits.to_vec();
        if let Some(highlight_distance) = options.highlight_lines_distance {
            let names_within_distance: ahash::AHashSet<&str> = hits_with_highlight
                .iter_mut()
                .filter_map(|h| {
                    if h.manhattan_distance() <= highlight_distance {
                        h.is_highlighted = true;
                        Some(h.series_name.as_str())
                    } else {
                        None
                    }
                })
                .collect();

            for item in &mut self.items {
                if names_within_distance.contains(item.name()) {
                    item.highlight();
                }
            }
        }

        // Draw band fill and vertical guide
        let r = options.radius_px;
        let band_min_x = (pointer_screen.x - r).max(frame.left());
        let band_max_x = (pointer_screen.x + r).min(frame.right());

        {
            let painter = egui::Painter::new(ctx.clone(), self.response.layer_id, *frame);

            if options.draw_band_fill && band_max_x > band_min_x {
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

        // Draw markers on foreground layer
        if options.show_markers {
            let marker_painter = egui::Painter::new(
                ctx.clone(),
                egui::LayerId::new(egui::Order::Foreground, egui::Id::new("tooltip_markers")),
                *frame,
            );
            for h in &hits_with_highlight {
                marker_painter.circle_filled(h.screen_pos, options.marker_radius, h.color);
                marker_painter.circle_stroke(
                    h.screen_pos,
                    options.marker_radius,
                    Stroke::new(1.0, visuals.window_stroke().color),
                );
            }
        }

        // Calculate tooltip position
        let frame_center_x = frame.center().x;
        let horizontal_offset = if pointer_screen.x < frame_center_x {
            options.tooltip_horizontal_gap
        } else {
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
            ui_builder(ui, &hits_with_highlight);
        });
    }

    /// Show tooltip with default hit collection.
    ///
    /// Convenience method that collects hits and shows tooltip in one call.
    pub fn show_tooltip(&mut self, options: &TooltipOptions) {
        let hits = self.collect_hits(options.radius_px);
        self.show_tooltip_with_hits(options, &hits);
    }

    /// Show tooltip with custom UI builder.
    pub fn show_tooltip_custom(
        &mut self,
        options: &TooltipOptions,
        ui_builder: impl FnOnce(&mut egui::Ui, &[HitPoint]),
    ) {
        let hits = self.collect_hits(options.radius_px);
        self.show_tooltip_with_hits_custom(options, &hits, ui_builder);
    }
}

/// Default tooltip content: a compact table with a row per hit.
fn default_tooltip_ui(ui: &mut egui::Ui, hits: &[HitPoint]) {
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
}
