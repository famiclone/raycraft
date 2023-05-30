const VERSION: &str = "0.1.0";

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::{Duration, Instant};

mod traits;
use traits::{Drawable, Updatable};
mod player;
use player::Player;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

//const MAP_WIDTH: u32 = 16;
//const MAP_HEIGHT: u32 = 16;

//type Map = [u8; (MAP_WIDTH as usize) * (MAP_HEIGHT as usize)];


struct Sprite<'a> {
    width: u32,
    height: u32,
    texture: sdl2::render::Texture<'a>,
}


fn main() {
    let mut last_time = Instant::now();
    let mut dt = Duration::from_secs(0);

    let ctx = sdl2::init().unwrap();
    let video_subsystem = ctx.video().unwrap();

    let window = video_subsystem
        .window(format!("raycraft {}", VERSION).as_str(), WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut player = Player::new(0, 0);
    let mut event_pump = ctx.event_pump().unwrap();

    'run: loop {
        let now = Instant::now();
        dt = now - last_time;
        last_time = now;

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

        player.update(dt.as_secs_f32());
        player.draw(&mut canvas);

        canvas.present();
    }
}
