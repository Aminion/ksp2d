
extern crate crossbeam;
extern crate glam;
extern crate legion;
extern crate rand;
extern crate sdl2;

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::mixer::InitFlag;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use sdl2::event::Event;
use std::time::Duration;


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
        .build()
        .expect("could not make a canvas");

    // Render first empty framed
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.filled_ellipse(50, 50, 15, 35, Color::RGB(0, 0, 255));
    canvas.present();

    let event_pump = sdl_context.event_pump().unwrap();
    
    Ok((canvas, event_pump))
}

pub fn main() -> () {
    let r = initialize().unwrap();
    loop {}
}
