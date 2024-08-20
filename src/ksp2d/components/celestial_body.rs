use glam::DVec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CelestialBody {
    pub mass: f64,
    pub acc: DVec2,
    pub vel: DVec2,
    pub a: f64,
    pub a_vel: f64,
    pub pos: DVec2,
    pub prev_pos: DVec2,
}
#[inline]
fn change_radian(current: f64, change: f64) -> f64 {
    (current + change) % (2.0 * std::f64::consts::PI)
}

impl CelestialBody {
    pub fn change_a_vel(&mut self, change: f64) {
        self.a_vel = change_radian(self.a_vel, change);
    }

    pub fn change_vel(&mut self, change: f64) {
        self.a = change_radian(self.a, change);
    }
}
