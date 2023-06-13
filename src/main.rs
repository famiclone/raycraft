extern crate sdl2;

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::{event::Event, libc::TH_FLAGS_IDLE};
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
const TILE_SIZE: u32 = 16;

//type Map = [u8; (MAP_WIDTH as usize) * (MAP_HEIGHT as usize)];

fn main() {
    let map: Vec<u8> = vec![
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 10
        1, 0, 0, 0, 0, 0, 0, 0, 0, 1, // 20
        1, 0, 0, 0, 0, 0, 0, 0, 0, 1, // 30
        1, 0, 0, 0, 0, 0, 0, 0, 0, 1, // 40
        1, 0, 0, 0, 0, 0, 0, 0, 0, 1, // 50
        1, 0, 0, 0, 0, 0, 0, 0, 0, 1, // 60
        1, 0, 0, 0, 0, 0, 0, 0, 0, 1, // 70
        1, 0, 0, 0, 0, 0, 0, 0, 0, 1, // 80
        1, 0, 0, 0, 0, 0, 0, 0, 0, 1, // 91
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 101
    ];

    let mut last_time = Instant::now();
    let target_frame_duration = Duration::from_secs_f32(1.0 / 60.0);

    let ctx = sdl2::init().unwrap();
    let mut renderer = Renderer::new(ctx, WINDOW_WIDTH, WINDOW_HEIGHT);

    let mut counter: f32 = 0.0;
    let player: Entity = Entity::Player(Player::new(100.0, 100.0));
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
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => keyboard_state.handle_key_down(keycode),
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => keyboard_state.handle_key_up(keycode),
                Event::MouseMotion { x, y, .. } => {
                    // player rotation

                    let mouse_x = x as f32;
                    let mouse_y = y as f32;

                    let player_x = scene_manager
                        .get_current_scene()
                        .unwrap()
                        .get_player()
                        .unwrap()
                        .get_x();

                    let player_y = scene_manager
                        .get_current_scene()
                        .unwrap()
                        .get_player()
                        .unwrap()
                        .get_y();

                    let angle = (mouse_y - player_y as f32).atan2(mouse_x - player_x as f32);

                    scene_manager
                        .get_current_scene()
                        .unwrap()
                        .get_player()
                        .unwrap()
                        .rotate(angle);
                },
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


        debugger.render_text(
            &mut renderer.canvas,
            "Hello, world!",
            sdl2::rect::Point::new(0, 0),
            sdl2::pixels::Color::RGB(255, 255, 255),
        );

        for (i, row) in map.chunks(10).enumerate() {
            for (j, tile) in row.iter().enumerate() {
                if *tile == 1 {
                    renderer.canvas.set_draw_color(Color::RGB(255, 255, 255));
                    renderer
                        .canvas
                        .fill_rect(sdl2::rect::Rect::new(
                            (j as u32 * TILE_SIZE) as i32,
                            (i as u32 * TILE_SIZE) as i32,
                            TILE_SIZE,
                            TILE_SIZE,
                        ))
                        .unwrap();
                }
            }
        }

        scene_manager.draw(&mut renderer.canvas);
        renderer.present();

        last_time = now;
        let frame_duration = Instant::now().duration_since(last_time);
        if frame_duration < target_frame_duration {
            std::thread::sleep(target_frame_duration - frame_duration);
        }
    }
}
