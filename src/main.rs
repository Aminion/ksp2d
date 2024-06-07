pub mod ksp2d;

extern crate crossbeam;
extern crate glam;
extern crate legion;
extern crate rand;
extern crate sdl2;

use legion::world::SubWorld;
use log::{info, warn};
use ndarray::{arr1, arr2, Array2};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::mixer::InitFlag;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::{event, EventPump};
use sdl2::{event::Event, keyboard::Scancode};
use std::collections::HashSet;
use std::io::Write;
use std::{
    f32::consts::PI,
    ops::Add,
    time::{Duration, Instant},
};

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
        .present_vsync()
        .build()
        .expect("could not make a canvas");
    let event_pump = sdl_context.event_pump().unwrap();

    Ok((canvas, event_pump))
}

fn rotation_mtx(a: &f64) -> Array2<f64> {
    let sin_a = a.sin();
    let cos_a = a.cos();
    arr2(&[[cos_a, -sin_a], [sin_a, cos_a]])
}
#[derive(Clone, Copy, Debug, PartialEq)]
struct Position {
    x: f64,
    y: f64,
    a: f64,
}

#[system(for_each)]
fn update_positions(
    pos: &mut Position,
    #[resource] dt: &Duration,
    #[resource] input: &HashSet<PlayerInput>,
) {
    const ANGLE_SPD: f64 = std::f64::consts::PI;
    const LINEAR_SPD: f64 = 64f64;

    if input.contains(&PlayerInput::RotateRight) {
        pos.a += ANGLE_SPD * dt.as_secs_f64();
    } else if input.contains(&PlayerInput::RotateLeft) {
        pos.a -= ANGLE_SPD * dt.as_secs_f64();
    }
    let r_mtx = rotation_mtx(&pos.a);
    let d = LINEAR_SPD * dt.as_secs_f64();
    if input.contains(&PlayerInput::MoveRight) {
        let v = r_mtx.dot(&arr1(&[d, 0f64]));
        pos.x += v[0];
        pos.y += v[1];
    } else if input.contains(&PlayerInput::MoveLeft) {
        let v = r_mtx.dot(&arr1(&[-d, 0f64]));
        pos.x += v[0];
        pos.y += v[1];
    }
    if input.contains(&PlayerInput::MoveForward) {
        let v = r_mtx.dot(&arr1(&[0f64, -d]));
        pos.x += v[0];
        pos.y += v[1];
    } else if input.contains(&PlayerInput::MoveBackward) {
        let v = r_mtx.dot(&arr1(&[0f64, d]));
        pos.x += v[0];
        pos.y += v[1];
    }
}

const COLOR: Color = Color::RGB(0, 255, 255);

#[system]
#[read_component(Position)]
fn render(#[resource] canvas: &mut WindowCanvas, world: &SubWorld) {
    info!("render");
    let mut position_query = <&Position>::query();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    fn tran(c: &f64) -> i16 {
        c.round() as i16
    }
    for position in position_query.iter(world) {
        let offset_vec = arr1(&[position.x, position.y]);
        let r_mtx = rotation_mtx(&position.a);
        let pt0 = r_mtx
            .dot(&arr1(&[-25.0, 0.0]))
            .map(tran)
            .add(&offset_vec.map(tran));
        let pt1 = r_mtx
            .dot(&arr1(&[0.0, -43.3013]))
            .map(tran)
            .add(&offset_vec.map(tran));
        let pt2 = r_mtx
            .dot(&arr1(&[25.0, 0.0]))
            .map(tran)
            .add(&offset_vec.map(tran));
        canvas.filled_trigon(pt0[0], pt0[1], pt1[0], pt1[1], pt2[0], pt2[1], COLOR);
        canvas.line(pt2[0], pt2[1], pt0[0], pt0[1], Color::RGB(255, 0, 0));
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
        y: 200f64,
        x: 200f64,
        a: 0f64,
    },));

    let mut schedule = Schedule::builder()
        .add_system(update_positions_system())
        .flush()
        .add_thread_local(render_system())
        .build();
    'running: loop {
        let dt = frame.elapsed();
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
