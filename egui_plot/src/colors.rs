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