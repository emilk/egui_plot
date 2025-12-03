use emath::{Float, Pos2};
use crate::{ClosestElem, PlotTransform};
use crate::rect_elem::RectElement;

/// Returns the x-coordinate of a possible intersection between a line segment
/// from `p1` to `p2` and a horizontal line at the given y-coordinate.
pub fn y_intersection(p1: &Pos2, p2: &Pos2, y: f32) -> Option<f32> {
    ((p1.y > y && p2.y < y) || (p1.y < y && p2.y > y))
        .then_some(((y * (p1.x - p2.x)) - (p1.x * p2.y - p1.y * p2.x)) / (p1.y - p2.y))
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