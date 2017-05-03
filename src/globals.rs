extern crate sdl2;

use sdl2::pixels::Color;

pub const WINDOW_DIMENSIONS: (u32, u32) = (640u32, 480u32);

pub const WINDOW_TITLE: &'static str = "Rusteroids";

pub const BACKGROUND_COLOR: Color = Color::RGB(0, 153, 204);
pub const ROCKET_COLOR: Color = Color::RGB(0, 0, 0);
pub const ASTEROID_COLOR: Color = Color::RGB(86, 31, 3);

pub const ROCKET_DIMENSIONS: (f32, f32) = (16.0f32, 32.0f32);
pub const ROCKET_ACCELERATION: f32 = 0.2f32;
pub const ROCKET_SPIN_SPEED: f32 = 0.2f32;

pub const MAX_FRAMERATE: u32 = 30u32;
