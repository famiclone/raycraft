use crate::traits::{Drawable, Updatable, DrawableUpdatable, Movable, Person};

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub struct Player {
    speed: f32,
    sprite: Rect,
    vel_x: f32,
    vel_y: f32,
    rotation: f32,
}

impl Player {
    pub fn new(pos_x: f32, pos_y: f32) -> Self {
        Self {
            speed: 250.0,
            sprite: Rect::new(pos_x.round() as i32, pos_y.round() as i32, 16, 16),
            vel_x: 0.0,
            vel_y: 0.0,
            rotation: 0.0,
        }
    }

    pub fn get_x(&self) -> i32 {
        self.sprite.x
    }

    pub fn get_y(&self) -> i32 {
        self.sprite.y
    }
}

impl Drawable for Player {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        // apply rotation
        canvas.fill_rect(self.sprite).unwrap();
        canvas.draw_line(
            sdl2::rect::Point::new(self.sprite.x, self.sprite.y + 8),
            sdl2::rect::Point::new(self.sprite.x + 16 + 32, self.sprite.y + 8),
        ).unwrap();
    }
}

impl Updatable for Player {
    fn update(&mut self, dt: f32) {
        self.sprite.x = self.sprite.x + (self.vel_x * dt * self.speed) as i32;
        self.sprite.y = self.sprite.y + (self.vel_y * dt * self.speed) as i32;

        // rotation
        if self.vel_x != 0.0 || self.vel_y != 0.0 {
            self.rotation = self.vel_y.atan2(self.vel_x);
        }
    }
}

impl Movable for Player {
    fn move_forward(&mut self) {
        self.vel_y = -1.0
    }

    fn move_backward(&mut self) {
        self.vel_y = 1.0
    }

    fn move_left(&mut self) {
        self.vel_x = -1.0;
    }

    fn move_right(&mut self) {
        self.vel_x = 1.0;
    }

    fn stop_horizontal (&mut self) {
        self.vel_x = 0.0;
    }

    fn stop_vertical (&mut self) {
        self.vel_y = 0.0;
    }

    fn set_angle(&mut self, angle: f32) {
        self.rotation = angle;
    }

}

impl DrawableUpdatable for Player {}
impl Person for Player {}
