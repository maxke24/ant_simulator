use std::collections::HashMap;

use crate::{ant::Ant, pheromone::Pheromone};

#[derive(Clone)]
pub struct SpatialGrid {
    cell_size: f32,
    cells: HashMap<(i32, i32), Vec<usize>>,
}

impl SpatialGrid {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            cells: HashMap::new(),
        }
    }

    pub fn world_to_cell(&self, x: f32, y: f32) -> (i32, i32) {
        let cell_x = f32::floor(x / self.cell_size) as i32;
        let cell_y = f32::floor(y / self.cell_size) as i32;
        (cell_x, cell_y)
    }

    pub fn rebuild(&mut self, pheromones: &Vec<Pheromone>) {
        self.cells.clear();

        for i in 0..pheromones.len() {
            let pheromone = &pheromones[i];
            let cell = self.world_to_cell(pheromone.location.x, pheromone.location.y);
            self.cells.entry(cell).or_default().push(i)
        }
    }

    pub fn query<'a>(&self, ant: &Ant, pheromones: &'a Vec<Pheromone>) -> Vec<&'a Pheromone> {
        let mut result = Vec::new();
        let radius_squared = ant.sensor_range * ant.sensor_range;

        let min_x = f32::floor((ant.location.x - ant.sensor_range) / self.cell_size) as i32;
        let max_x = f32::floor((ant.location.x + ant.sensor_range) / self.cell_size) as i32;
        let min_y = f32::floor((ant.location.y - ant.sensor_range) / self.cell_size) as i32;
        let max_y = f32::floor((ant.location.y + ant.sensor_range) / self.cell_size) as i32;

        for cell_x in min_x..=max_x {
            for cell_y in min_y..=max_y {
                let cell_key = (cell_x, cell_y);
                if !self.cells.contains_key(&cell_key) {
                    continue;
                }

                if let Some(pheromone_indices) = self.cells.get(&cell_key) {
                    for &pheromone_index in pheromone_indices {
                        let pheromone = &pheromones[pheromone_index];

                        let dx = pheromone.location.x - ant.location.x;
                        let dy = pheromone.location.y - ant.location.y;

                        let dist_squared = dx * dx + dy * dy;

                        if dist_squared <= radius_squared {
                            result.push(pheromone);
                        }
                    }
                }
            }
        }
        return result;
    }
}
