extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

const MAP_WIDTH: u32 = 16;
const MAP_HEIGHT: u32 = 16;

type Map = [u8; (MAP_WIDTH as usize) * (MAP_HEIGHT as usize)];

struct Player {
    pos_x: i32,
    pos_y: i32,
    sprite: Rect,
}

impl Player {
    fn new(pos_x: i32, pos_y: i32) -> Self {
        Self { pos_x, pos_y, sprite: Rect::new(pos_x, pos_y, 100, 100)}
    }

    fn update(&mut self, dt: usize) {
        self.sprite.x = self.sprite.x() + 1;
        self.sprite.y = self.pos_y + 1;
    }

    fn draw(&mut self, canvas: &mut Canvas<Window> ) {
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.fill_rect(self.sprite).unwrap();
    }
}

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_subsystem = ctx.video().unwrap();

    let window = video_subsystem
        .window("Raycraft", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = ctx.event_pump().unwrap();

    //let player_sprite = Rect::new(player.pos_x, player.pos_y, 100, 100);

    'run: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'run,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let mut player = Player::new(10, 10);
        &player.update(0);
        &player.draw(&mut canvas);


        canvas.present();
    }
}
