use std::collections::HashSet;

use glam::{dvec2, DVec2};
use legion::{world::SubWorld, *};

use crate::{
    ksp2d::components::{celestial_body::CelestialBody, rocket::PlayerInput},
    Dt, Rocket,
};

#[system(for_each)]
#[write_component(CelestialBody)]
pub fn update_positions(
    pos: &mut Rocket,
    #[resource] dt: &Dt,
    #[resource] input: &HashSet<PlayerInput>,
    world: &mut SubWorld,
) {
    const ANGLE_SPD: f64 = std::f64::consts::FRAC_PI_8;
    const TRUST: f64 = 34343000000000000.0;
    let mut query = world.entry_mut(pos.celestial_body).unwrap();
    let body = query.get_component_mut::<CelestialBody>().unwrap();

    if input.contains(&PlayerInput::RotateRight) {
        body.a_vel += ANGLE_SPD * dt.0;
    } else if input.contains(&PlayerInput::RotateLeft) {
        body.a_vel -= ANGLE_SPD * dt.0;
    }

    body.update_a(dt);

    let mut d_f_local = DVec2::ZERO;

    if input.contains(&PlayerInput::MoveRight) {
        d_f_local.x = TRUST;
    } else if input.contains(&PlayerInput::MoveLeft) {
        d_f_local.x = -TRUST;
    }
    if input.contains(&PlayerInput::MoveForward) {
        d_f_local.y = -TRUST;
    } else if input.contains(&PlayerInput::MoveBackward) {
        d_f_local.y = TRUST;
    }

    let d_f_global = DVec2::from_angle(body.a).rotate(d_f_local);
    let d_a = d_f_global / body.mass;
    let d_v = d_a * dt.0;
    let d_p = d_v * dt.0;

    body.vel += d_v;
    body.pos += d_p;
}
