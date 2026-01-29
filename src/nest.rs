use macroquad::prelude::*;

use crate::point::Point;

#[derive(Copy, Clone)]
pub struct Nest {
    pub location: Point,
    pub radius: f32,
    pub food_count: u8,
}

impl Nest {
    pub fn new(_screen_w: f32, _screen_h: f32) -> Self {
        // Use actual screen dimensions from macroquad after window is ready
        let screen_w = macroquad::prelude::screen_width();
        let screen_h = macroquad::prelude::screen_height();
        let x = rand::gen_range(30.0, screen_w - 30.0);
        let y = rand::gen_range(30.0, screen_h - 30.0);
        Self {
            location: Point { x, y },
            radius: 20.0,
            food_count: 0,
        }
    }
}
