mod ant;
mod config;
mod nest;

use ant::Ant;
use config::Settings;
use macroquad::prelude::*;
use nest::Nest;

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

    let actual_w = screen_width();
    let actual_h = screen_height();

    // Load ant spritesheet (white version for color tinting)
    let ant_texture = load_texture("walk_white.png").await.expect("Failed to load walk_white.png");

    let nest = Nest::new(actual_w, actual_h);

    let mut ants: Vec<Ant> = (0..settings.ant_count)
        .map(|_| Ant::new(nest.x, nest.y))
        .collect();

    loop {
        let delta = get_frame_time();

        clear_background(settings.get_bg_color());

        for ant in ants.iter_mut() {
            ant.update(screen_width(), screen_height(), settings.ant_speed, delta, settings.animation_speed);
            ant.draw(&ant_texture, settings.get_ant_color(), settings.ant_scale);
        }

        draw_circle(nest.x, nest.y, nest.radius, settings.get_nest_color());
        next_frame().await;
    }
}
