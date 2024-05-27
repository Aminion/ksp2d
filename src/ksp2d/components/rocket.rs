#[derive(PartialEq, Eq, Hash)]
pub enum PlayerInput {
    MoveLeft,
    MoveRight,
    MoveForward,
    MoveBackward,
    RotateLeft,
    RotateRight,
}
