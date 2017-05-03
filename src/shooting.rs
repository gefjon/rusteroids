extern crate sdl2;

use sdl2::pixels::Color;
use super::globals::*;
use sdl2::gfx::primitives::DrawRenderer;
// the more primitive draw functions like filled_circle
// are part of the trait DrawRenderer as opposed to
// just being Renderer methods

pub struct Shot {
    x: f32,
    y: f32,
    dx: f32, // storing direction and calculating dy and dx every frame would be expensive
    dy: f32, // and since it's constant it doesn't matter
    radius: f32,
    color: Color,
}

impl Shot {
    pub fn draw(&self, renderer: &sdl2::render::Renderer) {
        // simple draw function
        // shots are just filled circles
        renderer.filled_circle(
            self.x.round() as i16,
            self.y.round() as i16,
            self.radius as i16,
            self.color
        )
            .unwrap();
    }
    pub fn update(&self) -> Option<Shot> {
        let x = self.x + self.dx;
        let y = self.y + self.dy;

        // the shot ceases to exist if it fully leaves any screen boundary
        // so return None in these cases
        if (x + self.radius) < 0.0f32 {
            return None;
        }
        if (x - self.radius) > (WINDOW_DIMENSIONS.0 as f32) {
            return None;
        }
        if (y + self.radius) < 0.0f32 {
            return None;
        }
        if (y - self.radius) > (WINDOW_DIMENSIONS.1 as f32) {
            return None;
        }
        
        Some(
            Shot {
                x: x,
                y: y,
                dx: self.dx,
                dy: self.dy,
                radius: self.radius,
                color: self.color,
            }
        )
    }
    pub fn new(x: f32, y: f32, angle: f32) -> Shot {
        Shot {
            x: x,
            y: y,
            radius: SHOT_RADIUS,
            color: SHOT_COLOR,
            dx: angle.cos() * SHOT_SPEED,
            dy: angle.sin() * SHOT_SPEED,
        }
    }
}
            
