use crate::Settings;
use crate::ant::Ant;
use crate::food::Food;
use crate::nest::Nest;
use crate::pheromone::Pheromone;
use crate::spatial_grid::SpatialGrid;
use macroquad::prelude::*;

pub struct World {
    pub nest: Nest,
    pub ants: Vec<Ant>,
    pub food: Food,
    pub pheromones: Vec<Pheromone>,
    pub grid: SpatialGrid,
}

impl World {
    pub fn new(settings: &Settings) -> Self {
        let pieces = rand::gen_range(1000, 100000);
        let screen_w = settings.window_width as f32;
        let screen_h = settings.window_height as f32;
        let nest = Nest::new(screen_w, screen_h);
        Self {
            nest,
            ants: (0..settings.ant_count)
                .map(|_| Ant::new(nest, settings))
                .collect(),
            food: Food::new(screen_w, screen_h, pieces),
            pheromones: Vec::new(),
            grid: SpatialGrid::new(40.0),
        }
    }

    pub fn update(&mut self, settings: &Settings, delta: f32, ant_texture: &Texture2D) {
        draw_circle(
            self.food.location.x,
            self.food.location.y,
            self.food.radius(),
            settings.get_food_color(),
        );

        for ant in self.ants.iter_mut() {
            if let Some(pheromone) = ant.update(
                delta,
                settings,
                &mut self.food,
                &self.pheromones,
                &mut self.grid,
            ) {
                self.pheromones.push(pheromone);
            }
            ant.draw(ant_texture, settings.get_ant_color(), settings.ant_scale);
        }
        for pheromone in self.pheromones.iter_mut() {
            pheromone.update();
        }

        self.pheromones.retain(|p| p.strength > 0.0);

        draw_circle(
            self.nest.location.x,
            self.nest.location.y,
            self.nest.radius,
            settings.get_nest_color(),
        );
        self.grid.rebuild(&self.pheromones);
    }
}
