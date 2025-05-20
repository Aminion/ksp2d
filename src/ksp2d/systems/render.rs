use std::cmp::Ordering;

use glam::{dvec2, vec2, DMat3, DVec2, DVec3, I16Vec2, Vec3Swizzles};
use legion::{world::SubWorld, *};
use sdl2::{
    gfx::primitives::DrawRenderer,
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureAccess},
    video::Window,
};

use crate::{
    ksp2d::{
        components::{celestial_body::CelestialBody, newton_body::NewtonBody, rocket::Rocket},
        systems::performance_info::PerformanceInfo,
    },
    CameraMode, CanvasResources, FontRenderer, FrameDuration, FrameTimer, SpacePadding, SpaceScale,
    WindowSize, SPACE_SIZE,
};

const BACKGROUD_COLOR: Color = Color::BLACK;
const COLOR: Color = Color::CYAN;

#[system]
#[read_component(Rocket)]
#[read_component(CelestialBody)]
#[read_component(NewtonBody)]
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

    let (scale_, padding_) = get_scaling(window_size.0.x, window_size.0.y);
    let scale = SpaceScale(scale_);
    let padding = &SpacePadding(padding_);

    let camera_mode = CameraMode::Default;
    let t_mtx = match camera_mode {
        CameraMode::Default => {
            let scale_mtx = DMat3::from_diagonal(DVec3::new(scale_, scale_, 1.0));
            let rotate_mtx = DMat3::IDENTITY;
            let transform_mtx = DMat3::IDENTITY;
            let mtx = scale_mtx * rotate_mtx * transform_mtx;
            mtx
        }
        CameraMode::Landing => DMat3::ZERO,
    };
    let mut position_query = <(&Rocket, &NewtonBody)>::query();

    let (rocket, body) = position_query.iter(world).last().unwrap();

    let (tex, padded) = get_space_rect(window_size.0.x, window_size.0.y);
    println!("{:?}|{:?}", tex, padded);
    {
        let mut intermediate_texture = canvas_resources
            .texture_creator
            .create_texture(
                None, // Use default pixel format, or specify one like PixelFormatEnum::RGBA8888
                TextureAccess::Target,
                tex.width(),  // Width
                tex.height(), // Height
            )
            .unwrap();

        render_rocket_(
            &mut canvas_resources.canvas,
            &mut intermediate_texture,
            &t_mtx,
            rocket,
            body,
        );
        canvas_resources
            .canvas
            .copy(&intermediate_texture, None, Some(padded));
    }

    let mut obj_query = <(&CelestialBody, &NewtonBody)>::query();
    for (c_body, body) in obj_query.iter(world) {
        //   render_celestial_body(&mut canvas_resources.canvas, &scale, padding, c_body, body)
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

fn render_rocket_(
    canvas: &mut Canvas<Window>,
    texture: &mut Texture,
    r_mtx: &DMat3,
    _: &Rocket,
    n_body: &NewtonBody,
) {
    #[inline]
    fn tranaslate(x: DVec2, a: DVec2, pos: DVec2) -> I16Vec2 {
        (x.rotate(a) + pos).as_i16vec2()
    }
    let _ = canvas.with_texture_canvas(texture, |c| {
        const L0: DVec2 = dvec2(-25.0, 0.0);
        const L1: DVec2 = dvec2(0.0, -43.3013);
        const L2: DVec2 = dvec2(25.0, 0.0);
        let original_vector_homogeneous = DVec3::ONE.with_xy(n_body.pos);
        let pos_s = r_mtx.mul_vec3(original_vector_homogeneous);
        let r = pos_s.xy();

        let p0_i16 = tranaslate(L0, n_body.angle, r);
        let p1_i16 = tranaslate(L1, n_body.angle, r);
        let p2_i16 = tranaslate(L2, n_body.angle, r);
        let _ = c.filled_trigon(
            p0_i16.x, p0_i16.y, p1_i16.x, p1_i16.y, p2_i16.x, p2_i16.y, COLOR,
        );
        let _ = c.line(p2_i16.x, p2_i16.y, p0_i16.x, p0_i16.y, Color::RED);
    });
}

fn render_rocket(
    canvas: &mut Canvas<Window>,
    scale: &SpaceScale,
    padding: &SpacePadding,
    _: &Rocket,
    n_body: &NewtonBody,
) {
    let pos_s = n_body.pos * scale.0;

    #[inline]
    fn tranaslate(padding: &SpacePadding, x: DVec2, a: DVec2, pos: DVec2) -> I16Vec2 {
        (x.rotate(a) + pos).as_i16vec2() + padding.0
    }
    const L0: DVec2 = dvec2(-25.0, 0.0);
    let p0_i16 = tranaslate(padding, L0, n_body.angle, pos_s);

    const L1: DVec2 = dvec2(0.0, -43.3013);
    let p1_i16 = tranaslate(padding, L1, n_body.angle, pos_s);

    const L2: DVec2 = dvec2(25.0, 0.0);
    let p2_i16 = tranaslate(padding, L2, n_body.angle, pos_s);

    let _ = canvas.filled_trigon(
        p0_i16.x, p0_i16.y, p1_i16.x, p1_i16.y, p2_i16.x, p2_i16.y, COLOR,
    );
    let _ = canvas.line(p2_i16.x, p2_i16.y, p0_i16.x, p0_i16.y, Color::RED);
}

fn render_celestial_body(
    canvas: &mut Canvas<Window>,
    scale: &SpaceScale,
    padding: &SpacePadding,
    c_body: &CelestialBody,
    n_body: &NewtonBody,
) {
    let pos_scaled = scale.s_dvec2(n_body.pos);
    let r_scaled = scale.s_f64(c_body.radius);
    let s = pos_scaled.as_i16vec2() + padding.0;
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

fn get_space_rect(x: i32, y: i32) -> (Rect, Rect) {
    #[inline]
    fn padding(a: i32, b: i32) -> i32 {
        (((a - b) as f64) * 0.5) as i32
    }
    match x.cmp(&y) {
        Ordering::Greater => {
            let r = x as u32;
            let rr = Rect::new(0, 0, r, r);
            (rr, rr.right_shifted(padding(x, y)))
        }
        Ordering::Less => {
            let r = y as u32;
            let rr = Rect::new(0, 0, r, r);
            (rr, rr.bottom_shifted(padding(y, x)))
        }
        _ => {
            let r = y as u32;
            let rr = Rect::new(0, 0, r, r);
            (rr, rr)
        }
    }
}
fn get_scaling(x: i32, y: i32) -> (f64, I16Vec2) {
    let x_f = x as f64;
    let y_f = y as f64;
    #[inline]
    fn padding(a: f64, b: f64) -> i16 {
        ((a - b) * 0.5) as i16
    }
    match x.cmp(&y) {
        Ordering::Greater => (y_f / SPACE_SIZE, {
            let x = padding(x_f, y_f);
            I16Vec2 { x, y: 0 }
        }),
        Ordering::Less => (x_f / SPACE_SIZE, {
            let y = padding(y_f, x_f);
            I16Vec2 { x: 0, y }
        }),
        _ => (x_f / SPACE_SIZE, I16Vec2::ZERO),
    }
}
