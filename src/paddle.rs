use macroquad::prelude::*;

pub struct Paddle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    pub fn update(&mut self, up_key: KeyCode, down_key: KeyCode, speed: f32) {
        if is_key_down(up_key) {
            self.y -= speed * get_frame_time();
        }
        if is_key_down(down_key) {
            self.y += speed * get_frame_time();
        }
        self.y = self.y.clamp(0.0, screen_height() - self.height);
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, self.width, self.height, WHITE);
    }
}