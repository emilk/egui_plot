use std::ops::RangeInclusive;
use std::sync::Arc;

use egui::Color32;
use egui::Id;
use egui::Mesh;
use egui::Pos2;
use egui::Shape;
use egui::Stroke;
use egui::Ui;

use super::DEFAULT_FILL_ALPHA;
use crate::PlotBounds;
use crate::PlotGeometry;
use crate::PlotItem;
use crate::PlotItemBase;
use crate::PlotPoint;
use crate::PlotPoints;
use crate::PlotTransform;

/// A filled area between two lines.
///
/// Takes x-coordinates and corresponding y_min and y_max values, and fills the area between them.
/// Useful for visualizing confidence intervals, ranges, and uncertainty bands.
pub struct FilledArea {
    base: PlotItemBase,
    /// Lower boundary line (x, y_min pairs)
    lower_line: Vec<PlotPoint>,
    /// Upper boundary line (x, y_max pairs)
    upper_line: Vec<PlotPoint>,
    /// Fill color for the area
    fill_color: Color32,
    /// Optional stroke for the boundaries
    stroke: Option<Stroke>,
}

impl FilledArea {
    /// Create a new filled area between two lines.
    ///
    /// # Arguments
    /// * `name` - Name of this plot item (shows in legend)
    /// * `xs` - X coordinates
    /// * `ys_min` - Lower Y values
    /// * `ys_max` - Upper Y values
    ///
    /// All slices must have the same length.
    ///
    /// # Panics
    /// Panics if the slices don't have the same length.
    pub fn new(name: impl Into<String>, xs: &[f64], ys_min: &[f64], ys_max: &[f64]) -> Self {
        assert_eq!(xs.len(), ys_min.len(), "xs and ys_min must have the same length");
        assert_eq!(xs.len(), ys_max.len(), "xs and ys_max must have the same length");

        let lower_line: Vec<PlotPoint> = xs
            .iter()
            .zip(ys_min.iter())
            .map(|(&x, &y)| PlotPoint::new(x, y))
            .collect();

        let upper_line: Vec<PlotPoint> = xs
            .iter()
            .zip(ys_max.iter())
            .map(|(&x, &y)| PlotPoint::new(x, y))
            .collect();

        Self {
            base: PlotItemBase::new(name.into()),
            lower_line,
            upper_line,
            fill_color: Color32::from_gray(128).linear_multiply(DEFAULT_FILL_ALPHA),
            stroke: None,
        }
    }

    /// Set the fill color for the area.
    #[inline]
    pub fn fill_color(mut self, color: impl Into<Color32>) -> Self {
        self.fill_color = color.into();
        self
    }

    /// Add a stroke around the boundaries of the filled area.
    #[inline]
    pub fn stroke(mut self, stroke: impl Into<Stroke>) -> Self {
        self.stroke = Some(stroke.into());
        self
    }

    /// Name of this plot item.
    ///
    /// This name will show up in the plot legend, if legends are turned on.
    #[expect(clippy::needless_pass_by_value)]
    #[inline]
    pub fn name(mut self, name: impl ToString) -> Self {
        self.base_mut().name = name.to_string();
        self
    }

    /// Highlight this plot item.
    #[inline]
    pub fn highlight(mut self, highlight: bool) -> Self {
        self.base_mut().highlight = highlight;
        self
    }

    /// Allow hovering this item in the plot. Default: `true`.
    #[inline]
    pub fn allow_hover(mut self, hovering: bool) -> Self {
        self.base_mut().allow_hover = hovering;
        self
    }

    /// Sets the id of this plot item.
    #[inline]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.base_mut().id = id.into();
        self
    }
}

impl PlotItem for FilledArea {
    fn shapes(&self, _ui: &Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        if self.lower_line.is_empty() {
            return;
        }

        let n = self.lower_line.len();

        // Create a mesh for the filled area
        let mut mesh = Mesh::default();
        mesh.reserve_triangles((n - 1) * 2);
        mesh.reserve_vertices(n * 2);

        // Add vertices for upper and lower lines
        for point in &self.upper_line {
            let pos = transform.position_from_point(point);
            mesh.colored_vertex(pos, self.fill_color);
        }
        for point in &self.lower_line {
            let pos = transform.position_from_point(point);
            mesh.colored_vertex(pos, self.fill_color);
        }

        // Create triangles connecting upper and lower lines
        for i in 0..(n - 1) {
            // Each quad is split into two triangles
            // Triangle 1: upper[i], lower[i], upper[i+1]
            mesh.add_triangle(i as u32, (n + i) as u32, (i + 1) as u32);
            // Triangle 2: lower[i], lower[i+1], upper[i+1]
            mesh.add_triangle((n + i) as u32, (n + i + 1) as u32, (i + 1) as u32);
        }

        shapes.push(Shape::Mesh(Arc::new(mesh)));

        // Draw optional stroke around boundaries
        if let Some(stroke) = self.stroke {
            // Upper boundary line
            let upper_points: Vec<Pos2> = self
                .upper_line
                .iter()
                .map(|point| transform.position_from_point(point))
                .collect();
            shapes.push(Shape::line(upper_points, stroke));

            // Lower boundary line
            let lower_points: Vec<Pos2> = self
                .lower_line
                .iter()
                .map(|point| transform.position_from_point(point))
                .collect();
            shapes.push(Shape::line(lower_points, stroke));
        }
    }

    fn initialize(&mut self, _x_range: RangeInclusive<f64>) {
        // No initialization needed for explicit data
    }

    fn color(&self) -> Color32 {
        self.fill_color
    }

    fn geometry(&self) -> PlotGeometry<'_> {
        // Return all points (both min and max boundaries) for hit testing
        PlotGeometry::None
    }

    fn bounds(&self) -> PlotBounds {
        // Calculate bounds from all points
        let mut all_points = self.lower_line.clone();
        all_points.extend(self.upper_line.iter());
        PlotPoints::Owned(all_points).bounds()
    }

    fn base(&self) -> &PlotItemBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut PlotItemBase {
        &mut self.base
    }
}
