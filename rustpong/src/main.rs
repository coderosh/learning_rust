use raylib::prelude::*;

pub mod ball;
pub mod player;

const SCREEN_HEIGHT: i32 = 800;
const SCREEN_WIDTH: i32 = 1280;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Pong")
        .build();

    rl.set_target_fps(60);

    let mut ball = ball::Ball::new(15, 4, 4, SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);

    let paddle_height = 130;
    let paddle_width = 15;
    let paddle_gap = 10;
    let player_speed = 8;

    let mut player_right = player::Player::new(
        paddle_height,
        paddle_width,
        player_speed,
        SCREEN_WIDTH - paddle_width - paddle_gap,
        SCREEN_HEIGHT / 2 - paddle_height / 2,
    );

    let mut player_left = player::Player::new(
        paddle_height,
        paddle_width,
        player_speed,
        paddle_gap,
        SCREEN_HEIGHT / 2 - paddle_height / 2,
    );

    let background_color = Color::new(62, 50, 60, 255);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(background_color);

        ball.update();
        ball.constrain(SCREEN_HEIGHT);
        ball.draw(&mut d);

        if ball.check_left_wall_collision() {
            player_right.score += 1;

            ball.reset();
            ball.increase_speed();
        }

        if ball.check_right_wall_collision(SCREEN_WIDTH) {
            player_left.score += 1;
            ball.reset();
            ball.increase_speed();
        }

        if player_left.check_collision_with_ball(&ball)
            || player_right.check_collision_with_ball(&ball)
        {
            if ball.speed_x < 0 {
                ball.x = paddle_width + ball.radius + paddle_gap;
            } else {
                ball.x = SCREEN_WIDTH - paddle_width - paddle_gap - ball.radius;
            }

            ball.speed_x *= -1;
        }

        player_right.update(&mut d, true);
        player_right.constrain(SCREEN_HEIGHT);
        player_right.draw(&mut d);

        player_left.update(&mut d, false);
        player_left.constrain(SCREEN_HEIGHT);
        player_left.draw(&mut d);

        let player_left_score_text = format!("{}", player_left.score);
        let playe_left_text_width = d.measure_text(player_left_score_text.as_str(), 100);

        let player_right_score_text = format!("{}", player_right.score);
        let playe_right_text_width = d.measure_text(player_right_score_text.as_str(), 100);

        d.draw_line(
            SCREEN_WIDTH / 2,
            0,
            SCREEN_WIDTH / 2,
            SCREEN_HEIGHT,
            Color::WHITE,
        );

        d.draw_text(
            player_left_score_text.as_str(),
            SCREEN_WIDTH / 4 - playe_left_text_width / 2,
            20,
            100,
            Color::WHITE,
        );

        d.draw_text(
            player_right_score_text.as_str(),
            SCREEN_WIDTH - SCREEN_WIDTH / 4 - playe_right_text_width / 2,
            20,
            100,
            Color::WHITE,
        );
    }
}
