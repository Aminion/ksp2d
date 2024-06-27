use glam::DVec2;

//true iff segment intersect circle in two points
pub fn is_segment_intersect_circle(
    seg_p1: DVec2,
    seg_p2: DVec2,
    c_center: DVec2,
    c_radius: f64,
) -> bool {
    //length of segment
    let len = seg_p1.distance(seg_p2);
    //dot product to form closest perpendicular intersection point with line on which segment lies
    let dot = (c_center - seg_p1).dot(seg_p2 - seg_p1) / len.powi(2);
    //closest point to circle center on segment (may be off segment, but on line with it)
    let closest: DVec2 = seg_p1 + dot * (seg_p2 - seg_p1);
    //intersect or on tangent line to circle
    c_radius >= c_center.distance(closest)
}
