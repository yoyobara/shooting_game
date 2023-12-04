#[path = "../common.rs"]
mod common;

use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let video_subsystem = sdl_ctx.video().unwrap();

    let window = video_subsystem.window("Rust Shooter Game", 600, 600).position_centered().build().unwrap();
    let mut renderer = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_ctx.event_pump().unwrap();

    'mainloop: loop {
        for e in event_pump.poll_iter() {
            match e {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'mainloop;
                }
                _ => {}
            }
        }

        renderer.set_draw_color(Color::GREY);
        renderer.clear();
        renderer.present();
    }
}
