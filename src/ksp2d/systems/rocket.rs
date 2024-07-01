use std::collections::HashSet;

use glam::{dvec2, DMat2, DVec2};
use legion::system;

use crate::{ksp2d::components::rocket::PlayerInput, Dt, Position};

#[inline(always)]
fn rotate_vec_by_mtx(r_mtx: &DMat2, v: DVec2) -> DVec2 {
    DVec2::new(r_mtx.row(0).dot(v), r_mtx.row(1).dot(v))
}

#[system(for_each)]
pub fn update_positions(
    pos: &mut Position,
    #[resource] dt: &Dt,
    #[resource] input: &HashSet<PlayerInput>,
) {
    const ANGLE_SPD: f64 = std::f64::consts::PI;
    const LINEAR_SPD: f64 = 64f64;

    if input.contains(&PlayerInput::RotateRight) {
        pos.a += ANGLE_SPD * dt.0;
    } else if input.contains(&PlayerInput::RotateLeft) {
        pos.a -= ANGLE_SPD * dt.0;
    }

    let r_mtx = DMat2::from_angle(pos.a);

    if input.contains(&PlayerInput::MoveRight) {
        let local = dvec2(LINEAR_SPD * dt.0, 0f64);
        pos.p += rotate_vec_by_mtx(&r_mtx, local);
    } else if input.contains(&PlayerInput::MoveLeft) {
        let local = dvec2(-LINEAR_SPD * dt.0, 0f64);
        pos.p += rotate_vec_by_mtx(&r_mtx, local);
    }
    if input.contains(&PlayerInput::MoveForward) {
        let local = dvec2(0f64, -LINEAR_SPD * dt.0);
        pos.p += rotate_vec_by_mtx(&r_mtx, local);
    } else if input.contains(&PlayerInput::MoveBackward) {
        let local = dvec2(0f64, LINEAR_SPD * dt.0);
        pos.p += rotate_vec_by_mtx(&r_mtx, local);
    }
}
