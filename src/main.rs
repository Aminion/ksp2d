pub mod ksp2d;

extern crate crossbeam;
extern crate glam;
extern crate legion;
extern crate rand;
extern crate sdl2;

use core::f64;
use glam::{dvec2, vec2, DVec2, Vec2};
use ksp2d::components::rocket::Position;
use ksp2d::systems::render::render_system;
use ksp2d::systems::rocket::update_positions_system;
use log::info;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::mixer::InitFlag;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use sdl2::{event::Event, keyboard::Scancode};
use std::collections::HashSet;
use std::time::Instant;

use legion::*;

use crate::ksp2d::components::rocket::PlayerInput;

pub struct Dt(f64);

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
        //.present_vsync()
        .build()
        .expect("could not make a canvas");
    let event_pump = sdl_context.event_pump().unwrap();

    Ok((canvas, event_pump))
}

pub fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let (mut canvas, mut event_pump) = initialize().unwrap();
    let mut frame = Instant::now();
    let mut world = World::default();
    let mut resources = Resources::default();
    //resources.insert(canvas);
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

    let mut pl1 = Obj {
        mass: 5.9722e24,
        pos: dvec2(149597870700.0, 0.0),
        vel: dvec2(0.0, 111111110.0),
        acc: dvec2(0.0, 0.0),
    };

    let mut pl2 = Obj {
        mass: 1.9884e30,
        pos: dvec2(149597870700.0 * 2.0, 149597870700.0),
        vel: dvec2(0.0, 0.0),
        acc: dvec2(0.0, 0.0),
    };

    let mut pl3 = Obj {
        mass: 6.39e23,
        pos: dvec2(149597870700.0 / 4.0, 149597870700.0),
        vel: dvec2(-0.0, 0.0),
        acc: dvec2(0.0, 0.0),
    };

    let mut planets = vec![pl1, pl2, pl3];

    'running: loop {
        let dt = Dt(frame.elapsed().as_secs_f64());
        let dtt = dt.0;
        info!("FPS {}", 1.0f64 / dt.0);
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
        //schedule.execute(&mut world, &mut resources);
        n_body_iter(&mut planets, dtt * 100.0);

        let s_e = (planets[0].pos * 1.7112543e-9).as_i16vec2();
        let s_s = (planets[1].pos * 1.7112543e-9).as_i16vec2();
        let s_m = (planets[2].pos * 1.7112543e-9).as_i16vec2();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        //canvas.clear();
        let _ = canvas.pixel(100 + s_e.x, 100 + s_e.y, Color::RGB(0, 255, 0));
        let _ = canvas.pixel(100 + s_s.x, 100 + s_s.y, Color::RGB(255, 255, 0));
        let _ = canvas.pixel(100 + s_m.x, 100 + s_m.y, Color::RGB(255, 0, 0));
        canvas.present();
    }
}

fn n_body_iter(objs: &mut Vec<Obj>, dt: f64) {
    for i in 0..objs.len() {
        let mut a = dvec2(0.0, 0.0);
        for j in 0..objs.len() {
            if i == j {
                continue;
            }

            let dist = objs[j].pos - objs[i].pos;
            let dist_sq = dist.dot(dist);
            let dist_cu = dist_sq.sqrt() * dist_sq;
            let f = G * objs[i].mass * objs[j].mass / dist_sq;
            a += f * dist / dist_cu;
        }
        objs[i].acc += a;
    }

    for o in objs.iter_mut() {
        o.vel += o.acc * dt;
        o.pos += o.vel * dt;
    }
}

const G: f64 = physical_constants::NEWTONIAN_CONSTANT_OF_GRAVITATION;
#[derive(Copy, Clone)]
struct Obj {
    pos: DVec2,
    vel: DVec2,
    acc: DVec2,
    mass: f64,
}
