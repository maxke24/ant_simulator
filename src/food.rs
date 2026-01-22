use macroquad::prelude::*;

pub struct Food {
    pub x: f32,
    pub y: f32,
    pub pieces_remaining: u32,
    pub max_pieces: u32,
}

impl Food {
    pub fn new(screen_w: f32, screen_h: f32, pieces: u32) -> Self {
        Self {
            x: rand::gen_range(30.0, screen_w - 30.0),
            y: rand::gen_range(30.0, screen_h - 30.0),
            max_pieces: pieces,
            pieces_remaining: pieces,
        }
    }

    pub fn radius(&self) -> f32 {
        let ratio = self.pieces_remaining as f32 / self.max_pieces as f32;
        let min_radius = 5.0;
        min_radius + (15.0 * ratio) // ranges from 5 to 20
    }
}
