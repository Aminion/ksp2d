#[derive(PartialEq, Eq, Hash, Debug)]
pub enum PlayerInput {
    MoveLeft,
    MoveRight,
    MoveForward,
    MoveBackward,
    RotateLeft,
    RotateRight,
}
