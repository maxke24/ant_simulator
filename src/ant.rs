use macroquad::prelude::*;

pub struct Ant {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub angle: f32,
}

impl Ant {
    pub fn new() -> Self {
        Self {
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
            w: 2.0,
            h: 5.0,
            angle: rand::gen_range(0.0, 360.0),
        }
    }
}
