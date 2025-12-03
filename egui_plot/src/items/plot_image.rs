use std::ops::RangeInclusive;

use egui::Color32;
use egui::CornerRadius;
use egui::ImageOptions;
use egui::Shape;
use egui::Stroke;
use egui::TextureId;
use egui::Ui;
use emath::Rect;
use emath::Rot2;
use emath::Vec2;
use emath::pos2;

use crate::Id;
use crate::bounds::PlotBounds;
use crate::data::PlotPoint;
use crate::items::PlotGeometry;
use crate::items::PlotItem;
use crate::items::PlotItemBase;
use crate::transform::PlotTransform;

/// An image in the plot.
#[derive(Clone)]
pub struct PlotImage {
    base: PlotItemBase,
    pub(crate) position: PlotPoint,
    pub(crate) texture_id: TextureId,
    pub(crate) uv: Rect,
    pub(crate) size: Vec2,
    pub(crate) rotation: f64,
    pub(crate) bg_fill: Color32,
    pub(crate) tint: Color32,
}

impl PlotImage {
    /// Create a new image with position and size in plot coordinates.
    pub fn new(
        name: impl Into<String>,
        texture_id: impl Into<TextureId>,
        center_position: PlotPoint,
        size: impl Into<Vec2>,
    ) -> Self {
        Self {
            base: PlotItemBase::new(name.into()),
            position: center_position,
            texture_id: texture_id.into(),
            uv: Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
            size: size.into(),
            rotation: 0.0,
            bg_fill: Default::default(),
            tint: Color32::WHITE,
        }
    }

    /// Select UV range. Default is (0,0) in top-left, (1,1) bottom right.
    #[inline]
    pub fn uv(mut self, uv: impl Into<Rect>) -> Self {
        self.uv = uv.into();
        self
    }

    /// A solid color to put behind the image. Useful for transparent images.
    #[inline]
    pub fn bg_fill(mut self, bg_fill: impl Into<Color32>) -> Self {
        self.bg_fill = bg_fill.into();
        self
    }

    /// Multiply image color with this. Default is WHITE (no tint).
    #[inline]
    pub fn tint(mut self, tint: impl Into<Color32>) -> Self {
        self.tint = tint.into();
        self
    }

    /// Rotate the image counter-clockwise around its center by an angle in
    /// radians.
    #[inline]
    pub fn rotate(mut self, angle: f64) -> Self {
        self.rotation = angle;
        self
    }

    /// Name of this plot item.
    ///
    /// This name will show up in the plot legend, if legends are turned on.
    ///
    /// Setting the name via this method does not change the item's id, so you
    /// can use it to change the name dynamically between frames without
    /// losing the item's state. You should make sure the name passed to
    /// [`Self::new`] is unique and stable for each item, or set unique and
    /// stable ids explicitly via [`Self::id`].
    #[expect(clippy::needless_pass_by_value)]
    #[inline]
    pub fn name(mut self, name: impl ToString) -> Self {
        self.base_mut().name = name.to_string();
        self
    }

    /// Highlight this plot item, typically by scaling it up.
    ///
    /// If false, the item may still be highlighted via user interaction.
    #[inline]
    pub fn highlight(mut self, highlight: bool) -> Self {
        self.base_mut().highlight = highlight;
        self
    }

    /// Allowed hovering this item in the plot. Default: `true`.
    #[inline]
    pub fn allow_hover(mut self, hovering: bool) -> Self {
        self.base_mut().allow_hover = hovering;
        self
    }

    /// Sets the id of this plot item.
    ///
    /// By default the id is determined from the name passed to [`Self::new`],
    /// but it can be explicitly set to a different value.
    #[inline]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.base_mut().id = id.into();
        self
    }
}

impl PlotItem for PlotImage {
    fn shapes(&self, ui: &Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        let Self {
            position,
            rotation,
            texture_id,
            uv,
            size,
            bg_fill,
            tint,
            base,
            ..
        } = self;
        let image_screen_rect = {
            let left_top = PlotPoint::new(position.x - 0.5 * size.x as f64, position.y - 0.5 * size.y as f64);
            let right_bottom = PlotPoint::new(position.x + 0.5 * size.x as f64, position.y + 0.5 * size.y as f64);
            let left_top_screen = transform.position_from_point(&left_top);
            let right_bottom_screen = transform.position_from_point(&right_bottom);
            Rect::from_two_pos(left_top_screen, right_bottom_screen)
        };
        let screen_rotation = -*rotation as f32;

        egui::paint_texture_at(
            ui.painter(),
            image_screen_rect,
            &ImageOptions {
                uv: *uv,
                bg_fill: *bg_fill,
                tint: *tint,
                rotation: Some((Rot2::from_angle(screen_rotation), Vec2::splat(0.5))),
                corner_radius: CornerRadius::ZERO,
            },
            &(*texture_id, image_screen_rect.size()).into(),
        );
        if base.highlight {
            let center = image_screen_rect.center();
            let rotation = Rot2::from_angle(screen_rotation);
            let outline = [
                image_screen_rect.right_bottom(),
                image_screen_rect.right_top(),
                image_screen_rect.left_top(),
                image_screen_rect.left_bottom(),
            ]
            .iter()
            .map(|point| center + rotation * (*point - center))
            .collect();
            shapes.push(Shape::closed_line(
                outline,
                Stroke::new(1.0, ui.visuals().strong_text_color()),
            ));
        }
    }

    fn initialize(&mut self, _x_range: RangeInclusive<f64>) {}

    fn color(&self) -> Color32 {
        Color32::TRANSPARENT
    }

    fn geometry(&self) -> PlotGeometry<'_> {
        PlotGeometry::None
    }

    fn bounds(&self) -> PlotBounds {
        let mut bounds = PlotBounds::NOTHING;
        let left_top = PlotPoint::new(
            self.position.x as f32 - self.size.x / 2.0,
            self.position.y as f32 - self.size.y / 2.0,
        );
        let right_bottom = PlotPoint::new(
            self.position.x as f32 + self.size.x / 2.0,
            self.position.y as f32 + self.size.y / 2.0,
        );
        bounds.extend_with(&left_top);
        bounds.extend_with(&right_bottom);
        bounds
    }

    fn base(&self) -> &PlotItemBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut PlotItemBase {
        &mut self.base
    }
}
