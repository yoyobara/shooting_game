use sdl2::{render::Canvas, video::Window, gfx::primitives::DrawRenderer, pixels::Color};

use crate::math::{Vec2D, ZERO};


pub struct Player {
    pub name: String,
    pub position: Vec2D,
    pub velocity: Vec2D, // per tick ig
}

impl Player {

    pub fn new(name: String, position: Vec2D) -> Player {
        Player { name, position, velocity: ZERO }
    }

    pub fn tick_move(&mut self) {
        self.position = self.position + self.velocity;
    }

    pub fn draw(&self, renderer: &mut Canvas<Window>) {
        renderer.filled_circle(self.position.x as i16, self.position.y as i16, 20, Color::WHITE).unwrap();
    }
}

