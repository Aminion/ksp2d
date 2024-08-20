use glam::DVec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CelestialBody {
    pub mass: f64,
    pub acc: DVec2,
    pub vel: DVec2,
    pub a_vel: f64,
    pub pos: DVec2,
    pub prev_pos: DVec2,
}
