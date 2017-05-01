extern crate sdl2;

use sdl2::pixels::Color;
use super::globals::*;
use sdl2::gfx::primitives::DrawRenderer;

pub struct Asteroid {
    radius: i16,
    x: i16,
    y: i16,
    dx: i16,
    dy: i16,
    color: Color,
}

impl Asteroid {
    pub fn draw(&self, renderer: &sdl2::render::Renderer) {
        renderer.filled_circle::<Color>(self.x, self.y, self.radius, self.color).unwrap();
    }
    pub fn update(&self) -> Asteroid {
        let mut x = self.x + self.dx;
        if (x + self.radius) < 0 {
            x += (WINDOW_DIMENSIONS.0 as i16) + self.radius * 2;
        }
        if (x - self.radius) > (WINDOW_DIMENSIONS.0 as i16) {
            x -= (WINDOW_DIMENSIONS.0 as i16) + self.radius * 2;
        }
        let mut y = self.y + self.dy;
        if (y + self.radius) < 0 {
            y += (WINDOW_DIMENSIONS.1 as i16) + self.radius * 2;
        }
        if (y - self.radius) > (WINDOW_DIMENSIONS.1 as i16) {
            y -= (WINDOW_DIMENSIONS.1 as i16) + self.radius * 2;
        }

        Asteroid {
            x: x,
            y: y,
            dx: self.dx,
            dy: self.dy,
            radius: self.radius,
            color: self.color,
        }
    }
    pub fn new(starting_x: i16, starting_y: i16, radius: i16, dx: i16, dy: i16) -> Asteroid {
        Asteroid {
            radius: radius,
            x: starting_x,
            y: starting_y,
            dx: dx,
            dy: dy,
            color: ASTEROID_COLOR,
        }
    }
}
