use glam::{DVec2, Vec2};
use legion::*;
use world::SubWorld;

use crate::ksp2d::components::newton_body::NewtonBody;
use crate::ksp2d::components::{
    celestial_body::CelestialBody, closest_celestial_body::ClosestCelestialBody,
};

#[system(for_each)]
#[read_component(Entity)]
#[read_component(NewtonBody)]
#[read_component(CelestialBody)]
#[write_component(ClosestCelestialBody)]
pub fn closest_celestial(world: &SubWorld, n_body: &NewtonBody, ccb: &mut ClosestCelestialBody) {
    let mut query = <(Entity, &NewtonBody, &CelestialBody)>::query();
    let mut obj: Vec<_> = query.iter(world).collect();
    obj.sort_by_cached_key(|o| o.1.pos.distance_squared(n_body.pos) as u64);
    let (p_id, p_n_body, p_c_body) = *obj.first().unwrap();
    ccb.id = *p_id;
    let a = DVec2::Y
        .with_x(p_c_body.radius)
        .rotate(n_body.pos.normalize())
        + p_n_body.pos;
    ccb.closest_surface_point = a;
}
