extern crate sdl2;

use sdl2::pixels::Color;
use super::globals::*;
use sdl2::gfx::primitives::DrawRenderer;

pub struct Asteroid {
    radius: f32,
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    color: Color,
}

impl Asteroid {
    pub fn draw(&self, renderer: &sdl2::render::Renderer) {
        // a very simple draw function
        // asteroids are just circles
        renderer.filled_circle::<Color>(
            self.x.round() as i16,
            self.y.round() as i16,
            self.radius as i16,
            self.color
        )
            .unwrap();
    }
    pub fn update(&self) -> Option<Asteroid> {
        let mut x = self.x + self.dx;

        // this algorithm for screen-wrapping is the same as the one in rocket.rs
        // wrap around when the whole asteroid is offscreen and wrap it to just offscreen
        // for visual cleanliness
        if (x + self.radius) < 0.0f32 {
            x += (WINDOW_DIMENSIONS.0 as f32) + self.radius * 2.0f32;
        }
        if (x - self.radius) > (WINDOW_DIMENSIONS.0 as f32) {
            x -= (WINDOW_DIMENSIONS.0 as f32) + self.radius * 2.0f32;
        }
        let mut y = self.y + self.dy;
        if (y + self.radius) < 0.0f32 {
            y += (WINDOW_DIMENSIONS.1 as f32) + self.radius * 2.0f32;
        }
        if (y - self.radius) > (WINDOW_DIMENSIONS.1 as f32) {
            y -= (WINDOW_DIMENSIONS.1 as f32) + self.radius * 2.0f32;
        }

        // one day asteroids will interact and be destroyed
        // but not yet
        Some(
            Asteroid {
            x: x,
            y: y,
            dx: self.dx,
            dy: self.dy,
            radius: self.radius,
            color: self.color,
            }
        )
    }
    pub fn new(starting_x: f32, starting_y: f32, radius: f32, direction: f32, vel: f32) -> Asteroid {
        Asteroid {
            radius: radius,
            x: starting_x,
            y: starting_y,
            dx: (direction.cos() * vel),
            dy: (direction.sin() * vel),
            color: ASTEROID_COLOR,
        }
    }
}
