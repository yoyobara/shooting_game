#[path = "../common.rs"]
mod common;
mod math;
mod player;

use sdl2::{
    event::Event, keyboard::Keycode,
    gfx::framerate::FPSManager, joystick::Joystick
};

use math::{Vec2D, joystick_normalize_and_deadband};
use player::Player;

/*
 * updates the game
 */
fn update(p: &mut Player, joystick: &Joystick) {
    let joystick_vec: Vec2D = Vec2D::new(joystick.axis(0).unwrap() as f32, joystick.axis(1).unwrap() as f32);
    p.velocity = joystick_normalize_and_deadband(joystick_vec, 5.0);

    p.tick_move();
}

fn main() -> Result<(), String> {
    let sdl_ctx = sdl2::init()?;
    let video_subsystem = sdl_ctx.video()?;

    // video variables
    let window = video_subsystem.window("Rust Shooter Game", 600, 600).position_centered().build().map_err(|e| e.to_string())?;
    let mut renderer = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_ctx.event_pump()?; 
    let mut fps_manager = FPSManager::new();
    fps_manager.set_framerate(30)?;

    let joystick = sdl_ctx.joystick()?.open(0).map_err(|e| e.to_string())?;

    // game variables
    let mut player = Player::new("yotam".to_string(), Vec2D::new(300.0, 300.0));

    'mainloop: loop {
        for e in event_pump.poll_iter() {
            match e {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'mainloop;
                }
                _ => {}
            }
        }
        update(&mut player, &joystick);
        renderer.present();
        fps_manager.delay();
    }

    Ok(())
}
