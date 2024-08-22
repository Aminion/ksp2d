use glam::{dvec2, DVec2};
use legion::{world::SubWorld, *};
use log::info;
use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, render::WindowCanvas};

use crate::{
    ksp2d::components::{celestial_body::CelestialBody, newton_body::NewtonBody, rocket::Rocket},
    SpaceScale,
};

const COLOR: Color = Color::RGB(0, 255, 255);

#[system]
#[read_component(Rocket)]
#[read_component(CelestialBody)]
#[read_component(NewtonBody)]
pub fn render(
    #[resource] canvas: &mut WindowCanvas,
    #[resource] scale: &SpaceScale,
    world: &SubWorld,
) { 
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let mut position_query = <(&Rocket, &NewtonBody)>::query();

    for (_, body) in position_query.iter(world) {
        info!("ROCKET");
        let pos_s = body.pos * scale.0;
        let r_vec = DVec2::from_angle(body.a);
        const L0: DVec2 = dvec2(-25.0, 0.0);
        let l0_t = r_vec.rotate(L0) + pos_s;
        let p0_i16 = l0_t.as_i16vec2();

        const L1: DVec2 = dvec2(0.0, -43.3013);
        let l1_t = r_vec.rotate(L1) + pos_s;
        let p1_i16 = l1_t.as_i16vec2();

        const L2: DVec2 = dvec2(25.0, 0.0);
        let l2_t = r_vec.rotate(L2) + pos_s;
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

    let mut obj_query = <(&CelestialBody, &NewtonBody)>::query();

    for (_, body) in obj_query.iter(world) {
        let s = (body.pos * scale.0).as_i16vec2();
        let _ = canvas.circle(s.x, s.y, 5, Color::RGB(0, 255, 0));
    }
    canvas.present();
}
