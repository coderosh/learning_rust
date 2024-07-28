use raylib::prelude::*;

pub struct Ball {
    i_x: i32,
    i_y: i32,
    pub radius: i32,
    pub speed_x: i32,
    pub speed_y: i32,
    pub x: i32,
    pub y: i32,
}

impl Ball {
    pub fn new(radius: i32, speed_x: i32, speed_y: i32, x: i32, y: i32) -> Self {
        Ball {
            radius,
            speed_x,
            speed_y,
            x,
            y,
            i_x: x,
            i_y: y,
        }
    }

    pub fn draw<'a>(self: &Self, handle: &mut RaylibDrawHandle<'a>) {
        handle.draw_circle(self.x, self.y, self.radius as f32, Color::WHITE);
    }

    pub fn update(self: &mut Self) {
        self.x += self.speed_x;
        self.y += self.speed_y;
    }

    pub fn check_left_wall_collision(self: &Self) -> bool {
        self.x - self.radius <= 0
    }

    pub fn check_right_wall_collision(self: &Self, width: i32) -> bool {
        self.x + self.radius >= width
    }

    pub fn reset(self: &mut Self) {
        self.x = self.i_x;
        self.y = self.i_y;
    }

    pub fn increase_speed(self: &mut Self) {
        if self.speed_x <= 9 {
            let inr_f = 1;

            if self.speed_x > 0 {
                self.speed_x += inr_f;
                self.speed_y += inr_f;
            } else {
                self.speed_x -= inr_f;
                self.speed_y -= inr_f;
            }
        }
    }

    pub fn constrain(self: &mut Self, height: i32) {
        if self.y + self.radius >= height || self.y - self.radius <= 0 {
            self.speed_y *= -1;
        }
    }
}
