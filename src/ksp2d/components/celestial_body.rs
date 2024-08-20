use glam::DVec2;

use crate::Dt;

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
fn bound_radian(x: f64) -> f64 {
    const PI_SQ: f64 = std::f64::consts::PI * 2.0;
    (PI_SQ + x) % PI_SQ
}

impl CelestialBody {
    pub fn update_a(&mut self, dt: &Dt) {
        self.a = bound_radian(self.a + self.a_vel * dt.0);
    }
}