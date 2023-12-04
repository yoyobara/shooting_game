#[path = "../common.rs"]
mod common;

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};

struct Player {
    name: String,
    pos: Rect,
}

fn main() -> Result<(), String> {
    let sdl_ctx = sdl2::init()?;
    let video_subsystem = sdl_ctx.video()?;

    let window = video_subsystem.window("Rust Shooter Game", 600, 600).position_centered().build().map_err(|e| e.to_string())?;
    let mut renderer = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_ctx.event_pump()?;

    let joystick = sdl_ctx.joystick()?.open(0).map_err(|e| e.to_string())?;

    'mainloop: loop {
        for e in event_pump.poll_iter() {
            match e {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'mainloop;
                }
                _ => {}
            }
        }

        let i = joystick.axis(5).map_err(|e| e.to_string())? / 256;
        let j = (i + 128) as u8;
        println!("{}", j);
        renderer.set_draw_color(Color::RGB(j, j, j));
        renderer.clear();
        renderer.present();
    }

    Ok(())
}
