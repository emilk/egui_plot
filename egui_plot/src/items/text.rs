use crate::builder_methods_for_base;
use crate::{Id, PlotBounds, PlotGeometry, PlotItem, PlotItemBase, PlotPoint, PlotTransform};
use egui::epaint::TextShape;
use egui::{Color32, Shape, Stroke, TextStyle, Ui, WidgetText};
use emath::Align2;
use std::ops::RangeInclusive;

impl Text {
    pub fn new(name: impl Into<String>, position: PlotPoint, text: impl Into<WidgetText>) -> Self {
        Self {
            base: PlotItemBase::new(name.into()),
            text: text.into(),
            position,
            color: Color32::TRANSPARENT,
            anchor: Align2::CENTER_CENTER,
        }
    }

    /// Text color.
    #[inline]
    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        self.color = color.into();
        self
    }

    /// Anchor position of the text. Default is `Align2::CENTER_CENTER`.
    #[inline]
    pub fn anchor(mut self, anchor: Align2) -> Self {
        self.anchor = anchor;
        self
    }

    builder_methods_for_base!();
}

impl PlotItem for Text {
    fn shapes(&self, ui: &Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        let color = if self.color == Color32::TRANSPARENT {
            ui.style().visuals.text_color()
        } else {
            self.color
        };

        let galley = self.text.clone().into_galley(
            ui,
            Some(egui::TextWrapMode::Extend),
            f32::INFINITY,
            TextStyle::Small,
        );

        let pos = transform.position_from_point(&self.position);
        let rect = self.anchor.anchor_size(pos, galley.size());

        shapes.push(TextShape::new(rect.min, galley, color).into());

        if self.base.highlight {
            shapes.push(Shape::rect_stroke(
                rect.expand(1.0),
                1.0,
                Stroke::new(0.5, color),
                egui::StrokeKind::Outside,
            ));
        }
    }

    fn initialize(&mut self, _x_range: RangeInclusive<f64>) {}

    fn color(&self) -> Color32 {
        self.color
    }

    fn geometry(&self) -> PlotGeometry<'_> {
        PlotGeometry::None
    }

    fn bounds(&self) -> PlotBounds {
        let mut bounds = PlotBounds::NOTHING;
        bounds.extend_with(&self.position);
        bounds
    }

    fn base(&self) -> &PlotItemBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut PlotItemBase {
        &mut self.base
    }
}

/// Text inside the plot.
#[derive(Clone)]
pub struct Text {
    base: PlotItemBase,
    pub(crate) text: WidgetText,
    pub(crate) position: PlotPoint,
    pub(crate) color: Color32,
    pub(crate) anchor: Align2,
}
