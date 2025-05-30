use core::f64;
use std::collections::HashSet;

use legion::*;

use crate::{
    ksp2d::components::{newton_body::NewtonBody, rocket::PlayerInput},
    Dt, Rocket,
};

#[system(for_each)]
#[write_component(NewtonBody)]
#[read_component(Rocket)]
pub fn update_positions(
    rocket: &mut Rocket,
    body: &mut NewtonBody,
    #[resource] dt: &Dt,
    #[resource] input: &HashSet<PlayerInput>,
) {
    const ANGLE_SPD: f64 = std::f64::consts::FRAC_PI_8;
    const TRUST: f64 = 343430000000000.0;

    if input.contains(&PlayerInput::RotateRight) {
        body.angular_vel += ANGLE_SPD * dt.0;
    } else if input.contains(&PlayerInput::RotateLeft) {
        body.angular_vel -= ANGLE_SPD * dt.0;
    }

    body.update_a(dt);

    if input.contains(&PlayerInput::MoveRight) {
        rocket.engine_left.full();
        rocket.engine_right.disable();
    } else {
        rocket.engine_left.disable();
    }

    if input.contains(&PlayerInput::MoveLeft) {
        rocket.engine_right.full();
        rocket.engine_left.disable();
    } else {
        rocket.engine_right.disable();
    }

    if input.contains(&PlayerInput::MoveForward) {
        rocket.engine_averse.change_throttle(dt.0);
    }

    if input.contains(&PlayerInput::MoveBackward) {
        if rocket.engine_averse.throttle != 0.0 {
            rocket.engine_averse.change_throttle(-dt.0);
        } else {
            rocket.engine_reverse.full();
        }
    } else {
        rocket.engine_reverse.disable();
    }

    let d_f_local = rocket.trust();
    let d_f_global = (body.angle).rotate(d_f_local);
    let d_a = d_f_global / body.mass;
    let d_v = d_a * dt.0;
    let d_p = d_v * dt.0;

    body.vel += d_v;
    body.pos += d_p;
}
