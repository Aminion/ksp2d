pub mod fonts;
pub mod ksp2d;

extern crate crossbeam;
extern crate glam;
extern crate legion;
extern crate rand;
extern crate sdl2;

use fonts::{load_fonts, FontRenderer};
use glam::{dvec2, ivec2, DVec2, I16Vec2, IVec2};
use ksp2d::components::celestial_body::{CelestialBody, CelestialBodyType};
use ksp2d::components::newton_body::NewtonBody;
use ksp2d::components::rocket::Rocket;
use ksp2d::systems::newton_body::celestial_body_system;
use ksp2d::systems::performance_info::{update_info_system, PerformanceInfo};
use ksp2d::systems::render::render_system;
use ksp2d::systems::rocket::update_positions_system;
use rand::Rng;
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

const SPACE_SIZE: f64 = 4.5029e+12 / 512.0;
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

    let initial_size = ivec2(INITIAL_WINDOW_WIDTH as i32, INITIAL_WINDOW_HEIGHT as i32);
    let (a, b) = get_scaling(initial_size.x, initial_size.y);
    resources.insert(SpaceScale(a));
    resources.insert(WindowSize(initial_size));
    resources.insert(SpacePadding(b));
    resources.insert(FrameTimer(Instant::now()));
    resources.insert(FrameDuration(Duration::ZERO));
    resources.insert(Dt(0.0));
    resources
}

fn get_system(
    planets_count: usize,
    mass_min: f64,
    mass_max: f64,
    radius_min: f64,
    radius_max: f64,
) -> Vec<(CelestialBody, NewtonBody)> {
    let mut v: Vec<(CelestialBody, NewtonBody)> = Vec::with_capacity(planets_count);
    let mut rng = rand::rng();
    let half_space = SPACE_SIZE / 2.0;
    let system_center = dvec2(half_space, half_space);
    let star = (
        CelestialBody {
            b_type: CelestialBodyType::Star,
            color: Color::YELLOW,
            radius: 696340000.0,
        },
        NewtonBody {
            angle: DVec2::Y,
            angular_vel: 1.0,
            mass: 1.98847e30,
            pos: system_center,
            vel: DVec2::ZERO,
            acc: DVec2::ZERO,
        },
    );
    v.push(star);

    let mut orb = star.0.radius * 2.0;

    for _ in 0..planets_count {
        let mass = rng.random_range(mass_min..mass_max);
        orb += rng.random_range(radius_min..radius_max);
        let radius = orb;
        let angle = rng.random_range(0.0..2.0 * std::f64::consts::PI);
        let position = DVec2::from_angle(angle) * radius + system_center;
        let orbital_speed =
            (physical_constants::NEWTONIAN_CONSTANT_OF_GRAVITATION * star.1.mass / radius).sqrt();

        let velocity = DVec2::new(-position.y, position.x).normalize() * orbital_speed;
        let planet = (
            CelestialBody {
                b_type: CelestialBodyType::Planet,
                color: Color::GREEN,
                radius: 400000000.0,
            },
            NewtonBody {
                angle: DVec2::Y,
                angular_vel: 8.0,
                mass: mass,
                pos: position,
                vel: velocity,
                acc: DVec2::ZERO,
            },
        );
        v.push(planet);
    }
    v
}

fn initial_world() -> World {
    let mut world = World::default();
    let rocket_body = NewtonBody {
        angle: DVec2::Y,
        angular_vel: 0.0,
        mass: 2965000.0,
        pos: dvec2(SPACE_SIZE / 4.0, SPACE_SIZE / 4.0),
        vel: DVec2::ZERO,
        acc: DVec2::ZERO,
    };

    world.push((Rocket {}, rocket_body));
    let sys = get_system(3, 3.30104e23, 1898.6e24, 24397000.0, 142800000.0);
    world.extend(sys);
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
                        win_event: WindowEvent::Resized(x, y),
                        ..
                    } => {
                        let (a, b) = get_scaling(x, y);
                        println!("{} {} {} {}", x, y, a, b);
                        let mut r = resources.get_mut::<SpaceScale>().unwrap();
                        let mut z = resources.get_mut::<WindowSize>().unwrap();
                        let mut e = resources.get_mut::<SpacePadding>().unwrap();

                        r.0 = a;
                        z.0 = ivec2(x, y);
                        e.0 = b;
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
    fn padding(a: f64, b: f64) -> i16 {
        ((a - b) * 0.5) as i16
    }
    if x > y {
        (y_f / SPACE_SIZE, I16Vec2::new(padding(x_f, y_f), 0))
    } else if y < x {
        (x_f / SPACE_SIZE, I16Vec2::new(0, padding(y_f, x_f)))
    } else {
        (x_f / SPACE_SIZE, I16Vec2::ZERO)
    }
}
