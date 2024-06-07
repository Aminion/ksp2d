pub mod ksp2d;

extern crate crossbeam;
extern crate glam;
extern crate legion;
extern crate rand;
extern crate sdl2;

use glam::{DMat2, DVec2};
use legion::world::SubWorld;
use log::info;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::mixer::InitFlag;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use sdl2::{event::Event, keyboard::Scancode};
use std::collections::HashSet;
use std::time::{Duration, Instant};

use legion::*;

use crate::ksp2d::components::rocket::PlayerInput;

fn initialize<'a, 'b>() -> Result<(WindowCanvas, EventPump), String> {
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

#[derive(Clone, Copy, Debug, PartialEq)]
struct Position {
    p: DVec2,
    a: f64,
}

#[inline(always)]
fn rotate_vec_by_mtx(r_mtx: &DMat2, v: DVec2) -> DVec2 {
    DVec2::new(r_mtx.row(0).dot(v), r_mtx.row(1).dot(v))
}

#[system(for_each)]
fn update_positions(
    pos: &mut Position,
    #[resource] dt: &Duration,
    #[resource] input: &HashSet<PlayerInput>,
) {
    const ANGLE_SPD: f64 = std::f64::consts::PI;
    const LINEAR_SPD: f64 = 64f64;

    let dt_s = dt.as_secs_f64();

    if input.contains(&PlayerInput::RotateRight) {
        pos.a += ANGLE_SPD * dt_s;
    } else if input.contains(&PlayerInput::RotateLeft) {
        pos.a -= ANGLE_SPD * dt_s;
    }

    let r_mtx = DMat2::from_angle(pos.a);

    if input.contains(&PlayerInput::MoveRight) {
        let local = DVec2::new(LINEAR_SPD * dt_s, 0f64);
        pos.p += rotate_vec_by_mtx(&r_mtx, local);
    } else if input.contains(&PlayerInput::MoveLeft) {
        let local = DVec2::new(-LINEAR_SPD * dt_s, 0f64);
        pos.p += rotate_vec_by_mtx(&r_mtx, local);
    }
    if input.contains(&PlayerInput::MoveForward) {
        let local = DVec2::new(0f64, -LINEAR_SPD * dt_s);
        pos.p += rotate_vec_by_mtx(&r_mtx, local);
    } else if input.contains(&PlayerInput::MoveBackward) {
        let local = DVec2::new(0f64, LINEAR_SPD * dt_s);
        pos.p += rotate_vec_by_mtx(&r_mtx, local);
    }
}

const COLOR: Color = Color::RGB(0, 255, 255);

#[system]
#[read_component(Position)]
fn render(#[resource] canvas: &mut WindowCanvas, world: &SubWorld) {
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

        canvas.filled_trigon(p0.x, p0.y, p1.x, p1.y, p2.x, p2.y, COLOR);
        canvas.line(p2.x, p2.y, p0.x, p0.y, Color::RGB(255, 0, 0));
    }
    canvas.present();
}

pub fn main() -> () {
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
