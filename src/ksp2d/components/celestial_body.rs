use glam::DVec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Obj {
    pub pos: DVec2,
    pub vel: DVec2,
    pub acc: DVec2,
    pub mass: f64,
}
