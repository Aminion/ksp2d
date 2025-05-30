pub mod fonts;
pub mod ksp2d;
pub mod system_generation;

extern crate crossbeam;
extern crate glam;
extern crate legion;
extern crate rand;
extern crate sdl2;

use fonts::{load_fonts, FontRenderer};
use glam::{dvec2, ivec2, DVec2, I16Vec2, IVec2};
use ksp2d::components::closest_celestial_body::ClosestCelestialBody;
use ksp2d::components::newton_body::NewtonBody;
use ksp2d::components::rocket::Rocket;
use ksp2d::systems::closest_celestial::closest_celestial_system;
use ksp2d::systems::landing::landing_system;
use ksp2d::systems::newton_body::celestial_body_system;
use ksp2d::systems::performance_info::{update_info_system, PerformanceInfo};
use ksp2d::systems::planet_resting::planet_resting_system;
use ksp2d::systems::render::render_system;
use ksp2d::systems::rocket::update_positions_system;
use sdl2::event::WindowEvent;
use sdl2::mixer::InitFlag;
use sdl2::render::{Canvas, TextureCreator, WindowCanvas};
use sdl2::video::{Window, WindowContext};
use sdl2::EventPump;
use sdl2::{event::Event, keyboard::Scancode};
use std::collections::HashSet;
use std::time::{Duration, Instant};
use system_generation::get_system;
use systems::CommandBuffer;

use std::cmp::Ordering;

use legion::*;

use crate::ksp2d::components::rocket::PlayerInput;

pub struct Dt(f64);
pub struct FrameTimer(Instant);
pub struct FrameDuration(Duration);
pub struct SpaceScale(f64);

pub struct SpacePadding(I16Vec2);

impl SpaceScale {
    fn s_f64(&self, x: f64) -> f64 {
        x * self.0
    }

    fn s_dvec2(&self, x: DVec2) -> DVec2 {
        x * self.0
    }
}

pub struct WindowSize(IVec2);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CameraMode {
    Default,
    Landing,
}

const SPACE_SIZE: f64 = 1e10;
const INITIAL_WINDOW_WIDTH: u32 = 1280;
const INITIAL_WINDOW_HEIGHT: u32 = 720;

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
        .window("KSP 2D", INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT)
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

fn initial_resources(canvas: Canvas<Window>, world: &World) -> Resources {
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

    let initial_size = ivec2(INITIAL_WINDOW_WIDTH as i32, INITIAL_WINDOW_HEIGHT as i32);

    resources.insert(WindowSize(initial_size));
    resources.insert(FrameTimer(Instant::now()));
    resources.insert(FrameDuration(Duration::ZERO));
    resources.insert(Dt(0.0));
    resources.insert(CameraMode::Default);

    let command_buffer = CommandBuffer::new(world);
    resources.insert(command_buffer);
    resources
}

fn initial_world() -> World {
    let mut world = World::default();
    let rocket_body = NewtonBody {
        angle: DVec2::Y,
        angular_vel: 0.0,
        mass: 2965000.0,
        pos: dvec2(SPACE_SIZE / 8.0, SPACE_SIZE / 8.0),
        vel: DVec2::ZERO,
        acc: DVec2::ZERO,
    };

    let sys = get_system(SPACE_SIZE * 0.5);
    let first_celestial = world.extend(sys);
    let first_celestial_enity = *first_celestial.first().unwrap();
    world.push((
        Rocket::new(),
        rocket_body,
        ClosestCelestialBody(first_celestial_enity),
    ));
    world
}

pub fn main() {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let (canvas, mut event_pump) = initialize().unwrap();
    let mut world = initial_world();
    let mut resources = initial_resources(canvas, &world);
    let mut schedule = Schedule::builder()
        .add_system(update_positions_system())
        .add_system(celestial_body_system())
        .add_system(update_info_system())
        .add_system(landing_system())
        .add_system(planet_resting_system())
        .add_system(closest_celestial_system())
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
                        scancode: Some(Scancode::C),
                        ..
                    } => {
                        let mut camera_mode_res = resources.get_mut::<CameraMode>().unwrap();
                        let new_mode = if *camera_mode_res == CameraMode::Default {
                            CameraMode::Landing
                        } else {
                            CameraMode::Default
                        };
                        *camera_mode_res = new_mode;
                    }
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
                            Scancode::C => Some(PlayerInput::SwitchCamera),
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
                            Scancode::C => Some(&PlayerInput::SwitchCamera),
                            _ => None,
                        };
                        if let Some(player_input) = insertion {
                            pinput.remove(player_input);
                        }
                    }
                    Event::Window {
                        win_event: WindowEvent::Resized(x, y),
                        ..
                    } => {
                        let mut window_size = resources.get_mut::<WindowSize>().unwrap();
                        window_size.0 = ivec2(x, y);
                    }
                    _ => {}
                }
            }
        }

        schedule.execute(&mut world, &mut resources);
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
