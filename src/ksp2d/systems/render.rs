use glam::{dvec2, DMat2, DVec2};
use legion::{world::SubWorld, *};
use log::info;
use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, render::WindowCanvas};

use crate::ksp2d::collision::*;
use crate::ksp2d::components::celestial_body::Obj;
use crate::Position;

#[inline(always)]
fn rotate_vec_by_mtx(r_mtx: &DMat2, v: DVec2) -> DVec2 {
    DVec2::new(r_mtx.row(0).dot(v), r_mtx.row(1).dot(v))
}

const COLOR: Color = Color::RGB(0, 255, 255);

#[system]
#[read_component(Position)]
#[read_component(Obj)]
pub fn render(#[resource] canvas: &mut WindowCanvas, world: &SubWorld) {
    let mut position_query = <&Position>::query();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    //canvas.clear();

    for position in position_query.iter(world) {
        let r_mtx = DMat2::from_angle(position.a);
        const L0: DVec2 = dvec2(-25.0, 0.0);
        let l0_t = rotate_vec_by_mtx(&r_mtx, L0) + position.p;
        let p0_i16 = l0_t.as_i16vec2();

        const L1: DVec2 = dvec2(0.0, -43.3013);
        let l1_t = rotate_vec_by_mtx(&r_mtx, L1) + position.p;
        let p1_i16 = l1_t.as_i16vec2();

        const L2: DVec2 = dvec2(25.0, 0.0);
        let l2_t = rotate_vec_by_mtx(&r_mtx, L2) + position.p;
        let p2_i16 = l2_t.as_i16vec2();

        let _ = canvas.filled_trigon(
            p0_i16.x, p0_i16.y, p1_i16.x, p1_i16.y, p2_i16.x, p2_i16.y, COLOR,
        );
        let _ = canvas.line(
            p2_i16.x,
            p2_i16.y,
            p0_i16.x,
            p0_i16.y,
            Color::RGB(255, 0, 0),
        );
    }

    let mut obj_query = <&Obj>::query();
    for o in obj_query.iter(world) {
        let s = (o.pos * 1.7112543e-9).as_i16vec2();
        let _ = canvas.pixel(100 + s.x, 100 + s.y, Color::RGB(0, 255, 0));
    }
    canvas.present();
}
