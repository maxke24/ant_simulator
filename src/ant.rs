use crate::config::Settings;
use crate::food::Food;
use crate::nest::Nest;
use crate::pheromone::Pheromone;
use crate::pheromone::PheromoneType;
use crate::point::Point;
use macroquad::prelude::*;

// Spritesheet constants
const SPRITE_FRAMES: usize = 8; // frames are arranged vertically
const FRAME_WIDTH: f32 = 202.0;
const FRAME_HEIGHT: f32 = 248.0;

pub struct Ant {
    pub location: Point,
    pub angle: f32,
    pub animation_frame: usize,
    pub animation_timer: f32,
    pub pheromone_timer: f32,
    pub state: PheromoneType,
    pub drop_frequency: f32,
    pub nest: Nest,
}

impl Ant {
    pub fn new(nest: Nest) -> Self {
        Self {
            location: nest.location,
            angle: rand::gen_range(0.0, 360.0),
            animation_frame: rand::gen_range(0, SPRITE_FRAMES as u32) as usize,
            animation_timer: 0.0,
            pheromone_timer: 0.0,
            state: PheromoneType::Searching,
            drop_frequency: rand::gen_range(0.8, 2.2),
            nest,
        }
    }

    pub fn update(&mut self, delta: f32, settings: &Settings, fd: &mut Food) -> Option<Pheromone> {
        let dx = f32::cos(self.angle.to_radians()) * settings.ant_speed;
        let dy = f32::sin(self.angle.to_radians()) * settings.ant_speed;
        self.location.x += dx;
        self.location.y += dy;

        let angle_change: f32 = rand::gen_range(-3.0, 3.0);
        self.angle += angle_change;

        // Update animation
        self.animation_timer += delta;
        if self.animation_timer > settings.animation_speed {
            self.animation_timer = 0.0;
            self.animation_frame = (self.animation_frame + 1) % SPRITE_FRAMES;
        }

        // Screen wrapping
        if self.location.x < 0.0 {
            self.location.x = settings.window_width as f32;
        }
        if self.location.x > settings.window_width as f32 {
            self.location.x = 0.0;
        }
        if self.location.y < 0.0 {
            self.location.y = settings.window_height as f32;
        }
        if self.location.y > settings.window_height as f32 {
            self.location.y = 0.0;
        }

        if self.state == PheromoneType::Searching && self.distance_to(fd.location) < fd.radius() {
            self.state = PheromoneType::Returning;
            fd.pieces_remaining -= 1;
        } else if self.state == PheromoneType::Returning
            && self.distance_to(self.nest.location) < self.nest.radius
        {
            self.state = PheromoneType::Searching;
            self.nest.food_count += 1;
        }

        self.pheromone_timer += delta;
        if self.pheromone_timer > self.drop_frequency {
            self.pheromone_timer = 0.0;
            return Some(Pheromone::new(
                self.location.x,
                self.location.y,
                self.state,
                settings,
            ));
        }

        None
    }

    fn distance_to(&self, target: Point) -> f32 {
        f32::abs(self.location.x - target.x) + f32::abs(self.location.y - target.y)
    }

    pub fn draw(&self, texture: &Texture2D, color: Color, scale: f32) {
        let source_rect = Rect::new(
            0.0, // Use first column of spritesheet
            (self.animation_frame as f32) * FRAME_HEIGHT,
            FRAME_WIDTH,
            FRAME_HEIGHT,
        );

        let dest_size = vec2(FRAME_WIDTH * scale, FRAME_HEIGHT * scale);

        draw_texture_ex(
            texture,
            self.location.x - dest_size.x / 2.0,
            self.location.y - dest_size.y / 2.0,
            color,
            DrawTextureParams {
                source: Some(source_rect),
                dest_size: Some(dest_size),
                rotation: (self.angle + 90.0).to_radians(),
                pivot: Some(vec2(self.location.x, self.location.y)),
                ..Default::default()
            },
        );
    }
}
