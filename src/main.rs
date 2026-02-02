mod ant;
mod config;
mod food;
mod nest;
mod pheromone;
mod point;
mod spatial_grid;
mod world;

use config::Settings;
use macroquad::prelude::*;
use world::World;

fn window_conf() -> Conf {
    Conf {
        window_title: "Ant Simulator".to_owned(),
        ..Default::default()
    }
}

#[macroquad::main("Ant Simulator")]
async fn main() {
    let settings = Settings::load();
    let screen_w = settings.window_width as f32;
    let screen_h = settings.window_height as f32;
    request_new_screen_size(screen_w, screen_h);

    next_frame().await;
    rand::srand(macroquad::miniquad::date::now() as u64);

    // Load ant spritesheet (white version for color tinting)
    let ant_texture = load_texture("walk_white.png")
        .await
        .expect("Failed to load walk_white.png");
    let mut world = World::new(&settings);
    loop {
        let delta = get_frame_time();

        clear_background(settings.get_bg_color());
        world.update(&settings, delta, &ant_texture);
        next_frame().await;
    }
}
