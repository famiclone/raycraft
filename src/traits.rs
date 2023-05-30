use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait Drawable {
    fn draw(&self, canvas: &mut Canvas<Window>);
}

pub trait Updatable {
    fn update(&mut self, dt: f32);
}

pub trait DrawableUpdatable: Drawable + Updatable {}
