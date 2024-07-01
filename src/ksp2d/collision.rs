use glam::{dvec2, DVec2};
use log::info;

pub fn triangle_aabb(p1: DVec2, p2: DVec2, p3: DVec2) -> (DVec2, DVec2) {
    (
        dvec2(p1.x.min(p2.x).min(p3.x), p1.y.min(p2.y).min(p3.y)),
        dvec2(p1.x.max(p2.x).max(p3.x), p1.y.max(p2.y).max(p3.y)),
    )
}
pub fn circle_aabb(c_center: DVec2, c_radius: f64) -> (DVec2, DVec2) {
    let disp = DVec2::splat(c_radius);
    (c_center - disp, c_center + disp)
}

pub fn is_aabb_intersected(a_lt: DVec2, a_rb: DVec2, b_lt: DVec2, b_rb: DVec2) -> bool {
    #[inline(always)]
    fn rb_from(a: DVec2, b: DVec2) -> bool {
        a.x >= b.x && a.y >= b.y
    }
    rb_from(a_rb, b_lt) && rb_from(b_rb, a_lt)
}

//true iff segment intersect circle in two points
pub fn is_segment_(seg_p1: DVec2, seg_p2: DVec2, c_center: DVec2, c_radius: f64) -> bool {
    //squared length of segment
    let len = seg_p1.distance_squared(seg_p2);
    //dot product to form closest perpendicular intersection point with line on which segment lies
    let dot = (c_center - seg_p1).dot(seg_p2 - seg_p1) / len;
    //closest point to circle center on segment (may be off segment, but on line with it)
    let closest: DVec2 = seg_p1 + dot * (seg_p2 - seg_p1);
    //intersect or on tangent line to circle
    closest.distance_squared(seg_p1) + closest.distance_squared(seg_p2) <= len
        && c_radius >= c_center.distance(closest)
}
