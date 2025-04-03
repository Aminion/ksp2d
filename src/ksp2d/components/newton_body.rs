use glam::DVec2;

use crate::Dt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NewtonBody {
    pub mass: f64,
    pub acc: DVec2,
    pub vel: DVec2,
    pub angle: f64,
    pub angular_vel: f64,
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
        self.angle = bound_radian(self.angle + self.angular_vel * dt.0);
    }
}
