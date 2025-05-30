use core::f64;

use glam::DVec2;

pub const AVERSE_TRUST: f64 = 343430000000000.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Engine {
    pub vector: DVec2,
    pub throttle: f64,
}

const THROTTLE_MIN: f64 = 0.0;
const THROTTLE_MAX: f64 = 1.0;

impl Engine {
    pub fn trust(self) -> DVec2 {
        self.vector * self.throttle
    }

    pub fn change_throttle(&mut self, t: f64) {
        self.set_throttle(self.throttle + t);
    }

    pub fn set_throttle(&mut self, t: f64) {
        self.throttle = t.clamp(THROTTLE_MIN, THROTTLE_MAX);
    }

    pub fn full(&mut self) {
        self.throttle = THROTTLE_MAX;
    }

    pub fn disable(&mut self) {
        self.throttle = THROTTLE_MIN;
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rocket {
    pub engine_averse: Engine,
    pub engine_reverse: Engine,
    pub engine_left: Engine,
    pub engine_right: Engine,
}

impl Rocket {
    pub fn new() -> Rocket {
        Rocket {
            engine_averse: Engine {
                vector: DVec2::NEG_Y * AVERSE_TRUST,
                throttle: 0.0,
            },
            engine_reverse: Engine {
                vector: DVec2::Y * AVERSE_TRUST,
                throttle: 0.0,
            },
            engine_left: Engine {
                vector: DVec2::X * AVERSE_TRUST,
                throttle: 0.0,
            },
            engine_right: Engine {
                vector: DVec2::NEG_X * AVERSE_TRUST,
                throttle: 0.0,
            },
        }
    }

    pub fn trust(self) -> DVec2 {
        self.engine_averse.trust()
            + self.engine_reverse.trust()
            + self.engine_left.trust()
            + self.engine_right.trust()
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum PlayerInput {
    MoveLeft,
    MoveRight,
    MoveForward,
    MoveBackward,
    RotateLeft,
    RotateRight,
    WindowResize(i32, i32),
    SwitchCamera,
}
