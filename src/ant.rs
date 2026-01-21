use macroquad::prelude::*;

// Spritesheet constants
const SPRITE_FRAMES: usize = 8; // frames are arranged vertically
const FRAME_WIDTH: f32 = 202.0;
const FRAME_HEIGHT: f32 = 248.0;

pub struct Ant {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub animation_frame: usize,
    pub animation_timer: f32,
}

impl Ant {
    pub fn new(nest_x: f32, nest_y: f32) -> Self {
        Self {
            x: nest_x,
            y: nest_y,
            angle: rand::gen_range(0.0, 360.0),
            animation_frame: rand::gen_range(0, SPRITE_FRAMES as u32) as usize,
            animation_timer: 0.0,
        }
    }

    pub fn update(
        &mut self,
        screen_w: f32,
        screen_h: f32,
        speed: f32,
        delta: f32,
        animation_speed: f32,
    ) {
        let dx = f32::cos(self.angle.to_radians()) * speed;
        let dy = f32::sin(self.angle.to_radians()) * speed;
        self.x += dx;
        self.y += dy;

        let angle_change: f32 = rand::gen_range(-3.0, 3.0);
        self.angle += angle_change;

        // Update animation
        self.animation_timer += delta;
        if self.animation_timer > animation_speed {
            self.animation_timer = 0.0;
            self.animation_frame = (self.animation_frame + 1) % SPRITE_FRAMES;
        }

        // Screen wrapping
        if self.x < 0.0 {
            self.x = screen_w;
        }
        if self.x > screen_w {
            self.x = 0.0;
        }
        if self.y < 0.0 {
            self.y = screen_h;
        }
        if self.y > screen_h {
            self.y = 0.0;
        }
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
            self.x - dest_size.x / 2.0,
            self.y - dest_size.y / 2.0,
            color,
            DrawTextureParams {
                source: Some(source_rect),
                dest_size: Some(dest_size),
                rotation: (self.angle + 90.0).to_radians(),
                pivot: Some(vec2(self.x, self.y)),
                ..Default::default()
            },
        );
    }
}
