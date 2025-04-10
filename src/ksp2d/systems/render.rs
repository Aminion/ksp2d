use glam::{dvec2, vec2, DVec2, I16Vec2};
use legion::{world::SubWorld, *};
use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, render::Canvas, video::Window};

use crate::{
    ksp2d::{
        components::{celestial_body::CelestialBody, newton_body::NewtonBody, rocket::Rocket},
        systems::performance_info::PerformanceInfo,
    },
    CanvasResources, FontRenderer, FrameDuration, FrameTimer, SpaceScale, WindowSize,
};

const BACKGROUD_COLOR: Color = Color::BLACK;
const COLOR: Color = Color::CYAN;

#[system]
#[read_component(Rocket)]
#[read_component(CelestialBody)]
#[read_component(NewtonBody)]
pub fn render(
    #[resource] canvas_resources: &mut CanvasResources,
    #[resource] scale: &SpaceScale,
    #[resource] window_size: &WindowSize,
    #[resource] font_renderer: &mut FontRenderer<1>,
    #[resource] performance_info: &PerformanceInfo,
    #[resource] fd: &mut FrameDuration,
    #[resource] ft: &FrameTimer,
    world: &SubWorld,
) {
    canvas_resources.canvas.set_draw_color(BACKGROUD_COLOR);
    canvas_resources.canvas.clear();

    let mut position_query = <(&Rocket, &NewtonBody)>::query();

    let (rocket, body) = position_query.iter(world).last().unwrap();
    render_rocket(&mut canvas_resources.canvas, scale, rocket, body);

    let mut obj_query = <(&CelestialBody, &NewtonBody)>::query();
    for (c_body, body) in obj_query.iter(world) {
        render_celestial_body(&mut canvas_resources.canvas, scale, c_body, body)
    }

    render_ui(
        canvas_resources,
        window_size,
        font_renderer,
        rocket,
        body,
        performance_info,
    );

    fd.0 = ft.0.elapsed();
    canvas_resources.canvas.present();
}

fn render_rocket(canvas: &mut Canvas<Window>, scale: &SpaceScale, _: &Rocket, n_body: &NewtonBody) {
    let pos_s = n_body.pos * scale.0;
    
    #[inline]
    fn tranaslate(x: DVec2, a: DVec2, pos: DVec2) -> I16Vec2 {
        (x.rotate(a) + pos).as_i16vec2()
    }
    const L0: DVec2 = dvec2(-25.0, 0.0);
    let p0_i16 = tranaslate(L0, n_body.angle, pos_s);

    const L1: DVec2 = dvec2(0.0, -43.3013);
    let p1_i16 = tranaslate(L1, n_body.angle, pos_s);

    const L2: DVec2 = dvec2(25.0, 0.0);
    let p2_i16 = tranaslate(L2, n_body.angle, pos_s);

    let _ = canvas.filled_trigon(
        p0_i16.x, p0_i16.y, p1_i16.x, p1_i16.y, p2_i16.x, p2_i16.y, COLOR,
    );
    let _ = canvas.line(p2_i16.x, p2_i16.y, p0_i16.x, p0_i16.y, Color::RED);
}

fn render_celestial_body(
    canvas: &mut Canvas<Window>,
    scale: &SpaceScale,
    c_body: &CelestialBody,
    n_body: &NewtonBody,
) {
    let pos_scaled = scale.s_dvec2(n_body.pos);
    let r_scaled = scale.s_f64(c_body.radius) * 2048.0;
    let s = pos_scaled.as_i16vec2();
    let lnn = dvec2(0.0, r_scaled).rotate(n_body.angle).as_i16vec2() + s;
    let _ = canvas.circle(s.x, s.y, r_scaled as i16, c_body.color);
    let _ = canvas.line(s.x, s.y, lnn.x, lnn.y, c_body.color);
}

fn render_ui(
    canvas_resources: &mut CanvasResources,
    window_size: &WindowSize,
    font_renderer: &mut FontRenderer<1>,
    _: &Rocket,
    n_body: &NewtonBody,
    performance_info: &PerformanceInfo,
) {
    font_renderer
        .render_text(
            canvas_resources,
            &format!(
                "SPEED    {}\nA.SPEED {}\nIN FLIGHT",
                n_body.vel.length(),
                n_body.angular_vel
            ),
            vec2((window_size.0.x - 450) as f32, 0.0),
            16.0,
            Color::YELLOW,
            0,
        )
        .unwrap();

    font_renderer
        .render_text(
            canvas_resources,
            &format!(
                "FPS {} \nF. TIME {} uS",
                performance_info.fps, performance_info.frame_time
            ),
            vec2(0.0, 0.0),
            16.0,
            Color::YELLOW,
            0,
        )
        .unwrap();
}
