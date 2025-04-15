use std::collections::HashSet;

use glam::DVec2;
use legion::*;

use crate::{
    ksp2d::components::{newton_body::NewtonBody, rocket::PlayerInput},
    Dt, Rocket,
};

#[system(for_each)]
#[read_component(Rocket)]
#[write_component(NewtonBody)]
pub fn update_positions(
    _rocket: &Rocket,
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

    let d_f_global = body.angle.rotate(d_f_local);
    let d_a = d_f_global / body.mass;
    let d_v = d_a * dt.0;
    let d_p = d_v * dt.0;

    body.vel += d_v;
    body.pos += d_p;
}
