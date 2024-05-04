pub mod ksp2d;

extern crate crossbeam;
extern crate glam;
extern crate legion;
extern crate rand;
extern crate sdl2;

use ndarray::{arr1, arr2, Array2};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::mixer::InitFlag;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use sdl2::{event::Event, keyboard::Scancode};
use std::collections::HashSet;
use std::{
    f32::consts::PI,
    ops::Add,
    time::{Duration, Instant},
};

use legion::*;

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
}

#[system(for_each)]
fn update_positions(pos: &mut Position, #[resource] time: &Duration) {
    println!("{:?}", time)
}

pub fn main() -> () {
    const ANGLE_SPD: f64 = std::f64::consts::PI;
    const LINEAR_SPD: f64 = 64f64;
    const COLOR: Color = Color::RGB(0, 255, 255);
    let (mut canvas, mut e) = initialize().unwrap();
    let mut frame = Instant::now();
    let mut angle = 0f64;
    let mut offset_vec = arr1(&[600f64, 200f64]);
    let mut code_map: HashSet<Scancode> = HashSet::new();
    let mut world = World::default();
    let entity: Entity = world.push((Position {
        x: 0.600f64,
        y: 200f64,
    },));

    let mut schedule = Schedule::builder()
        .add_system(update_positions_system())
        .build();
    'running: loop {
        let dt = frame.elapsed();
        frame = Instant::now();
        // println!("{:?}",e.poll_iter().count() );
        for event in e.poll_iter() {
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
                    code_map.insert(code);
                }
                Event::KeyUp {
                    scancode: Some(code),
                    ..
                } => {
                    code_map.remove(&code);
                }
                _ => {}
            }
        }

        let r_mtx = rotation_mtx(&angle);
        if code_map.contains(&Scancode::E) {
            angle += ANGLE_SPD * dt.as_secs_f64();
        } else if code_map.contains(&Scancode::Q) {
            angle -= ANGLE_SPD * dt.as_secs_f64();
        }
        if code_map.contains(&Scancode::D) {
            let d = LINEAR_SPD * dt.as_secs_f64();
            let v = r_mtx.dot(&arr1(&[d, 0f64]));
            offset_vec = offset_vec + v;
        } else if code_map.contains(&Scancode::A) {
            let d = LINEAR_SPD * dt.as_secs_f64();
            let v = r_mtx.dot(&arr1(&[-d, 0f64]));
            offset_vec = offset_vec + v;
        }
        if code_map.contains(&Scancode::W) {
            let d = LINEAR_SPD * dt.as_secs_f64();
            let v = r_mtx.dot(&arr1(&[0f64, -d]));
            offset_vec = offset_vec + v;
        } else if code_map.contains(&Scancode::S) {
            let d = LINEAR_SPD * dt.as_secs_f64();
            let v = r_mtx.dot(&arr1(&[0f64, d]));
            offset_vec = offset_vec + v;
        }

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
        fn tran(c: &f64) -> i16 {
            c.round() as i16
        }
        let mut res = Resources::default();
        res.insert(dt);
        schedule.execute(&mut world, &mut res);
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.filled_trigon(pt0[0], pt0[1], pt1[0], pt1[1], pt2[0], pt2[1], COLOR);
        canvas.line(pt2[0], pt2[1], pt0[0], pt0[1], Color::RGB(255, 0, 0));
        canvas.present();
    }
}
