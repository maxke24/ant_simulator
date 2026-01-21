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

    let mut nest = Nest::new(actual_w, actual_h);

    let mut ants: Vec<Ant> = (0..settings.ant_count)
        .map(|_| Ant::new(nest.x, nest.y))
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
            ant.update(screen_width(), screen_height());
        }
        draw_circle(nest.x, nest.y, nest.radius, settings.get_nest_color());
        next_frame().await;
    }
}
