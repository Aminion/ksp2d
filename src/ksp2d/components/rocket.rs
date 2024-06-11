use glam::DVec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub p: DVec2,
    pub a: f64,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum PlayerInput {
    MoveLeft,
    MoveRight,
    MoveForward,
    MoveBackward,
    RotateLeft,
    RotateRight,
}
