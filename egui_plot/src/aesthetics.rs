use egui::Shape;
use egui::Stroke;
use egui::epaint::ColorMode;
use egui::epaint::PathStroke;
use emath::Pos2;
use emath::Rect;
use emath::pos2;

/// Solid, dotted, dashed, etc.
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum LineStyle {
    Solid,
    Dotted { spacing: f32 },
    Dashed { length: f32 },
}

impl LineStyle {
    pub fn dashed_loose() -> Self {
        Self::Dashed { length: 10.0 }
    }

    pub fn dashed_dense() -> Self {
        Self::Dashed { length: 5.0 }
    }

    pub fn dotted_loose() -> Self {
        Self::Dotted { spacing: 10.0 }
    }

    pub fn dotted_dense() -> Self {
        Self::Dotted { spacing: 5.0 }
    }

    pub(crate) fn style_line(&self, line: Vec<Pos2>, mut stroke: PathStroke, highlight: bool, shapes: &mut Vec<Shape>) {
        let path_stroke_color = match &stroke.color {
            ColorMode::Solid(c) => *c,
            ColorMode::UV(callback) => callback(Rect::from_min_max(pos2(0., 0.), pos2(0., 0.)), pos2(0., 0.)),
        };
        match line.len() {
            0 => {}
            1 => {
                let mut radius = stroke.width / 2.0;
                if highlight {
                    radius *= 2f32.sqrt();
                }
                shapes.push(Shape::circle_filled(line[0], radius, path_stroke_color));
            }
            _ => {
                match self {
                    Self::Solid => {
                        if highlight {
                            stroke.width *= 2.0;
                        }
                        shapes.push(Shape::line(line, stroke));
                    }
                    Self::Dotted { spacing } => {
                        // Take the stroke width for the radius even though it's not "correct",
                        // otherwise the dots would become too small.
                        let mut radius = stroke.width;
                        if highlight {
                            radius *= 2f32.sqrt();
                        }
                        shapes.extend(Shape::dotted_line(&line, path_stroke_color, *spacing, radius));
                    }
                    Self::Dashed { length } => {
                        if highlight {
                            stroke.width *= 2.0;
                        }
                        let golden_ratio = (5.0_f32.sqrt() - 1.0) / 2.0; // 0.61803398875
                        shapes.extend(Shape::dashed_line(
                            &line,
                            Stroke::new(stroke.width, path_stroke_color),
                            *length,
                            length * golden_ratio,
                        ));
                    }
                }
            }
        }
    }
}

impl std::fmt::Display for LineStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Solid => write!(f, "Solid"),
            Self::Dotted { spacing } => write!(f, "Dotted({spacing} px)"),
            Self::Dashed { length } => write!(f, "Dashed({length} px)"),
        }
    }
}

/// Determines whether a plot element is vertically or horizontally oriented.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Default for Orientation {
    fn default() -> Self {
        Self::Vertical
    }
}

/// Circle, Diamond, Square, Cross, …
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MarkerShape {
    Circle,
    Diamond,
    Square,
    Cross,
    Plus,
    Up,
    Down,
    Left,
    Right,
    Asterisk,
}

impl MarkerShape {
    /// Get a vector containing all marker shapes.
    pub fn all() -> impl ExactSizeIterator<Item = Self> {
        [
            Self::Circle,
            Self::Diamond,
            Self::Square,
            Self::Cross,
            Self::Plus,
            Self::Up,
            Self::Down,
            Self::Left,
            Self::Right,
            Self::Asterisk,
        ]
        .iter()
        .copied()
    }
}
