use std::{ops, task::Wake};

// trait for type in vector

#[derive(Clone, Copy)]
pub struct Vec2D {
    pub x: f32,
    pub y: f32
}

impl Vec2D {
    pub const fn new(x: f32, y: f32) -> Self {
        Vec2D {x, y}
    }

    pub fn from_polar(magnitude: f32, angle: f32) {
        // creates a new vector using polar representation.
        // angle is in radians
        
        Vec2D{ x: angle.cos(), y: angle.sin() } * magnitude
    }
}

impl ops::Add for Vec2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl ops::Mul<f32> for Vec2D {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {x: self.x * rhs, y: self.y * rhs }
    }
}

impl ops::Neg for Vec2D { 
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {x: -self.x, y: -self.y}
    }
}

impl ops::Sub for Vec2D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

pub const ZERO: Vec2D = Vec2D::new(0.0, 0.0);

pub fn movement_normalize_and_deadband(joystick_vec: Vec2D, max_velocity: f32) -> Vec2D {
    // deadbands a joystick movement input 
    const DEADBAND_RATIO: f32 = 0.15;
    let deadband_threshold: f32 = max_velocity * DEADBAND_RATIO;

    let mut velocity_vec: Vec2D = joystick_vec * (max_velocity/i16::MAX as f32);

    if velocity_vec.x.abs() < deadband_threshold {
        velocity_vec.x = 0.0;
    }
    if velocity_vec.y.abs() < deadband_threshold {
        velocity_vec.y = 0.0;
    }
    velocity_vec
}

pub fn aim_to_angle(joystick_vec: Vec2D) -> Vec2D {
    // controller aim input to normalized vector (deadband also)
    const DEADBAND_RATIO: f32 = 0.15;
    let deadband_threshold = i16::MAX as f32 * DEADBAND_RATIO;

    let angle = 
}
