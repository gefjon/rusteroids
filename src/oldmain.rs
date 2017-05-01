extern crate sdl2;

use sdl2::event::{Event};
use sdl2::rect::{Rect};

const window_width: u32 = 640;
const window_height: u32 = 480;

static mut square_vel: (i32, i32) = (1, 1);
static square_dimensions: (u32, u32) = (64, 64);
static mut square_pos: (i32, i32) = (((window_width  - square_dimensions.0) / 2) as i32, ((window_height - square_dimensions.1) / 2) as i32);

static light_blue: sdl2::pixels::Color = sdl2::pixels::Color::RGB(101, 208, 246);
static black: sdl2::pixels::Color = sdl2::pixels::Color::RGB(0, 0, 0);


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_context = sdl_context.video().unwrap();

    let mut window = match video_context.window("Arthur's Excellent Test Window", window_width, window_height).position_centered().opengl().build() {
        Ok(window) => window,
        Err(err) => panic!("Failed to create window with error {}", err),
    };

    let mut renderer = match window.renderer().build() {
        Ok(renderer) => renderer,
        Err(err) => panic!("Failed to create renderer with error {}", err),
    };

    'main_loop : loop {
        let mut event_pump = sdl_context.event_pump().unwrap();

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit{..} => break 'main_loop,
                _ => continue,
            }
        }
        move_square();
        draw(&mut renderer);
    }
}

fn move_square() {
    if (square_pos.0 < 0) || ((square_pos.0 + square_dimensions.0 as i32) > window_width as i32) {
        square_vel.0 *= -1;
    }
    if (square_pos.1 < 0) || ((square_pos.0 + square_dimensions.0 as i32) > window_width as i32) {
        square_vel.1 *= -1;
    }
    square_pos.0 += square_vel.0;
    square_pos.1 += square_vel.1;
}

fn draw_square(renderer: &mut sdl2::render::Renderer) {
    renderer.set_draw_color(black);
    let rect = Rect::new(square_pos.0, square_pos.1, square_dimensions.0, square_dimensions.1);
    renderer.fill_rect(rect)
        .expect("Failed to draw the bouncing square");
}

fn fill_background(renderer: &mut sdl2::render::Renderer) {
    renderer.set_draw_color(light_blue);
    renderer.fill_rect(None)
        .expect("Failed to fill the background");
}

fn draw(renderer: &mut sdl2::render::Renderer) {
    fill_background(&mut renderer);
    draw_square(&mut renderer);
    renderer.present();
}
