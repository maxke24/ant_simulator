use macroquad::prelude::*;

pub struct Nest {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub food_count: f32,
}

impl Nest {
    pub fn new(screen_w: f32, screen_h: f32) -> Self {
        Self {
            x: rand::gen_range(30.0, screen_w - 30.0),
            y: rand::gen_range(30.0, screen_h - 30.0),
            radius: 20.0,
            food_count: 0.0,
        }
    }
}
