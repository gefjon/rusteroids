extern crate sdl2;

use sdl2::pixels::Color;
use super::globals::*;
use sdl2::gfx::primitives::DrawRenderer;
use super::shooting::Shot;
use std;

#[derive(Copy, Clone)]
pub enum Sizes {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
}

impl Sizes {
    fn get_smaller_size(self) -> Option<Sizes> {
        match self {
            Tiny => None,
            Small => Some(Sizes::Tiny),
            Medium => Some(Sizes::Small),
            Large => Some(Sizes::Medium),
            Huge => Some(Sizes::Large),
        }
    }
}

pub struct Asteroid {
    radius: f32,
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    color: Color,
    size: Sizes,
    angle: f32,
    speed: f32,
}

impl Asteroid {
    fn little_baby_asteroids(&self) -> Vec<Asteroid> {
        if let Some(size) = self.size.get_smaller_size() {
            return vec![
                Asteroid::new(self.x, self.y, self.angle, self.speed, size),
                Asteroid::new(self.x, self.y, self.angle + std::f32::consts::FRAC_PI_2, self.speed, size),
                Asteroid::new(self.x, self.y, self.angle + std::f32::consts::PI, self.speed, size),
                Asteroid::new(self.x, self.y, self.angle - std::f32::consts::FRAC_PI_2, self.speed, size)
            ];
        }
        return vec![];
    }
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
    pub fn update(&self, shots: &Vec<Shot>) -> Vec<Asteroid> {
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

        for shot in shots.iter() {
            let xdist = x - shot.x;
            let ydist = y - shot.y;
            if ((xdist * xdist) + (ydist * ydist)) < (self.radius * self.radius) {
                return self.little_baby_asteroids();
            }
        }
        
        vec![
            Asteroid {
                x: x,
                y: y,
                dx: self.dx,
                dy: self.dy,
                radius: self.radius,
                color: self.color,
                size: self.size,
                speed: self.speed,
                angle: self.angle,
            }
        ]
    }
    pub fn new(starting_x: f32, starting_y: f32, direction: f32, vel: f32, size: Sizes) -> Asteroid {
        Asteroid {
            radius: 16.0f32,
            x: starting_x,
            y: starting_y,
            dx: (direction.cos() * vel),
            dy: (direction.sin() * vel),
            color: ASTEROID_COLOR,
            size: size,
            angle: direction,
            speed: vel,
        }
    }
}
