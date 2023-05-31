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
}

impl Player {
    pub fn new(pos_x: i32, pos_y: i32) -> Self {
        Self {
            speed: 0.11,
            sprite: Rect::new(pos_x, pos_y, 32, 32),
            vel_x: 1.0,
            vel_y: 0.0,
        }
    }
}

impl Drawable for Player {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.fill_rect(self.sprite).unwrap();
    }
}

impl Updatable for Player {
    fn update(&mut self, dt: f32) {
        self.sprite.x = (self.vel_x * self.sprite.x() as f32 + self.speed * dt).round() as i32;
        self.sprite.y = (self.vel_y * self.sprite.y() as f32 + self.speed * dt).round() as i32;
    }
}

impl Movable for Player {
    fn move_forward(&mut self) {
        self.vel_x = 0.0;
        self.vel_y = -1.0;
    }

    fn move_backward(&mut self) {
        self.vel_x = 0.0;
        self.vel_y = 1.0;
    }

    fn move_left(&mut self) {
        self.vel_x = -1.0;
        self.vel_y = 0.0;
    }

    fn move_right(&mut self) {
        self.vel_x = 1.0;
        self.vel_y = 0.0;
    }
}

impl DrawableUpdatable for Player {}
impl Person for Player {}
