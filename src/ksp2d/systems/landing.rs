use legion::{world::SubWorld, *};
use systems::CommandBuffer;

use crate::ksp2d::components::{
    celestial_body::CelestialBody, closest_celestial_body::ClosestCelestialBody,
    landing::LandingRelation, newton_body::NewtonBody, rocket::Rocket,
};

#[system(for_each)]
#[filter(!component::<LandingRelation>())]
#[read_component(Entity)]
#[read_component(NewtonBody)]
#[read_component(CelestialBody)]
#[read_component(ClosestCelestialBody)]
pub fn landing(
    world: &SubWorld,
    command_buffer: &mut CommandBuffer,
    _: &Rocket,
    e: &Entity,
    n_body: &NewtonBody,
    ccb: &ClosestCelestialBody,
) {
    let closest_celestial = world.entry_ref(ccb.id).unwrap();
    let celestial_comp = closest_celestial.get_component::<CelestialBody>().unwrap();
    let newton_body_comp = closest_celestial.get_component::<NewtonBody>().unwrap();
    if celestial_comp.radius >= n_body.pos.distance(newton_body_comp.pos) {
        command_buffer.add_component(*e, LandingRelation { planet_id: ccb.id });
    }
}
