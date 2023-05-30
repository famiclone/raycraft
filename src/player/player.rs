mod traits;
use traits::{Drawable, Updatable};

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;


pub struct Player {
    speed: f32,
    sprite: Rect,
}

impl Drawable for Player {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.fill_rect(self.sprite).unwrap();
    }
}

impl Updatable for Player {
    fn update(&mut self, dt: f32) {
        self.sprite.x = (self.sprite.x() as f32 + self.speed * dt).round() as i32;
        self.sprite.y = (self.sprite.y() as f32 + self.speed * dt).round() as i32;
    }
}

impl Player {
    fn new(pos_x: i32, pos_y: i32) -> Self {
        Self {
            speed: 0.11,
            sprite: Rect::new(pos_x, pos_y, 100, 100),
        }
    }
}
