use macroquad::prelude::*;

pub struct Nest {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub food_count: f32,
}

impl Nest {
    pub fn new(_screen_w: f32, _screen_h: f32) -> Self {
        // Use actual screen dimensions from macroquad after window is ready
        let screen_w = macroquad::prelude::screen_width();
        let screen_h = macroquad::prelude::screen_height();
        Self {
            x: rand::gen_range(30.0, screen_w - 30.0),
            y: rand::gen_range(30.0, screen_h - 30.0),
            radius: 20.0,
            food_count: 0.0,
        }
    }
}
