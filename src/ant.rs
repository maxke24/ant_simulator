use macroquad::{prelude::*, rand::RandomRange};

pub struct Ant {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub angle: f32,
}

impl Ant {
    pub fn new(screen_w: f32, screen_h: f32) -> Self {
        Self {
            x: rand::gen_range(10.0, screen_w - 10.0),
            y: rand::gen_range(10.0, screen_h - 10.0),
            w: 2.0,
            h: 5.0,
            angle: rand::gen_range(0.0, 360.0),
        }
    }

    pub fn update(&mut self, screen_w: f32, screen_h: f32) {
        let dx = f32::cos(self.angle.to_radians()) * 1.1;
        let dy = f32::sin(self.angle.to_radians()) * 1.1;
        self.x += dx;
        self.y += dy;
        let angle: f32 = rand::gen_range(-15.0, 15.0);
        self.angle += angle;

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
}
