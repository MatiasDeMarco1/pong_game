use macroquad::prelude::*;
use crate::paddle::Paddle;

pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub vx: f32,
    pub vy: f32,
}

impl Ball {
    pub fn reset(&mut self, direction: f32) {
        self.x = screen_width() / 2.0;
        self.y = screen_height() / 2.0;
        self.vx = 200.0 * direction;
        self.vy = 150.0;
    }

    pub fn update(&mut self, left_paddle: &Paddle, right_paddle: &Paddle) -> Option<bool> {
        self.x += self.vx * get_frame_time();
        self.y += self.vy * get_frame_time();

        if self.y - self.radius <= 0.0 || self.y + self.radius >= screen_height() {
            self.vy = -self.vy;
        }

        if self.x - self.radius <= left_paddle.x + left_paddle.width
            && self.x + self.radius >= left_paddle.x
            && self.y >= left_paddle.y
            && self.y <= left_paddle.y + left_paddle.height
        {
            self.vx = self.vx.abs();
        }

        if self.x + self.radius >= right_paddle.x
            && self.x - self.radius <= right_paddle.x + right_paddle.width
            && self.y >= right_paddle.y
            && self.y <= right_paddle.y + right_paddle.height
        {
            self.vx = -self.vx.abs();
        }

        if self.x - self.radius <= 0.0 {
            self.reset(1.0);
            return Some(false); 
        }

        if self.x + self.radius >= screen_width() {
            self.reset(-1.0);
            return Some(true); 
        }

        None
    }

    pub fn draw(&self) {
        draw_circle(self.x, self.y, self.radius, RED);
    }
}