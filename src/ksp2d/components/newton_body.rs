use glam::DVec2;

use crate::Dt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NewtonBody {
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
    const DOUBLE_PI: f64 = std::f64::consts::PI * 2.0;
    (DOUBLE_PI + x) % DOUBLE_PI
}

impl NewtonBody {
    pub fn update_a(&mut self, dt: &Dt) {
        self.a = bound_radian(self.a + self.a_vel * dt.0);
    }
}
