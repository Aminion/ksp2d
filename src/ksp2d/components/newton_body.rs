use glam::DVec2;

use crate::Dt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NewtonBody {
    pub mass: f64,
    pub acc: DVec2,
    pub vel: DVec2,
    pub angle: DVec2,
    pub angular_vel: f64,
    pub pos: DVec2,
    pub prev_pos: DVec2,
}

impl NewtonBody {
    pub fn update_a(&mut self, dt: &Dt) {
        self.angle = self
            .angle
            .rotate(DVec2::from_angle(self.angular_vel * dt.0));
    }
}
