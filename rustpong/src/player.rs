use ffi::Rectangle;
use raylib::prelude::*;

use crate::ball::Ball;

pub struct Player {
    pub height: i32,
    pub width: i32,
    pub speed: i32,
    pub x: i32,
    pub y: i32,

    pub score: i32,

    prev_pressed_up_count: i32,
    prev_pressed_down_count: i32,
}

const PADDLE_COLOR: Color = Color::new(150, 126, 118, 255);

impl Player {
    pub fn new(height: i32, width: i32, speed: i32, x: i32, y: i32) -> Self {
        Player {
            height,
            width,
            speed,
            x,
            y,
            score: 0,

            prev_pressed_up_count: 0,
            prev_pressed_down_count: 0,
        }
    }
    pub fn draw<'a>(self: &Self, handle: &mut RaylibDrawHandle<'a>) {
        handle.draw_rectangle_rounded(
            Rectangle {
                x: self.x as f32,
                y: self.y as f32,
                width: self.width as f32,
                height: self.height as f32,
            },
            10.0,
            0,
            PADDLE_COLOR,
        )
    }

    pub fn update<'a>(self: &mut Self, handle: &mut RaylibDrawHandle<'a>, use_arrow: bool) {
        let mut up_key = consts::KeyboardKey::KEY_UP;
        let mut down_key = consts::KeyboardKey::KEY_DOWN;

        if !use_arrow {
            up_key = consts::KeyboardKey::KEY_W;
            down_key = consts::KeyboardKey::KEY_S;
        }

        if handle.is_key_down(up_key) {
            self.y -= self.speed + self.prev_pressed_up_count * 1;

            self.prev_pressed_up_count += 1;
        } else {
            self.prev_pressed_up_count = 0;
        }

        if handle.is_key_down(down_key) {
            self.y += self.speed + self.prev_pressed_down_count * 1;

            self.prev_pressed_down_count += 1;
        } else {
            self.prev_pressed_down_count = 0;
        }
    }

    pub fn constrain(self: &mut Self, height: i32) {
        if self.y <= 0 {
            self.y = 0;
        }

        if self.y + self.height >= height {
            self.y = height - self.height;
        }
    }

    pub fn check_collision_with_ball(self: &Self, ball: &Ball) -> bool {
        let rec_center_x = self.x + self.width / 2;
        let rec_center_y = self.y + self.height / 2;

        let dx = (ball.x - rec_center_x).abs();
        let dy = (ball.y - rec_center_y).abs();

        if dx > (self.width / 2 + ball.radius) {
            return false;
        }

        if dy > (self.height / 2 + ball.radius) {
            return false;
        }

        if dx <= (self.width / 2) {
            return true;
        }

        if dy <= (self.height / 2) {
            return true;
        }

        let corner_dist_sq = (dx - self.width / 2) * (dx - self.width / 2)
            + (dy - self.height / 2) * (dy - self.height / 2);

        return corner_dist_sq <= (ball.radius * ball.radius);
    }
}
