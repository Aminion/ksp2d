pub mod fonts;
pub mod ksp2d;

extern crate crossbeam;
extern crate glam;
extern crate legion;
extern crate rand;
extern crate sdl2;

use fonts::{load_fonts, FontRenderer};
use glam::{dvec2, ivec2, IVec2};
use ksp2d::components::celestial_body::{CelestialBody, CelestialBodyType};
use ksp2d::components::newton_body::NewtonBody;
use ksp2d::components::rocket::Rocket;
use ksp2d::systems::newton_body::celestial_body_system;
use ksp2d::systems::performance_info::{update_info_system, PerformanceInfo};
use ksp2d::systems::render::render_system;
use ksp2d::systems::rocket::update_positions_system;
use sdl2::event::WindowEvent;
use sdl2::mixer::InitFlag;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, TextureCreator, WindowCanvas};
use sdl2::video::{Window, WindowContext};
use sdl2::EventPump;
use sdl2::{event::Event, keyboard::Scancode};
use std::collections::HashSet;
use std::time::{Duration, Instant};

use legion::*;

use crate::ksp2d::components::rocket::PlayerInput;

pub struct Dt(f64);
pub struct FrameTimer(Instant);
pub struct FrameDuration(Duration);
pub struct SpaceScale(f64);
pub struct WindowSize(IVec2);

const SPACE_SIZE: f64 = 4.5029e+12 / 8.0;

pub struct CanvasResources {
    pub canvas: Canvas<Window>,
    pub texture_creator: TextureCreator<WindowContext>,
}

fn initialize() -> Result<(WindowCanvas, EventPump), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _audio = sdl_context.audio()?;

    let _mixer_context =
        sdl2::mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG)?;
    sdl2::mixer::allocate_channels(20);

    let window = video_subsystem
        .window("KSP 2D", 1280, 720)
        .resizable()
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .expect("could not make a canvas");
    let event_pump = sdl_context.event_pump().unwrap();

    Ok((canvas, event_pump))
}

fn initial_resources(canvas: Canvas<Window>) -> Resources {
    let mut resources = Resources::default();
    let texture_creator = canvas.texture_creator();

    let canvas_resources = CanvasResources {
        canvas,
        texture_creator,
    };
    let fonts = load_fonts();
    let font_renderer = FontRenderer::new(fonts).unwrap();
    let perf_info = PerformanceInfo {
        fps: 0,
        frame_time: 0,
        update_timer: Instant::now(),
    };
    resources.insert(canvas_resources);
    resources.insert(perf_info);
    resources.insert(font_renderer);
    resources.insert(HashSet::<PlayerInput>::new());
    resources.insert(SpaceScale(1280.0 / SPACE_SIZE));
    resources.insert(WindowSize(ivec2(1280, 720)));
    resources.insert(FrameTimer(Instant::now()));
    resources.insert(FrameDuration(Duration::ZERO));
    resources.insert(Dt(0.0));
    resources
}

fn initial_world() -> World {
    let mut world = World::default();
    let rocket_body = NewtonBody {
        angle: 0.0,
        angular_vel: 0.0,
        mass: 2965000.0,
        pos: dvec2(149597870700.0 / 4.0, 149597870700.0),
        prev_pos: dvec2(149597870700.0 / 4.0, 149597870700.0),
        vel: dvec2(-0.0, 0.0),
        acc: dvec2(0.0, 0.0),
    };

    world.push((Rocket {}, rocket_body));

    let pl1 = NewtonBody {
        angle: 0.0,
        angular_vel: 0.0,
        mass: 5.9722e24,
        pos: dvec2(149597870700.0, 149597870700.0 * 0.5),
        prev_pos: dvec2(149597870700.0, 149597870700.0 * 0.5),
        vel: dvec2(0.0, 111111110.0),
        acc: dvec2(0.0, 0.0),
    };

    let pl2 = NewtonBody {
        angle: 0.0,
        angular_vel: 0.0,
        mass: 1.9884e30,
        pos: dvec2(149597870700.0 * 2.0, 149597870700.0),
        prev_pos: dvec2(149597870700.0 * 2.0, 149597870700.0),
        vel: dvec2(0.0, 0.0),
        acc: dvec2(0.0, 0.0),
    };

    world.extend(vec![
        (
            CelestialBody {
                b_type: CelestialBodyType::Planet,
                radius: 6051000.0,
                color: Color::GREEN,
            },
            pl1,
        ),
        (
            CelestialBody {
                b_type: CelestialBodyType::Planet,
                radius: 6378000.0,
                color: Color::RED,
            },
            pl2,
        ),
    ]);
    world
}

pub fn main() {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let (canvas, mut event_pump) = initialize().unwrap();
    let mut resources = initial_resources(canvas);
    let mut world = initial_world();
    let mut schedule = Schedule::builder()
        .add_system(update_positions_system())
        .add_system(celestial_body_system())
        .add_system(update_info_system())
        .flush()
        .add_thread_local(render_system())
        .build();

    'running: loop {
        {
            let mut frame_timer = resources.get_mut::<FrameTimer>().unwrap();
            let mut frame_duration = resources.get_mut::<Dt>().unwrap();
            frame_duration.0 = frame_timer.0.elapsed().as_secs_f64();
            frame_timer.0 = Instant::now();
            let mut pinput = resources.get_mut::<HashSet<PlayerInput>>().unwrap();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        scancode: Some(Scancode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown {
                        scancode: Some(code),
                        ..
                    } => {
                        let insertion = match code {
                            Scancode::A => Some(PlayerInput::MoveLeft),
                            Scancode::D => Some(PlayerInput::MoveRight),
                            Scancode::W => Some(PlayerInput::MoveForward),
                            Scancode::S => Some(PlayerInput::MoveBackward),
                            Scancode::Q => Some(PlayerInput::RotateLeft),
                            Scancode::E => Some(PlayerInput::RotateRight),
                            _ => None,
                        };
                        if let Some(player_input) = insertion {
                            pinput.insert(player_input);
                        }
                    }
                    Event::KeyUp {
                        scancode: Some(code),
                        ..
                    } => {
                        let insertion = match code {
                            Scancode::A => Some(&PlayerInput::MoveLeft),
                            Scancode::D => Some(&PlayerInput::MoveRight),
                            Scancode::W => Some(&PlayerInput::MoveForward),
                            Scancode::S => Some(&PlayerInput::MoveBackward),
                            Scancode::Q => Some(&PlayerInput::RotateLeft),
                            Scancode::E => Some(&PlayerInput::RotateRight),
                            _ => None,
                        };
                        if let Some(player_input) = insertion {
                            pinput.remove(player_input);
                        }
                    }
                    Event::Window {
                        win_event: WindowEvent::SizeChanged(x, y),
                        ..
                    } => {
                        let mut r = resources.get_mut::<SpaceScale>().unwrap();
                        let mut z = resources.get_mut::<WindowSize>().unwrap();
                        r.0 = x.max(y) as f64 / SPACE_SIZE;
                        z.0 = ivec2(x, y);
                    }
                    _ => {}
                }
            }
        }

        schedule.execute(&mut world, &mut resources);
    }
}
