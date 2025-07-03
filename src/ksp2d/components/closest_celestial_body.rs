use glam::DVec2;
use legion::Entity;

pub struct ClosestCelestialBody {
    pub id: Entity,
    pub closest_surface_point: DVec2,
}
