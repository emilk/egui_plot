use egui::{Color32, Rgba, Stroke, Ui};
use emath::NumExt;

pub(crate) fn rulers_color(ui: &Ui) -> Color32 {
    if ui.visuals().dark_mode {
        Color32::from_gray(100).additive()
    } else {
        Color32::from_black_alpha(180)
    }
}

pub(crate) fn highlighted_color(mut stroke: Stroke, fill: Color32) -> (Stroke, Color32) {
    stroke.width *= 2.0;

    let mut fill = Rgba::from(fill);
    if fill.is_additive() {
        // Make slightly brighter
        fill = 1.3 * fill;
    } else {
        // Make more opaque:
        let fill_alpha = (2.0 * fill.a()).at_most(1.0);
        fill = fill.to_opaque().multiply(fill_alpha);
    }

    (stroke, fill.into())
}

pub const DEFAULT_FILL_ALPHA: f32 = 0.05;
/// Default base colors. Used for now only in heatmap palette.
pub const BASE_COLORS: [Color32; 10] = [
    Color32::from_rgb(48, 18, 59),
    Color32::from_rgb(35, 106, 141),
    Color32::from_rgb(30, 160, 140),
    Color32::from_rgb(88, 200, 98),
    Color32::from_rgb(164, 223, 39),
    Color32::from_rgb(228, 223, 14),
    Color32::from_rgb(250, 187, 13),
    Color32::from_rgb(246, 135, 8),
    Color32::from_rgb(213, 68, 2),
    Color32::from_rgb(122, 4, 2),
];

/// Determine a color from a 0-1 strength value.
pub fn color_from_strength(ui: &Ui, strength: f32) -> Color32 {
    let base_color = ui.visuals().text_color();
    base_color.gamma_multiply(strength.sqrt())
}