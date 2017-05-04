extern crate sdl2;

use sdl2::pixels::{Color};
use super::globals::*;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::{Scancode};
use super::shooting::Shot;

pub struct RocketShip {
    x: f32,
    y: f32,
    semi_width: f32,             // the distance from center to edges
    semi_height: f32,            // the distance from center to point
    angle_facing: f32,           // angle in radians, starting leftward & increasing cw
    dx: f32,
    dy: f32,
    color: Color,
    acceleration: f32,           // how fast it gets to accelerate every frame
    spin_speed: f32,             // how fast it gets to rotate every frame
    shot_delay: u8,              // how many frames between shooting
    shot_delay_counter: u8,      // a counter for the delay between firing shots
}

impl RocketShip {
    pub fn update(&mut self, keyboard_state: &sdl2::keyboard::KeyboardState) -> Option<Shot> {
        // if we fired a shot, returns Some(shot)
        // otherwise, returns None
        
        if keyboard_state.is_scancode_pressed(Scancode::W) {
            // if 'w' is pressed, accelerate in the direction we're facing
            self.dx += self.angle_facing.cos() * self.acceleration;
            self.dy += self.angle_facing.sin() * self.acceleration;
        }
        
        if keyboard_state.is_scancode_pressed(Scancode::A) {
            // if 'a' is pressed, rotate ccw
            self.angle_facing -= self.spin_speed;
        }
        
        if keyboard_state.is_scancode_pressed(Scancode::D) {
            // if 's' is pressed, rotate cw
            self.angle_facing += self.spin_speed;
        }
        
        self.x += self.dx;
        self.y += self.dy;

        // if the ship is offscreen, wrap it around to the other side
        // we do all this crap with ifs and subtracting instead of just % for a cleaner transition
        // transition once the whole ship is off the side, not just the center
        if (self.x + self.semi_height) < 0.0f32 {
            self.x += (WINDOW_DIMENSIONS.0 as f32) + (self.semi_height * 2.0f32);
        } else if (self.x - self.semi_height) > (WINDOW_DIMENSIONS.0 as f32) {
            self.x -= (WINDOW_DIMENSIONS.0 as f32) + (self.semi_height * 2.0f32);
        }
        
        if (self.y + self.semi_height) < 0.0f32 {
            self.y += (WINDOW_DIMENSIONS.1 as f32) + (self.semi_height * 2.0f32);
        } else if (self.y - self.semi_height) > (WINDOW_DIMENSIONS.1 as f32) {
            self.y -= (WINDOW_DIMENSIONS.1 as f32) + (self.semi_height * 2.0f32);
        }

        if (
            (self.shot_delay_counter <= 0) &&
                keyboard_state.is_scancode_pressed(Scancode::Space)
        ) {
            self.shot_delay_counter += self.shot_delay;
            return Some(
                Shot::new(
                    self.x + (self.angle_facing.cos() * self.semi_height), // shot.x is the x-pos of the tip of the ship
                    self.y + (self.angle_facing.sin() * self.semi_height), // shot.y is the y-pos of the tip of the ship
                    self.angle_facing)
            );
        } else if (self.shot_delay_counter > 0) {
            self.shot_delay_counter -= 1;
        }
        return None;
        
    }
    pub fn make_trigon(&self) -> (i16, i16, i16, i16, i16, i16) {
        /*
        the ship in its current form is drawn using a filled trigon
        the trigon is three (x, y) pairs
        this function generates those (x, y) pairs
        x1, y1 is the tip of the ship
        x2, y2 is one of the back wings
        x3, y3 is the other
         */
        
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
        // this is a very simple draw function
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
            shot_delay: ROCKET_SHOT_DELAY,
            shot_delay_counter: 0u8,
        }
    }
}
