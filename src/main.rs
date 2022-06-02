use std::collections::HashMap;

use learn_sdl2::{game_window::GameWindow, resource::TextureManager};
use sdl2::{event::Event, image, keyboard::Keycode};

fn main() {
    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();
    sdl2::image::init(image::InitFlag::PNG).unwrap();

    let window = video_subsystem
        .window("learn_sd2", 800, 600)
        .position_centered()
        .build()
        .unwrap();

        let mut h = HashMap::new();
        h.insert(String::from("leonel"), 10);
        h.get("leonel");

    let canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut game_window = GameWindow { canvas };
    let mut texture_manager = TextureManager::new(&texture_creator);

    let texture = texture_manager.load("res/gfx/ground_grass_1.png").unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        game_window.clear();
        game_window.render_texture(&texture);
        game_window.present();
    }
}
