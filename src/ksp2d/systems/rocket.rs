use std::{collections::HashSet, time::Duration};

use glam::{DMat2, DVec2};
use legion::system;

use crate::{ksp2d::components::rocket::PlayerInput, Position};

#[inline(always)]
fn rotate_vec_by_mtx(r_mtx: &DMat2, v: DVec2) -> DVec2 {
    DVec2::new(r_mtx.row(0).dot(v), r_mtx.row(1).dot(v))
}

#[system(for_each)]
pub fn update_positions(
    pos: &mut Position,
    #[resource] dt: &Duration,
    #[resource] input: &HashSet<PlayerInput>,
) {
    const ANGLE_SPD: f64 = std::f64::consts::PI;
    const LINEAR_SPD: f64 = 64f64;

    let dt_s = dt.as_secs_f64();

    if input.contains(&PlayerInput::RotateRight) {
        pos.a += ANGLE_SPD * dt_s;
    } else if input.contains(&PlayerInput::RotateLeft) {
        pos.a -= ANGLE_SPD * dt_s;
    }

    let r_mtx = DMat2::from_angle(pos.a);

    if input.contains(&PlayerInput::MoveRight) {
        let local = DVec2::new(LINEAR_SPD * dt_s, 0f64);
        pos.p += rotate_vec_by_mtx(&r_mtx, local);
    } else if input.contains(&PlayerInput::MoveLeft) {
        let local = DVec2::new(-LINEAR_SPD * dt_s, 0f64);
        pos.p += rotate_vec_by_mtx(&r_mtx, local);
    }
    if input.contains(&PlayerInput::MoveForward) {
        let local = DVec2::new(0f64, -LINEAR_SPD * dt_s);
        pos.p += rotate_vec_by_mtx(&r_mtx, local);
    } else if input.contains(&PlayerInput::MoveBackward) {
        let local = DVec2::new(0f64, LINEAR_SPD * dt_s);
        pos.p += rotate_vec_by_mtx(&r_mtx, local);
    }
}
