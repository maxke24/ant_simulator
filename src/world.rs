use crate::Settings;
use crate::ant::Ant;
use crate::food::Food;
use crate::nest::Nest;
use crate::pheromone::{Pheromone, PheromoneType};
use macroquad::prelude::*;

pub struct World {
    pub nest: Nest,
    pub ants: Vec<Ant>,
    pub food: Food,
    pub pheromones: Vec<Pheromone>,
}

impl World {
    pub fn new(settings: &Settings) -> Self {
        let pieces = rand::gen_range(1000, 100000);
        let screen_w = settings.window_width as f32;
        let screen_h = settings.window_height as f32;
        let nest = Nest::new(screen_w, screen_h);
        let nest_x = nest.x;
        let nest_y = nest.y;
        Self {
            nest,
            ants: (0..settings.ant_count)
                .map(|_| Ant::new(nest_x, nest_y))
                .collect(),
            food: Food::new(screen_w, screen_h, pieces),
            pheromones: Vec::new(),
        }
    }

    pub fn update(&mut self, settings: &Settings, delta: f32, ant_texture: &Texture2D) {
        draw_circle(
            self.food.x,
            self.food.y,
            self.food.radius(),
            settings.get_food_color(),
        );

        for ant in self.ants.iter_mut() {
            if let Some(pheromone) = ant.update(delta, settings) {
                self.pheromones.push(pheromone);
            }
            ant.draw(ant_texture, settings.get_ant_color(), settings.ant_scale);
        }
        for pheromone in self.pheromones.iter_mut() {
            pheromone.update(
                settings.pheromone_size,
                settings.get_pheromone_color(),
                settings.decay_rate,
            );
        }

        self.pheromones.retain(|p| p.strength > 0.0);

        draw_circle(
            self.nest.x,
            self.nest.y,
            self.nest.radius,
            settings.get_nest_color(),
        );
    }
}
