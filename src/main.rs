use macroquad::prelude::*;
mod ant;

use ant::Ant;

#[macroquad::main("Ant Simulator")]
async fn main() {
    next_frame().await;
    rand::srand(macroquad::miniquad::date::now() as u64);
    let ant = Ant::new();
    println!("{}", ant.angle);
    loop{
        clear_background(BLACK);
        draw_ellipse(ant.x, ant.y, ant.w, ant.h, ant.angle, WHITE);
        next_frame().await;
    }
}
