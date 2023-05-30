const VERSION: &str = "0.1.0";

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::ttf::Font;
use std::path::Path;
use std::time::{Duration, Instant};

mod traits;
use traits::{Drawable, Updatable};
mod player;
use player::Player;

mod scene_manager;
use scene_manager::{Scene, SceneManager};

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
    let ttf_context = sdl2::ttf::init().unwrap();
    let font_path = Path::new("assets/fonts/font.ttf");
    let font_size = 16;
    let font = ttf_context.load_font(font_path, font_size).unwrap();

    let window = video_subsystem
        .window(
            format!("raycraft {}", VERSION).as_str(),
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut player = Player::new(0, 0);
    let mut event_pump = ctx.event_pump().unwrap();

    let mut scene_manager = SceneManager::new();
    let mut scene = Scene::new("main");
    scene_manager.add_scene("main", scene);
    scene_manager.set_current_scene("main");

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

        let text = format!(
            "FPS: {:.1}{}SCENE: {}",
            1.0 / dt.as_secs_f32(),
            "\n",
            scene_manager.get_current_scene().unwrap()
        );
        let surface = font
            .render(text.as_str())
            .blended(Color::RGB(255, 0, 0))
            .unwrap();
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        let texture_query = texture.query();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        scene_manager.update(dt.as_secs_f32());
        scene_manager.draw(&mut canvas);

        canvas
            .copy(
                &texture,
                None,
                Some(Rect::new(0, 0, texture_query.width, texture_query.height)),
            )
            .unwrap();

        canvas.present();
    }
}
