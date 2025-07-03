use glam::{dvec2, vec2, DMat3, DVec2, DVec3, I16Vec2};
use legion::{world::SubWorld, *};
use sdl2::{
    gfx::primitives::DrawRenderer,
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureAccess},
    video::Window,
};
use std::{cmp::Ordering, fmt::format};
use uom::si::{f64::*, velocity::meter_per_second};

use crate::{
    ksp2d::{
        components::{
            celestial_body::CelestialBody,
            closest_celestial_body::ClosestCelestialBody,
            flight_info::{self, FlightInfo},
            newton_body::NewtonBody,
            rocket::Rocket,
        },
        systems::performance_info::PerformanceInfo,
    },
    CameraMode, CanvasResources, FontRenderer, FrameDuration, FrameTimer, WindowSize, SPACE_SIZE,
};

const BACKGROUD_COLOR: Color = Color::BLACK;
const COLOR: Color = Color::CYAN;

#[system]
#[read_component(Rocket)]
#[read_component(CelestialBody)]
#[read_component(NewtonBody)]
#[read_component(ClosestCelestialBody)]
#[write_component(FlightInfo)]

pub fn render(
    #[resource] canvas_resources: &mut CanvasResources,
    #[resource] font_renderer: &mut FontRenderer<1>,
    #[resource] fd: &mut FrameDuration,
    // #[resource] camera_mode: &CameraMode,
    #[resource] window_size: &WindowSize,
    #[resource] performance_info: &PerformanceInfo,
    #[resource] ft: &FrameTimer,
    world: &SubWorld,
) {
    canvas_resources.canvas.set_draw_color(BACKGROUD_COLOR);
    canvas_resources.canvas.clear();

    let camera_mode = CameraMode::Default;
    let (tex, padded) = get_space_rect(window_size.0.x, window_size.0.y);
    let scale = tex.width() as f64 / SPACE_SIZE;
    let mut position_query = <(&Rocket, &NewtonBody, &ClosestCelestialBody, &FlightInfo)>::query();
    let (rocket, body, ccb, flight_info) = position_query.iter(world).last().unwrap();
    let closest_celestial = world.entry_ref(ccb.0).unwrap();
    let newton_body_comp = closest_celestial.get_component::<NewtonBody>().unwrap();
    let srt_mtx = match camera_mode {
        CameraMode::Default => {
            let scale_mtx = DMat3::from_diagonal(DVec3::new(scale, scale, 1.0));
            let rotate_mtx = DMat3::IDENTITY;
            let transform_mtx = DMat3::IDENTITY;
            let mtx = scale_mtx * rotate_mtx * transform_mtx;
            mtx
        }
        CameraMode::Landing => {
            let closest_celestial = world.entry_ref(ccb.0).unwrap();
            let newton_body_comp = closest_celestial.get_component::<NewtonBody>().unwrap();
            let dst = tex.width() as f64 / body.pos.distance(newton_body_comp.pos);
            let scale_mtx = DMat3::from_scale(DVec2::splat(dst * 0.5));
            let mid = scale_mtx.transform_point2(body.pos.midpoint(newton_body_comp.pos));
            let mid2 = body.pos.midpoint(newton_body_comp.pos);
            let rebase2 = DMat3::from_translation(-mid);
            let rebase3 = DMat3::from_translation(DVec2::splat(tex.width() as f64 / 2.0));
            let mtx = rebase3 * rebase2 * scale_mtx;
            mtx
        }
    };

    let mut obj_query = <(&CelestialBody, &NewtonBody)>::query();

    {
        let mut intermediate_texture = canvas_resources
            .texture_creator
            .create_texture(
                None, // Use default pixel format, or specify one like PixelFormatEnum::RGBA8888
                TextureAccess::Target,
                tex.width(),
                tex.height(),
            )
            .unwrap();
        let _ = canvas_resources
            .canvas
            .with_texture_canvas(&mut intermediate_texture, |c| {
                render_rocket(c, &srt_mtx, rocket, body, newton_body_comp.pos);
                for (c_body, body) in obj_query.iter(world) {
                    render_celestial_body(c, &srt_mtx, scale, c_body, body)
                }
            });

        let _ = canvas_resources
            .canvas
            .copy(&intermediate_texture, None, Some(padded));
    }

    render_ui(
        canvas_resources,
        window_size,
        font_renderer,
        flight_info,
        performance_info,
    );

    fd.0 = ft.0.elapsed();
    canvas_resources.canvas.present();
}

fn render_rocket(
    canvas: &mut Canvas<Window>,
    srt_mtx: &DMat3,
    _: &Rocket,
    n_body: &NewtonBody,
    p: DVec2,
) {
    #[inline]
    fn tranaslate(x: &DVec2, a: DVec2, pos: DVec2) -> I16Vec2 {
        (x.rotate(a) + pos).as_i16vec2()
    }

    let n_body_applied = srt_mtx.transform_point2(n_body.pos);
    let poits: Vec<_> = [dvec2(-25.0, 0.0), dvec2(0.0, -43.3013), dvec2(25.0, 0.0)]
        .iter()
        .map(|p| tranaslate(p, n_body.angle, n_body_applied))
        .collect();

    let _ = canvas.filled_trigon(
        poits[0].x, poits[0].y, poits[1].x, poits[1].y, poits[2].x, poits[2].y, COLOR,
    );
    let _ = canvas.line(poits[2].x, poits[2].y, poits[0].x, poits[0].y, Color::RED);
    let pp = srt_mtx.transform_point2(p).as_i16vec2();
    let rr = n_body_applied.as_i16vec2();
    let _ = canvas.line(rr.x, rr.y, pp.x, pp.y, Color::MAGENTA);
}

fn render_celestial_body(
    canvas: &mut Canvas<Window>,
    srt_mtx: &DMat3,
    scale: f64,
    c_body: &CelestialBody,
    n_body: &NewtonBody,
) {
    let n_body_applied = srt_mtx.transform_point2(n_body.pos).as_i16vec2();
    let radius_applied = c_body.radius * scale;
    let pointer = DVec2::ZERO
        .with_y(radius_applied)
        .rotate(n_body.angle)
        .as_i16vec2()
        + n_body_applied;
    let _ = canvas.circle(
        n_body_applied.x,
        n_body_applied.y,
        radius_applied as i16,
        c_body.color,
    );
    let _ = canvas.line(
        n_body_applied.x,
        n_body_applied.y,
        pointer.x,
        pointer.y,
        c_body.color,
    );
}

fn render_ui(
    canvas_resources: &mut CanvasResources,
    window_size: &WindowSize,
    font_renderer: &mut FontRenderer<1>,
    flight_info: &FlightInfo,
    performance_info: &PerformanceInfo,
) {
    font_renderer
        .render_text(
            canvas_resources,
            &format!(
                "SPEED    {:.1}\nA.SPEED {}\nIN FLIGHT",
                flight_info
                    .delta
                    .into_format_args(meter_per_second, uom::fmt::DisplayStyle::Abbreviation),
                0.0
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

fn get_space_rect(x: i32, y: i32) -> (Rect, Rect) {
    #[inline]
    fn padding(a: i32, b: i32) -> i32 {
        (((a - b) as f64) * 0.5) as i32
    }
    match x.cmp(&y) {
        Ordering::Greater => {
            let r = y as u32;
            let rr = Rect::new(0, 0, r, r);
            (rr, rr.right_shifted(padding(x, y)))
        }
        Ordering::Less => {
            let r = x as u32;
            let rr = Rect::new(0, 0, r, r);
            (rr, rr.bottom_shifted(padding(y, x)))
        }
        _ => {
            let r = x as u32;
            let rr = Rect::new(0, 0, r, r);
            (rr, rr)
        }
    }
}
