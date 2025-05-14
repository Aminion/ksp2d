#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rocket {}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum PlayerInput {
    MoveLeft,
    MoveRight,
    MoveForward,
    MoveBackward,
    RotateLeft,
    RotateRight,
    WindowResize(i32, i32),
    SwitchCamera
}
