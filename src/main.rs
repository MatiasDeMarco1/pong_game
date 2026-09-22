mod paddle;
mod ball;

use macroquad::prelude::*;
use paddle::Paddle;
use ball::Ball;

#[macroquad::main("Pong")]
async fn main() {
    let paddle_width = 5.0;
    let paddle_height = 50.0;
    let margin = 20.0;
    let paddle_speed = 300.0;

    let mut left_paddle = Paddle { x: margin, y: screen_height()/2.0 - paddle_height/2.0, width: paddle_width, height: paddle_height };
    let mut right_paddle = Paddle { x: screen_width()-margin-paddle_width, y: screen_height()/2.0 - paddle_height/2.0, width: paddle_width, height: paddle_height };
    let mut ball = Ball { x: screen_width()/2.0, y: screen_height()/2.0, radius: 8.0, vx: 200.0, vy: 150.0 };

    let mut left_score = 0;
    let mut right_score = 0;

    loop {
        clear_background(BLACK);

        let score_text = format!("{} - {}", left_score, right_score);
        let text_size = measure_text(&score_text, None, 40, 1.0);
        draw_text(&score_text, screen_width()/2.0 - text_size.width/2.0, 40.0, 40.0, WHITE);

        left_paddle.update(KeyCode::W, KeyCode::S, paddle_speed);
        right_paddle.update(KeyCode::Up, KeyCode::Down, paddle_speed);

        if let Some(left_scored) = ball.update(&left_paddle, &right_paddle) {
            if left_scored { left_score += 1; } else { right_score += 1; }
        }

        left_paddle.draw();
        right_paddle.draw();
        ball.draw();

        next_frame().await
    }
}