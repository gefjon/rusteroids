extern crate sdl2;

use sdl2::event::{Event};
use sdl2::gfx::framerate::FPSManager;
use sdl2::keyboard::{Scancode};

mod rocket;
mod globals;
mod asteroid;

use globals::*;
use rocket::RocketShip;
use asteroid::Asteroid;

struct World<'a> {
    player: RocketShip,
    renderer: sdl2::render::Renderer<'a>,
    sdl_context: sdl2::Sdl,
    asteroids: Vec<Asteroid>,
    framerate_controller: FPSManager,
}

fn main() {
    let sdl2_context = sdl2::init().unwrap();
    let video_context = sdl2_context.video().unwrap();

    let renderer = create_window(&video_context).renderer().build().unwrap();

    let asteroids: Vec<Asteroid> = vec![];

    let mut world = World {
        player: RocketShip::new(
            ((WINDOW_DIMENSIONS.0 / 2) as f32),
            ((WINDOW_DIMENSIONS.1 / 2) as f32),
            128.0f32,
            128.0f32
        ),
        renderer: renderer,
        asteroids: asteroids,
        sdl_context: sdl2_context,
        framerate_controller: FPSManager::new(),
    };


    world.framerate_controller.set_framerate(MAX_FRAMERATE)
        .unwrap();

    'main_loop : loop {
        if !update(&mut world) {
            break 'main_loop;
        } else {
            draw(&mut world);
        }
    }
}

fn create_window(video_context: &sdl2::VideoSubsystem) -> sdl2::video::Window {
    video_context.window(WINDOW_TITLE,
                         WINDOW_DIMENSIONS.0,
                         WINDOW_DIMENSIONS.1)
        .position_centered()
        .opengl()
        .build()
        .unwrap()
}

fn update(mut world: &mut World) -> bool {
    let mut ddx: f32 = 0.0f32;
    let mut ddy: f32 = 0.0f32;
    let mut event_pump = world.sdl_context.event_pump().unwrap();
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit{..} => {return false;},
            Event::KeyDown{scancode: Some(Scancode::Escape), ..} => {return false;},
            _ => continue,
        }
    }
    let keyboard_state = event_pump.keyboard_state();
    world.player.update(&keyboard_state);
    let mut new_asteroids: Vec<Asteroid> = Vec::new();
    for asteroid in world.asteroids.iter() {
        new_asteroids.push(asteroid.update());
    }
    world.asteroids = new_asteroids;
    return true;
}

fn draw_background(mut renderer: &mut sdl2::render::Renderer) {
    renderer.set_draw_color(BACKGROUND_COLOR);
    renderer.fill_rect(None).unwrap();
}

fn draw(mut world: &mut World) {
    draw_background(&mut world.renderer);
    world.player.draw(&world.renderer);
    for asteroid in world.asteroids.iter() {
        asteroid.draw(&world.renderer);
    }
    world.renderer.present();
    world.framerate_controller.delay();
}
