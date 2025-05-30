use std::collections::HashMap;

use glam::{dvec2, DVec2};
use legion::{world::SubWorld, *};

use crate::ksp2d::components::{
    celestial_body::CelestialBody, landing::LandingRelation, newton_body::NewtonBody,
};

#[system]
#[read_component(CelestialBody)]
#[read_component(LandingRelation)]
#[write_component(NewtonBody)]
pub fn planet_resting(world: &mut SubWorld) {
    let mut query = <(Entity, &LandingRelation)>::query();

    let rr: HashMap<_, _> = query
        .iter(world)
        .map(|(rocket_id, planet)| {
            let planet_entity = world.entry_ref(planet.planet_id).unwrap();
            let planet_n_body = planet_entity.get_component::<NewtonBody>().unwrap();
            let planet_celestial_body = planet_entity.get_component::<CelestialBody>().unwrap();
            let rocket_pos_update = planet_n_body.pos
                + dvec2(0.0, planet_celestial_body.radius).rotate(planet_n_body.angle);
            (*rocket_id, (-planet_n_body.angle, rocket_pos_update))
        })
        .collect();

    let mut query = <(Entity, &mut NewtonBody, &LandingRelation)>::query();

    for (rocket_id, rocket_n_body, _) in query.iter_mut(world) {
        let (angle_updated, pos_updated) = rr.get(rocket_id).unwrap();
        rocket_n_body.angle = *angle_updated;
        rocket_n_body.pos = *pos_updated;
    }
}

fn velocity_direct_2d(radius: f64, signed_omega: f64, unit_angle: DVec2) -> DVec2 {
    let vx = signed_omega * radius * unit_angle.y;
    let vy = -signed_omega * radius * unit_angle.x;
    DVec2::new(vx, vy)
}
