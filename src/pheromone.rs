use macroquad::prelude::*;

#[derive(Copy, Clone)]
pub enum PheromoneType {
    Searching,
    Returning,
}
pub struct Pheromone {
    pub x: f32,
    pub y: f32,
    pub pheromone: PheromoneType,
    pub strength: f32,
}

impl Pheromone {
    pub fn new(x: f32, y: f32, pheromone: PheromoneType) -> Self {
        Self {
            x,
            y,
            pheromone,
            strength: 1.0,
        }
    }

    pub fn update(&mut self, size: f32, color: Color, decay_rate: f32) {
        let faded_color = Color::new(color.r, color.g, color.b, self.strength);
        draw_circle(self.x, self.y, size, faded_color);
        self.strength -= decay_rate;
    }
}
