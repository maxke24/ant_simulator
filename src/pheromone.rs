use macroquad::prelude::*;

use crate::config::Settings;
use crate::point::Point;

#[derive(Copy, Clone, PartialEq)]
pub enum PheromoneType {
    Searching,
    Returning,
}
pub struct Pheromone {
    pub location: Point,
    pub pheromone: PheromoneType,
    pub strength: f32,
    pub radius: f32,
    pub color: Color,
    pub decay: f32,
}

impl Pheromone {
    pub fn new(x: f32, y: f32, pheromone: PheromoneType, settings: &Settings) -> Self {
        Self {
            location: Point { x, y },
            pheromone,
            strength: 1.0,
            radius: settings.pheromone_size,
            color: settings.get_pheromone_color(pheromone),
            decay: settings.decay_rate,
        }
    }

    pub fn update(&mut self) {
        let faded_color = Color::new(self.color.r, self.color.g, self.color.b, self.strength);
        draw_circle(self.location.x, self.location.y, self.radius, faded_color);
        self.strength -= self.decay;
    }
}
