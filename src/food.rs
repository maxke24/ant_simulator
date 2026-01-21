use macroquad::prelude::*;

pub struct Food {
    pub x: f32,
    pub y: f32,
    pub strength: f32,
}

impl Food {
    pub fn new(screen_w: f32, screen_h: f32) -> Self {
        Self {
            x: rand::gen_range(10.0, screen_w - 10.0),
            y: rand::gen_range(10.0, screen_h - 10.0),
            strength: 1.0,
        }
    }
}
