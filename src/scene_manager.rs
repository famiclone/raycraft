use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::traits::{Drawable, Updatable, DrawableUpdatable};

pub struct Scene {
    pub name: String,
    pub entities: Vec<Box<dyn DrawableUpdatable>>,
}

impl Updatable for Scene {
    fn update(&mut self, dt: f32) {}
}

impl Drawable for Scene {
    fn draw(&self, canvas: &mut Canvas<Window>) {}
}

impl Scene {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), entities: Vec::new() }
    }
}

pub struct SceneManager {
    scenes: Vec<Scene>,
    current_scene: Option<&'static str>,
}

impl Updatable for SceneManager {
    fn update(&mut self, dt: f32) {}
}

impl Drawable for SceneManager {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        if let Some(scene) = self.current_scene {
            for s in &self.scenes {
                if s.name == scene {
                    s.draw(canvas);
                }
            }
        }
    }
}

impl SceneManager {
    pub fn new() -> Self {
        Self { scenes: Vec::new(), current_scene: None }
    }

    pub fn add_scene(&mut self, scene_name: &str, scene: Scene) {
        self.scenes.push(scene);
    }

    pub fn set_current_scene(&mut self, scene_name: &'static str) {
        self.current_scene = Some(scene_name);
    }

    pub fn get_current_scene(&self) -> Option<&'static str> {
        self.current_scene
    }

    pub fn update(&mut self, dt: f32) {
        if let Some(scene) = self.current_scene {
            for s in &mut self.scenes {
                if s.name == scene {
                    s.update(dt);
                }
            }
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        if let Some(scene) = self.current_scene {
            for s in &self.scenes {
                if s.name == scene {
                    s.draw(canvas);
                }
            }
        }
    }
}
