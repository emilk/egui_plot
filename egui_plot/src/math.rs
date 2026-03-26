use emath::Float as _;
use emath::Pos2;

use crate::axis::PlotTransform;
use crate::items::ClosestElem;
use crate::rect_elem::RectElement;

/// Returns the x-coordinate of a possible intersection between a line segment
/// from `p1` to `p2` and a horizontal line at the given y-coordinate.
pub fn y_intersection(p1: &Pos2, p2: &Pos2, y: f32) -> Option<f32> {
    ((p1.y > y && p2.y < y) || (p1.y < y && p2.y > y))
        .then_some(((y * (p1.x - p2.x)) - (p1.x * p2.y - p1.y * p2.x)) / (p1.y - p2.y))
}

/// Squared distance from point `p` to the line segment `a`–`b`.
pub fn dist_sq_to_segment(p: Pos2, [a, b]: [Pos2; 2]) -> f32 {
    let ab = b - a;
    let ab_len_sq = ab.length_sq();

    if ab_len_sq == 0.0 {
        // Degenerate segment: treat as a single point.
        return p.distance_sq(a);
    }

    let ap = p - a;
    let t = ab.dot(ap) / ab_len_sq;
    let t = t.clamp(0.0, 1.0);
    let closest = a + t * ab;
    p.distance_sq(closest)
}

pub fn find_closest_rect<'a, T>(
    rects: impl IntoIterator<Item = &'a T>,
    point: Pos2,
    transform: &PlotTransform,
) -> Option<ClosestElem>
where
    T: 'a + RectElement,
{
    rects
        .into_iter()
        .enumerate()
        .map(|(index, bar)| {
            let bar_rect = transform.rect_from_values(&bar.bounds_min(), &bar.bounds_max());
            let dist_sq = bar_rect.distance_sq_to_pos(point);

            ClosestElem { index, dist_sq }
        })
        .min_by_key(|e| e.dist_sq.ord())
}
