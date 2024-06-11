pub mod ksp2d;

extern crate crossbeam;
extern crate glam;
extern crate legion;
extern crate rand;
extern crate sdl2;

use glam::DVec2;
use ksp2d::components::rocket::Position;
use ksp2d::systems::render::render_system;
use ksp2d::systems::rocket::update_positions_system;
use log::info;
use sdl2::mixer::InitFlag;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use sdl2::{event::Event, keyboard::Scancode};
use std::collections::HashSet;
use std::time::Instant;

use legion::*;

use crate::ksp2d::components::rocket::PlayerInput;

fn initialize() -> Result<(WindowCanvas, EventPump), String> {
    // Initialize libraries
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _audio = sdl_context.audio()?;

    let _mixer_context =
        sdl2::mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG)?;

    // Number of mixing channels available for sound effect `Chunk`s to play
    // simultaneously.
    sdl2::mixer::allocate_channels(20);

    // Initialize window systems
    let window = video_subsystem
        .window("KSP 2D", 1280, 720)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let canvas = window
        .into_canvas()
        .accelerated()
        //  .present_vsync()
        .build()
        .expect("could not make a canvas");
    let event_pump = sdl_context.event_pump().unwrap();

    Ok((canvas, event_pump))
}

pub fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let (canvas, mut event_pump) = initialize().unwrap();
    let mut frame = Instant::now();
    let mut world = World::default();
    let mut resources = Resources::default();
    resources.insert(canvas);
    resources.insert(HashSet::<PlayerInput>::new());
    world.push((Position {
        p: DVec2::new(200f64, 200f64),
        a: 0f64,
    },));

    let mut schedule = Schedule::builder()
        .add_system(update_positions_system())
        .flush()
        .add_thread_local(render_system())
        .build();
    'running: loop {
        let dt = frame.elapsed();
        info!("FPS {}", 1.0f64 / dt.as_secs_f64());
        frame = Instant::now();
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
                    } => match code {
                        Scancode::A => {
                            pinput.insert(PlayerInput::MoveLeft);
                        }
                        Scancode::D => {
                            pinput.insert(PlayerInput::MoveRight);
                        }
                        Scancode::W => {
                            pinput.insert(PlayerInput::MoveForward);
                        }
                        Scancode::S => {
                            pinput.insert(PlayerInput::MoveBackward);
                        }
                        Scancode::Q => {
                            pinput.insert(PlayerInput::RotateLeft);
                        }
                        Scancode::E => {
                            pinput.insert(PlayerInput::RotateRight);
                        }
                        _ => (),
                    },
                    Event::KeyUp {
                        scancode: Some(code),
                        ..
                    } => match code {
                        Scancode::A => {
                            pinput.remove(&PlayerInput::MoveLeft);
                        }
                        Scancode::D => {
                            pinput.remove(&PlayerInput::MoveRight);
                        }
                        Scancode::W => {
                            pinput.remove(&PlayerInput::MoveForward);
                        }
                        Scancode::S => {
                            pinput.remove(&PlayerInput::MoveBackward);
                        }
                        Scancode::Q => {
                            pinput.remove(&PlayerInput::RotateLeft);
                        }
                        Scancode::E => {
                            pinput.remove(&PlayerInput::RotateRight);
                        }
                        _ => (),
                    },
                    _ => {}
                }
            }
        }
        schedule.execute(&mut world, &mut resources);
    }
}
