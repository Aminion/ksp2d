extern crate crossbeam;
extern crate glam;
extern crate legion;
extern crate rand;
extern crate sdl2;

use ndarray::{arr1, arr2, Array2};
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::mixer::InitFlag;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use std::{
    f32::consts::PI,
    ops::Add,
    time::{Duration, Instant},
};

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

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .expect("could not make a canvas");
    let event_pump = sdl_context.event_pump().unwrap();

    Ok((canvas, event_pump))
}

fn rotationMtx(a: f64) -> Array2<f64> {
    let sin_a = a.sin();
    let cos_a = a.cos();
    let x = arr2(&[[cos_a, -sin_a], [sin_a, cos_a]]);
    x
}

pub fn main() -> () {
    const ANGLE_SPD: f64 = std::f64::consts::PI;
    const COLOR: Color = Color::RGB(0, 255, 255);
    let (mut canvas, mut e) = initialize().unwrap();
    let mut frame = Instant::now();
    let mut angle = 0f64;
    let offset_vec = arr1(&[600i16, 200]);
    'running: loop {
        let dt = frame.elapsed();
        frame = Instant::now();
        println!("{:?}", dt);
        for event in e.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        angle += ANGLE_SPD * dt.as_secs_f64();
        let r_mtx = rotationMtx(angle);
        let pt0 = r_mtx.dot(&arr1(&[-25.0, 0.0])).map(tran).add(&offset_vec);
        let pt1 = r_mtx
            .dot(&arr1(&[0.0, -43.3013]))
            .map(tran)
            .add(&offset_vec);
        let pt2 = r_mtx.dot(&arr1(&[25.0, 0.0])).map(tran).add(&offset_vec);
        fn tran(c: &f64) -> i16 {
            c.round() as i16
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.filled_trigon(pt0[0], pt0[1], pt1[0], pt1[1], pt2[0], pt2[1], COLOR);
        canvas.present();
    }
}
