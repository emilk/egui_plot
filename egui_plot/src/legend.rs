use std::{collections::BTreeMap, string::String};

use egui::{
    epaint::CircleShape, pos2, vec2, Align, Color32, Direction, Frame, Id, Layout, PointerButton,
    Rect, Response, Sense, Shadow, Shape, TextStyle, Ui, Widget, WidgetInfo, WidgetType,
};

use super::items::PlotItem;

/// Where to place the plot legend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Corner {
    LeftTop,
    RightTop,
    LeftBottom,
    RightBottom,
}

impl Corner {
    pub fn all() -> impl Iterator<Item = Self> {
        [
            Self::LeftTop,
            Self::RightTop,
            Self::LeftBottom,
            Self::RightBottom,
        ]
        .iter()
        .copied()
    }
}

/// How to handle multiple conflicting color for a legend item.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ColorConflictHandling {
    PickFirst,
    PickLast,
    RemoveColor,
}

/// The configuration for a plot legend.
#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Legend {
    pub text_style: TextStyle,
    pub background_alpha: f32,
    pub position: Corner,

    follow_insertion_order: bool,
    color_conflict_handling: ColorConflictHandling,

    /// Used for overriding the `hidden_items` set in [`LegendWidget`].
    hidden_items: Option<ahash::HashSet<Id>>,
}

impl Default for Legend {
    fn default() -> Self {
        Self {
            text_style: TextStyle::Body,
            background_alpha: 0.75,
            position: Corner::RightTop,
            follow_insertion_order: false,
            color_conflict_handling: ColorConflictHandling::RemoveColor,
            hidden_items: None,
        }
    }
}

impl Legend {
    /// Which text style to use for the legend. Default: `TextStyle::Body`.
    #[inline]
    pub fn text_style(mut self, style: TextStyle) -> Self {
        self.text_style = style;
        self
    }

    /// The alpha of the legend background. Default: `0.75`.
    #[inline]
    pub fn background_alpha(mut self, alpha: f32) -> Self {
        self.background_alpha = alpha;
        self
    }

    /// In which corner to place the legend. Default: `Corner::RightTop`.
    #[inline]
    pub fn position(mut self, corner: Corner) -> Self {
        self.position = corner;
        self
    }

    /// Specifies hidden items in the legend configuration to override the existing ones. This
    /// allows the legend traces' visibility to be controlled from the application code.
    #[inline]
    pub fn hidden_items<I>(mut self, hidden_items: I) -> Self
    where
        I: IntoIterator<Item = Id>,
    {
        self.hidden_items = Some(hidden_items.into_iter().collect());
        self
    }

    /// Specifies if the legend item order should be the inserted order.
    /// Default: `false`.
    /// If `true`, the order of the legend items will be the same as the order as they were added.
    /// By default it will be sorted alphabetically.
    #[inline]
    pub fn follow_insertion_order(mut self, follow: bool) -> Self {
        self.follow_insertion_order = follow;
        self
    }

    /// Specifies how to handle conflicting colors for an item.
    #[inline]
    pub fn color_conflict_handling(
        mut self,
        color_conflict_handling: ColorConflictHandling,
    ) -> Self {
        self.color_conflict_handling = color_conflict_handling;
        self
    }
}

#[derive(Clone)]
struct LegendEntry {
    id: Id,
    name: String,
    color: Color32,
    checked: bool,
    hovered: bool,
}

impl LegendEntry {
    fn new(id: Id, name: String, color: Color32, checked: bool) -> Self {
        Self {
            id,
            name,
            color,
            checked,
            hovered: false,
        }
    }

    fn ui(&self, ui: &mut Ui, text_style: &TextStyle) -> Response {
        let Self {
            id: _,
            name,
            color,
            checked,
            hovered: _,
        } = self;

        let font_id = text_style.resolve(ui.style());

        let galley = ui.fonts(|f| f.layout_delayed_color(name.clone(), font_id, f32::INFINITY));

        let icon_size = galley.size().y;
        let icon_spacing = icon_size / 5.0;
        let total_extra = vec2(icon_size + icon_spacing, 0.0);

        let desired_size = total_extra + galley.size();
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

        response.widget_info(|| {
            WidgetInfo::selected(
                WidgetType::Checkbox,
                ui.is_enabled(),
                *checked,
                galley.text(),
            )
        });

        let visuals = ui.style().interact(&response);
        let label_on_the_left = ui.layout().horizontal_placement() == Align::RIGHT;

        let icon_position_x = if label_on_the_left {
            rect.right() - icon_size / 2.0
        } else {
            rect.left() + icon_size / 2.0
        };
        let icon_position = pos2(icon_position_x, rect.center().y);
        let icon_rect = Rect::from_center_size(icon_position, vec2(icon_size, icon_size));

        let painter = ui.painter();

        painter.add(CircleShape {
            center: icon_rect.center(),
            radius: icon_size * 0.5,
            fill: visuals.bg_fill,
            stroke: visuals.bg_stroke,
        });

        if *checked {
            let fill = if *color == Color32::TRANSPARENT {
                ui.visuals().noninteractive().fg_stroke.color
            } else {
                *color
            };
            painter.add(Shape::circle_filled(
                icon_rect.center(),
                icon_size * 0.4,
                fill,
            ));
        }

        let text_position_x = if label_on_the_left {
            rect.right() - icon_size - icon_spacing - galley.size().x
        } else {
            rect.left() + icon_size + icon_spacing
        };

        let text_position = pos2(text_position_x, rect.center().y - 0.5 * galley.size().y);
        painter.galley(text_position, galley, visuals.text_color());

        response
    }
}

#[derive(Clone)]
pub(super) struct LegendWidget {
    rect: Rect,
    entries: Vec<LegendEntry>,
    config: Legend,
}

impl LegendWidget {
    /// Create a new legend from items, the names of items that are hidden and the style of the
    /// text. Returns `None` if the legend has no entries.
    pub(super) fn try_new<'a>(
        rect: Rect,
        config: Legend,
        items: &[Box<dyn PlotItem + 'a>],
        hidden_items: &ahash::HashSet<Id>, // Existing hidden items in the plot memory.
    ) -> Option<Self> {
        // If `config.hidden_items` is not `None`, it is used.
        let hidden_items = config.hidden_items.as_ref().unwrap_or(hidden_items);

        // Collect the legend entries. If multiple items have the same name, they share a
        // checkbox. If their colors don't match, we pick a neutral color for the checkbox.
        let mut keys: BTreeMap<String, usize> = BTreeMap::new();
        let mut entries: BTreeMap<(usize, &str), LegendEntry> = BTreeMap::new();
        items
            .iter()
            .filter(|item| !item.name().is_empty())
            .for_each(|item| {
                let next_entry = entries.len();
                let key = if config.follow_insertion_order {
                    *keys.entry(item.name().to_owned()).or_insert(next_entry)
                } else {
                    // Use the same key if we don't want insertion order
                    0
                };

                entries
                    .entry((key, item.name()))
                    .and_modify(|entry| {
                        if entry.color != item.color() {
                            match config.color_conflict_handling {
                                ColorConflictHandling::PickFirst => (),
                                ColorConflictHandling::PickLast => entry.color = item.color(),
                                ColorConflictHandling::RemoveColor => {
                                    // Multiple items with different colors
                                    entry.color = Color32::TRANSPARENT;
                                }
                            }
                        }
                    })
                    .or_insert_with(|| {
                        let color = item.color();
                        let checked = !hidden_items.contains(&item.id());
                        LegendEntry::new(item.id(), item.name().to_owned(), color, checked)
                    });
            });
        (!entries.is_empty()).then_some(Self {
            rect,
            entries: entries.into_values().collect(),
            config,
        })
    }

    // Get the names of the hidden items.
    pub fn hidden_items(&self) -> ahash::HashSet<Id> {
        self.entries
            .iter()
            .filter_map(|entry| (!entry.checked).then_some(entry.id))
            .collect()
    }

    // Get the name of the hovered items.
    pub fn hovered_item(&self) -> Option<Id> {
        self.entries
            .iter()
            .find_map(|entry| entry.hovered.then_some(entry.id))
    }
}

impl Widget for &mut LegendWidget {
    fn ui(self, ui: &mut Ui) -> Response {
        let LegendWidget {
            rect,
            entries,
            config,
        } = self;

        let main_dir = match config.position {
            Corner::LeftTop | Corner::RightTop => Direction::TopDown,
            Corner::LeftBottom | Corner::RightBottom => Direction::BottomUp,
        };
        let cross_align = match config.position {
            Corner::LeftTop | Corner::LeftBottom => Align::LEFT,
            Corner::RightTop | Corner::RightBottom => Align::RIGHT,
        };
        let layout = Layout::from_main_dir_and_cross_align(main_dir, cross_align);
        let legend_pad = 4.0;
        let legend_rect = rect.shrink(legend_pad);
        let mut legend_ui =
            ui.new_child(egui::UiBuilder::new().max_rect(legend_rect).layout(layout));
        legend_ui
            .scope(|ui| {
                let background_frame = Frame {
                    inner_margin: vec2(8.0, 4.0).into(),
                    corner_radius: ui.style().visuals.window_corner_radius,
                    shadow: Shadow::NONE,
                    fill: ui.style().visuals.extreme_bg_color,
                    stroke: ui.style().visuals.window_stroke(),
                    ..Default::default()
                }
                .multiply_with_opacity(config.background_alpha);
                background_frame
                    .show(ui, |ui| {
                        let mut focus_on_item = None;

                        let response_union = entries
                            .iter_mut()
                            .map(|entry| {
                                let response = entry.ui(ui, &config.text_style);

                                // Handle interactions. Alt-clicking must be deferred to end of loop
                                // since it may affect all entries.
                                handle_interaction_on_legend_item(&response, entry);
                                if response.clicked() && ui.input(|r| r.modifiers.alt) {
                                    focus_on_item = Some(entry.id);
                                }

                                response
                            })
                            .reduce(|r1, r2| r1.union(r2))
                            .expect("No entries in the legend");

                        if let Some(focus_on_item) = focus_on_item {
                            handle_focus_on_legend_item(&focus_on_item, entries);
                        }

                        response_union
                    })
                    .inner
            })
            .inner
    }
}

/// Handle per-entry interactions.
fn handle_interaction_on_legend_item(response: &Response, entry: &mut LegendEntry) {
    entry.checked ^= response.clicked_by(PointerButton::Primary);
    entry.hovered = response.hovered();
}

/// Handle alt-click interaction (which may affect all entries).
fn handle_focus_on_legend_item(clicked_entry: &Id, entries: &mut [LegendEntry]) {
    // if all other items are already hidden, we show everything
    let is_focus_item_only_visible = entries
        .iter()
        .all(|entry| !entry.checked || (clicked_entry == &entry.id));

    // either show everything or show only the focus item
    for entry in entries.iter_mut() {
        entry.checked = is_focus_item_only_visible || clicked_entry == &entry.id;
    }
}
