use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait Drawable {
    fn draw(&self, canvas: &mut Canvas<Window>);
}

pub trait Updatable {
    fn update(&mut self, dt: f32);
}

pub trait Movable {
    fn move_forward(&mut self);
    fn move_backward(&mut self);
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn stop_horizontal (&mut self);
    fn stop_vertical (&mut self);
}

pub trait DrawableUpdatable: Drawable + Updatable {}

pub trait Person: DrawableUpdatable + Movable {}
