use glam::{dvec2, DMat2, DVec2};
use legion::{world::SubWorld, *};
use log::info;
use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, render::WindowCanvas};

use crate::ksp2d::collision::*;
use crate::Position;

#[inline(always)]
fn rotate_vec_by_mtx(r_mtx: &DMat2, v: DVec2) -> DVec2 {
    DVec2::new(r_mtx.row(0).dot(v), r_mtx.row(1).dot(v))
}

const COLOR: Color = Color::RGB(0, 255, 255);

#[system]
#[read_component(Position)]
pub fn render(#[resource] canvas: &mut WindowCanvas, world: &SubWorld) {
    let mut position_query = <&Position>::query();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    const C: DVec2 = dvec2(500f64, 600f64);
    const C_R: f64 = 100f64;

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
        let _ = canvas.circle(500, 600, C_R as i16, Color::RGB(255, 0, 0));

        let (aa, bb) = triangle_aabb(l0_t, l1_t, l2_t);
        let aa_i16 = aa.as_i16vec2();
        let bb_i16 = bb.as_i16vec2();
        let _ = canvas.rectangle(
            aa_i16.x,
            aa_i16.y,
            bb_i16.x,
            bb_i16.y,
            Color::RGB(255, 0, 0),
        );

        let (c1,c2) = circle_aabb(C,C_R);
        let c1_i16 = c1.as_i16vec2();
        let c2_i16 = c2.as_i16vec2();
        let _ = canvas.rectangle(
            c1_i16.x,
            c1_i16.y,
            c2_i16.x,
            c2_i16.y,
            Color::RGB(255, 0, 0),
        );

        let one = rotate_vec_by_mtx(&r_mtx, L2) + position.p;
        let two = rotate_vec_by_mtx(&r_mtx, L0) + position.p;
        info!("AABB {}", is_aabb_intersected(aa, bb, c1, c2));
        info!("INTERSECTION {}", is_segment_intersects_circle(one, two, C, C_R));
    }
    canvas.present();
}
