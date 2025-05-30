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
    let mut query = <(Entity, &NewtonBody)>::query().filter(component::<CelestialBody>());
    let mut obj: Vec<_> = query.iter(world).collect();
    obj.sort_by_cached_key(|o| o.1.pos.distance_squared(n_body.pos) as u64);
    let (p_id, _) = obj.first().unwrap();
    ccb.0 = **p_id;
}
