mod ant;
mod config;

use ant::Ant;
use config::Settings;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Ant Simulator".to_owned(),
        ..Default::default()
    }
}

#[macroquad::main("Ant Simulator")]
async fn main() {
    let settings = Settings::load();
    request_new_screen_size(settings.window_width as f32, settings.window_height as f32);

    next_frame().await;

    rand::srand(macroquad::miniquad::date::now() as u64);
    let mut ants: Vec<Ant> = (0..settings.ant_count)
        .map(|_| Ant::new(settings.window_width as f32, settings.window_height as f32))
        .collect();

    loop {
        clear_background(settings.get_bg_color());
        for ant in ants.iter_mut() {
            draw_ellipse(
                ant.x,
                ant.y,
                ant.w,
                ant.h,
                ant.angle - 90.0,
                settings.get_ant_color(),
            );
            ant.update(settings.window_width as f32, settings.window_height as f32);
        }
        next_frame().await;
    }
}
