use emath::Pos2;

/// Returns the x-coordinate of a possible intersection between a line segment
/// from `p1` to `p2` and a horizontal line at the given y-coordinate.
pub fn y_intersection(p1: &Pos2, p2: &Pos2, y: f32) -> Option<f32> {
    ((p1.y > y && p2.y < y) || (p1.y < y && p2.y > y))
        .then_some(((y * (p1.x - p2.x)) - (p1.x * p2.y - p1.y * p2.x)) / (p1.y - p2.y))
}
