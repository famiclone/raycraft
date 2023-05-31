const VERSION: &str = "0.1.0";

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;
use std::path::Path;
use std::time::{Duration, Instant};

mod traits;
use traits::{Drawable, DrawableUpdatable, Movable, Updatable};
mod player;
use player::Player;

mod controller;
use controller::KeyboardState;

mod scene_manager;
use scene_manager::{Entity, Scene, SceneManager};

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

fn debug(
    canvas: &mut Canvas<Window>,
    scene_manager: &mut SceneManager,
    font: &Font,
    dt: f32,
    counter: &mut f32,
) {
    let color = Color::RGB(255, 255, 255);
    let text = format!(
        "FPS: {:.1}{}SCENE: {}",
        1.0 / dt,
        "\n",
        scene_manager.get_current_scene().unwrap().name
    );

    let lines: Vec<&str> = text.lines().collect();

    let surface = font.render(lines[0]).blended(color).unwrap();

    for line in lines.iter().skip(1) {
        let line_surface = font.render(line).blended(color).unwrap();
    }

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();
    let texture_query = texture.query();

    canvas
        .copy(
            &texture,
            None,
            Some(Rect::new(0, 0, texture_query.width, texture_query.height)),
        )
        .unwrap();
}

fn main() {
    let mut last_time = Instant::now();
    let target_frame_duration = Duration::from_secs_f32(1.0 / 60.0);

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

    let mut counter: f32 = 0.0;
    let mut canvas = window.into_canvas().build().unwrap();
    let mut player: Entity = Entity::Player(Player::new(0.0, 0.0));
    let mut event_pump = ctx.event_pump().unwrap();
    let mut keyboard_state = KeyboardState::new();

    let mut scene_manager = SceneManager::new();
    let mut main_scene = Scene::new("main");
    main_scene.add_entity(player);
    scene_manager.add_scene(main_scene);
    scene_manager.set_current_scene("main");

    'run: loop {
        let now = Instant::now();
        let dt = now.duration_since(last_time);

        counter += dt.as_secs_f32();

        if counter > 1000.0 {
            counter = 0.0;
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'run,
                //Event::KeyDown {
                //    keycode: Some(Keycode::W),
                //    ..
                //} => scene_manager
                //    .get_current_scene()
                //    .unwrap()
                //    .get_player()
                //    .unwrap()
                //    .move_forward(),
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => keyboard_state.handle_key_down(keycode),
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => keyboard_state.handle_key_up(keycode),
                _ => {}
            }
            if keyboard_state.is_key_pressed(Keycode::S) {
                scene_manager
                    .get_current_scene()
                    .unwrap()
                    .get_player()
                    .unwrap()
                    .move_backward();
            } else if keyboard_state.is_key_pressed(Keycode::W) {
                scene_manager
                    .get_current_scene()
                    .unwrap()
                    .get_player()
                    .unwrap()
                    .move_forward();
            } else {
                scene_manager
                    .get_current_scene()
                    .unwrap()
                    .get_player()
                    .unwrap()
                    .stop_vertical();
            }

            if keyboard_state.is_key_pressed(Keycode::A) {
                scene_manager
                    .get_current_scene()
                    .unwrap()
                    .get_player()
                    .unwrap()
                    .move_left();
            } else if keyboard_state.is_key_pressed(Keycode::D) {
                scene_manager
                    .get_current_scene()
                    .unwrap()
                    .get_player()
                    .unwrap()
                    .move_right();
            } else {
                scene_manager
                    .get_current_scene()
                    .unwrap()
                    .get_player()
                    .unwrap()
                    .stop_horizontal();
            }

        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        scene_manager.update(dt.as_secs_f32());
        scene_manager.draw(&mut canvas);

        debug(
            &mut canvas,
            &mut scene_manager,
            &font,
            dt.as_secs_f32(),
            &mut counter,
        );
        canvas.present();

        last_time = now;
        let frame_duration = Instant::now().duration_since(last_time);
        if frame_duration < target_frame_duration {
            std::thread::sleep(target_frame_duration - frame_duration);
        }
    }
}
