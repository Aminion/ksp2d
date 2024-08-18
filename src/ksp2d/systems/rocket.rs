use std::collections::HashSet;

use glam::{dvec2, DMat2, DVec2};
use legion::{world::SubWorld, *};
use log::info;

use crate::{
    ksp2d::components::{
        celestial_body::{self, CelestialBody},
        rocket::PlayerInput,
    },
    Dt, Rocket,
};

#[inline(always)]
fn rotate_vec_by_mtx(r_mtx: &DMat2, v: DVec2) -> DVec2 {
    DVec2::new(r_mtx.row(0).dot(v), r_mtx.row(1).dot(v))
}

#[system(for_each)]
#[write_component(CelestialBody)]
pub fn update_positions(
    pos: &mut Rocket,
    #[resource] dt: &Dt,
    #[resource] input: &HashSet<PlayerInput>,
    world: &mut SubWorld,
) {
    const ANGLE_SPD: f64 = std::f64::consts::PI;
    const LINEAR_SPD: f64 = 64f64;
    const TRUST: f64 = 343430000000000.0 * 1000000000000000000000000000000000000000000000000000000000000.0;
    let mut query = world.entry_mut(pos.celestial_body).unwrap();
    let body = query.get_component_mut::<CelestialBody>().unwrap();

    if input.contains(&PlayerInput::RotateRight) {
        pos.a += ANGLE_SPD * dt.0;
    } else if input.contains(&PlayerInput::RotateLeft) {
        pos.a -= ANGLE_SPD * dt.0;
    }

    let r_mtx = DMat2::from_angle(pos.a);

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

    let d_f_global = rotate_vec_by_mtx(&r_mtx, d_f_local);
    info!("{d_f_local} / {d_f_global} / {test}");
    let d_a = dvec2(
        if d_f_global.x.is_normal() {
            body.mass / d_f_global.x
        } else {
            0.0
        },
        if d_f_global.y.is_normal() {
            body.mass / d_f_global.y
        } else {
            0.0
        },
    );
    let d_v = d_a * dt.0;
    let d_p = d_v * dt.0;
    info!("{d_a} / {d_v} / {d_p}");
    body.acc += d_a;
    body.vel += d_v;
    body.pos += d_p;
}
