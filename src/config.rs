use macroquad::prelude::Color;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub window_width: i32,
    pub window_height: i32,
    pub ant_count: usize,
    pub ant_speed: f32,
    pub ant_scale: f32,
    pub animation_speed: f32, // seconds per frame (higher = slower)

    pub pheromone_size: f32,
    pub decay_rate: f32,

    // We load these as simple lists of numbers first
    pub background_color: [u8; 3],
    pub ant_color: [u8; 3],
    pub food_color: [u8; 3],
    pub pheromone_color: [u8; 3],
    pub nest_color: [u8; 3],
}

impl Settings {
    // This function tries to read the file and parse it
    pub fn load() -> Self {
        let file_contents = fs::read_to_string("config.yaml").expect("Could not read config.yaml");

        serde_yaml::from_str(&file_contents).expect("Could not parse config.yaml")
    }

    // Helper: Convert [255, 0, 0] -> Color::new(1.0, 0.0, 0.0, 1.0)
    pub fn get_bg_color(&self) -> Color {
        self.u8_to_color(self.background_color)
    }

    pub fn get_ant_color(&self) -> Color {
        self.u8_to_color(self.ant_color)
    }

    pub fn get_food_color(&self) -> Color {
        self.u8_to_color(self.food_color)
    }

    pub fn get_pheromone_color(&self) -> Color {
        self.u8_to_color(self.pheromone_color)
    }

    pub fn get_nest_color(&self) -> Color {
        self.u8_to_color(self.nest_color)
    }

    fn u8_to_color(&self, rgb: [u8; 3]) -> Color {
        Color::from_rgba(rgb[0], rgb[1], rgb[2], 255)
    }
}
