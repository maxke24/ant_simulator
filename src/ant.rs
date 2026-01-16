use macroquad::prelude::*;

pub struct Ant {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub angle: f32,
}

impl Ant {
    pub fn new(screen_w: f32, screen_h: f32) -> Self {
        Self {
            x: rand::gen_range(10.0, screen_w - 10.0),
            y: rand::gen_range(10.0, screen_h - 10.0),
            w: 2.0,
            h: 5.0,
            angle: rand::gen_range(0.0, 360.0),
        }
    }
}
