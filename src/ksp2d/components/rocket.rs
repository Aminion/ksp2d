use legion::Entity;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rocket {
    pub a: f64,
    pub celestial_body: Entity,
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
}
