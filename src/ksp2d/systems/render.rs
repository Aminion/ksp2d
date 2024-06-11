use glam::{DMat2, DVec2};
use legion::{world::SubWorld, *};
use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, render::WindowCanvas};

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

    for position in position_query.iter(world) {
        let r_mtx = DMat2::from_angle(position.a);
        let l0 = DVec2::new(-25.0, 0.0);
        let p0 = (rotate_vec_by_mtx(&r_mtx, l0) + position.p).as_i16vec2();

        let l1 = DVec2::new(0.0, -43.3013);
        let p1 = (rotate_vec_by_mtx(&r_mtx, l1) + position.p).as_i16vec2();

        let l2 = DVec2::new(25.0, 0.0);
        let p2 = (rotate_vec_by_mtx(&r_mtx, l2) + position.p).as_i16vec2();

        let _ = canvas.filled_trigon(p0.x, p0.y, p1.x, p1.y, p2.x, p2.y, COLOR);
        let _ = canvas.line(p2.x, p2.y, p0.x, p0.y, Color::RGB(255, 0, 0));
    }
    canvas.present();
}
