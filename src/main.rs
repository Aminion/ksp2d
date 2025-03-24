pub mod ksp2d;

extern crate crossbeam;
extern crate glam;
extern crate legion;
extern crate rand;
extern crate sdl2;

use core::f64;
use glam::dvec2;
use ksp2d::components::celestial_body::CelestialBody;
use ksp2d::components::newton_body::NewtonBody;
use ksp2d::components::rocket::Rocket;
use ksp2d::systems::newton_body::celestial_body_system;
use ksp2d::systems::render::render_system;
use ksp2d::systems::rocket::update_positions_system;
use log::info;
use sdl2::event::WindowEvent;
use sdl2::mixer::InitFlag;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use sdl2::{event::Event, keyboard::Scancode};
use std::collections::HashSet;
use std::time::Instant;

use legion::*;

use crate::ksp2d::components::rocket::PlayerInput;

pub struct Dt(f64);
pub struct SpaceScale(f64);

const SPACE_SIZE: f64 = 4.5029e+12 / 8.0;

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

pub fn main() {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let (canvas, mut event_pump) = initialize().unwrap();
    let mut frame = Instant::now();
    let mut frame_time = Instant::now();
    let mut world = World::default();
    let mut resources = Resources::default();
    resources.insert(canvas);
    resources.insert(HashSet::<PlayerInput>::new());
    resources.insert(SpaceScale(1280.0 / SPACE_SIZE));

    let mut schedule = Schedule::builder()
        .add_system(update_positions_system())
        .add_system(celestial_body_system())
        .flush()
        .add_thread_local(render_system())
        .build();

    let rocket_body = NewtonBody {
        a: 0.0,
        a_vel: 0.0,
        mass: 2965000.0,
        pos: dvec2(149597870700.0 / 4.0, 149597870700.0),
        prev_pos: dvec2(149597870700.0 / 4.0, 149597870700.0),
        vel: dvec2(-0.0, 0.0),
        acc: dvec2(0.0, 0.0),
    };

    world.push((Rocket {}, rocket_body));

    let pl1 = NewtonBody {
        a: 0.0,
        a_vel: 0.0,
        mass: 5.9722e24,
        pos: dvec2(149597870700.0, 0.0),
        prev_pos: dvec2(149597870700.0, 0.0),
        vel: dvec2(0.0, 111111110.0),
        acc: dvec2(0.0, 0.0),
    };

    let pl2 = NewtonBody {
        a: 0.0,
        a_vel: 0.0,
        mass: 1.9884e30,
        pos: dvec2(149597870700.0 * 2.0, 149597870700.0),
        prev_pos: dvec2(149597870700.0 * 2.0, 149597870700.0),
        vel: dvec2(0.0, 0.0),
        acc: dvec2(0.0, 0.0),
    };

    world.extend(vec![(CelestialBody {}, pl1), (CelestialBody {}, pl2)]);

    'running: loop {
        let dt = Dt(frame.elapsed().as_secs_f64());
        info!("FPS {}", dt.0.recip());
        frame = Instant::now();
        frame_time = Instant::now();
        resources.insert(dt);
        {
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
                        r.0 = x.max(y) as f64 / SPACE_SIZE;
                    }
                    _ => {}
                }
            }
        }
        schedule.execute(&mut world, &mut resources);
        let dt_frame_time = frame_time.elapsed().as_millis();
        info!("FRAME TIME {} MS", dt_frame_time);
    }
}
