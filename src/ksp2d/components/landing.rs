use glam::DVec2;
use legion::Entity;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LandingRelation {
    pub planet_id: Entity,
    pub angle_position: DVec2,
}
