extern crate sdl2;

use sdl2::pixels::{Color};
use super::globals::*;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::{Scancode};
use super::shooting::Shot;

pub struct RocketShip {
    x: f32,
    y: f32,
    semi_width: f32,
    semi_height: f32,
    angle_facing: f32,
    dx: f32,
    dy: f32,
    color: Color,
    acceleration: f32,
    spin_speed: f32,
    shot_delay_counter: u8,
}

impl RocketShip {
    pub fn update(&mut self, keyboard_state: &sdl2::keyboard::KeyboardState) -> Option<Shot> {
        if keyboard_state.is_scancode_pressed(Scancode::W) {
            self.dx += self.angle_facing.cos() * self.acceleration;
            self.dy += self.angle_facing.sin() * self.acceleration;
        }
        if keyboard_state.is_scancode_pressed(Scancode::A) {
            self.angle_facing -= self.spin_speed;
        }
        if keyboard_state.is_scancode_pressed(Scancode::D) {
            self.angle_facing += self.spin_speed;
        }
        
        self.x += self.dx;
        self.y += self.dy;
        
        if (self.x + self.semi_width) < 0.0f32 {
            self.x += WINDOW_DIMENSIONS.0 as f32;
        }
        self.x %= WINDOW_DIMENSIONS.0 as f32;
        
        self.y += self.dy;
        if (self.y + self.semi_height) < 0.0f32 {
            self.y += WINDOW_DIMENSIONS.1 as f32;
        }
        self.y %= WINDOW_DIMENSIONS.1 as f32;

        if (
            (self.shot_delay_counter <= 0) &&
            keyboard_state.is_scancode_pressed(Scancode::Space)
        ) {
            self.shot_delay_counter += ROCKET_SHOT_DELAY;
            return Some(
                Shot::new(self.x, self.y, self.angle_facing)
            );
        } else if (self.shot_delay_counter > 0) {
            self.shot_delay_counter -= 1;
        }
        return None;
        
    }
    pub fn make_trigon(&self) -> (i16, i16, i16, i16, i16, i16) {
        (
            (self.x
             + (self.angle_facing.cos() * self.semi_height))
                .round()
                as i16,
            
            (self.y
             + (self.angle_facing.sin() * self.semi_height))
                .round()
                as i16,
            
            (self.x
             - (self.angle_facing.cos() * self.semi_height)
             + (self.angle_facing.sin() * self.semi_width))
                .round()
                as i16,
            
            (self.y
             - (self.angle_facing.sin() * self.semi_height)
             - (self.angle_facing.cos() * self.semi_width))
                .round()
                as i16,
            
            (self.x
             - (self.angle_facing.cos() * self.semi_height)
             - (self.angle_facing.sin() * self.semi_width))
                .round()
                as i16,
            
            (self.y
             - (self.angle_facing.sin() * self.semi_height)
             + (self.angle_facing.cos() * self.semi_width))
                .round()
                as i16,
        )
    }
    pub fn draw(&self, renderer: &sdl2::render::Renderer) {
        let (x1, y1, x2, y2, x3, y3) = self.make_trigon();
        renderer.filled_trigon(x1, y1, x2, y2, x3, y3, self.color)
            .unwrap();
    }
    pub fn new(starting_x: f32, starting_y: f32) -> RocketShip {
        RocketShip {
            x: starting_x,
            y: starting_y,
            semi_width: ROCKET_DIMENSIONS.0 / 2.0f32,
            semi_height: ROCKET_DIMENSIONS.1 / 2.0f32,
            angle_facing: 0.0f32,
            dx: 0.0f32,
            dy: 0.0f32,
            color: ROCKET_COLOR,
            acceleration: ROCKET_ACCELERATION,
            spin_speed: ROCKET_SPIN_SPEED,
            shot_delay_counter: 0u8,
        }
    }
}
