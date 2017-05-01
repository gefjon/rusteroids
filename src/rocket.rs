extern crate sdl2;

use sdl2::rect::{Rect};
use sdl2::pixels::{Color};
use super::globals::*;

pub struct RocketShip {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    dx: i32,
    dy: i32,
    color: Color,
}

impl RocketShip {
    fn make_rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }
    pub fn update(&mut self, ddx: i32, ddy: i32) {
        self.dx += ddx;
        self.dy += ddy;
        self.x += self.dx;
        if (self.x + (self.width as i32)) < 0 {
            self.x += WINDOW_DIMENSIONS.0 as i32;
        }
        self.x %= WINDOW_DIMENSIONS.0 as i32;
        self.y += self.dy;
        if (self.y + (self.height as i32)) < 0 {
            self.y += WINDOW_DIMENSIONS.1 as i32;
        }
        self.y %= WINDOW_DIMENSIONS.1 as i32;
    }
    pub fn draw(&self, mut renderer: &mut sdl2::render::Renderer) {
        renderer.set_draw_color(self.color);
        renderer.fill_rect(self.make_rect())
            .unwrap();
    }
    pub fn new(starting_x: i32, starting_y: i32, width: u32, height: u32) -> RocketShip {
        RocketShip {
            x: starting_x,
            y: starting_y,
            width: width,
            height: height,
            dx: 0,
            dy: 0,
            color: ROCKET_COLOR,
        }
    }
}
