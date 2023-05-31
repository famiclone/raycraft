
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::{Duration, Instant};

mod traits;
use traits::Movable;
mod player;
use player::Player;

mod controller;
use controller::KeyboardState;

mod scene_manager;
use scene_manager::{Entity, Scene, SceneManager};

mod debugger;
use debugger::Debugger;

mod renderer;
use renderer::Renderer;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

//const MAP_WIDTH: u32 = 16;
//const MAP_HEIGHT: u32 = 16;

//type Map = [u8; (MAP_WIDTH as usize) * (MAP_HEIGHT as usize)];


fn main() {
    let mut last_time = Instant::now();
    let target_frame_duration = Duration::from_secs_f32(1.0 / 60.0);

    let ctx = sdl2::init().unwrap();
    let mut renderer = Renderer::new(ctx, WINDOW_WIDTH, WINDOW_HEIGHT);

    let mut counter: f32 = 0.0;
    let player: Entity = Entity::Player(Player::new(0.0, 0.0));
    let mut keyboard_state = KeyboardState::new();
    let ttf_context = sdl2::ttf::init().unwrap();
    let debugger = Debugger::new(&ttf_context);

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

        for event in renderer.event_pump.poll_iter() {
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

        renderer.canvas.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        scene_manager.update(dt.as_secs_f32());
        scene_manager.draw(&mut renderer.canvas);

        debugger.render_text(
            &mut renderer.canvas,
            "Hello, world!",
            sdl2::rect::Point::new(0, 0),
            sdl2::pixels::Color::RGB(255, 255, 255),
        );
        renderer.present();

        last_time = now;
        let frame_duration = Instant::now().duration_since(last_time);
        if frame_duration < target_frame_duration {
            std::thread::sleep(target_frame_duration - frame_duration);
        }
    }
}
