use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::{traits::{Drawable, DrawableUpdatable, Updatable, Person}, player::Player};

pub enum Entity {
    Generic(Box<dyn DrawableUpdatable>),
    Player(Player),
}

pub struct Scene {
    pub name: String,
    pub entities: Vec<Entity>,
}

impl Updatable for Scene {
    fn update(&mut self, dt: f32) {
        for entity in &mut self.entities {
            match entity {
                Entity::Generic(obj) => obj.update(dt),
                Entity::Player(player) => player.update(dt),
            }
        }
    }
}

impl Drawable for Scene {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        for entity in &self.entities {
            match entity {
                Entity::Generic(obj) => obj.draw(canvas),
                Entity::Player(player) => player.draw(canvas),
            }
        }
    }
}

impl Scene {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            entities: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn get_player(&mut self) -> Option<&mut Player> {
        for entity in &mut self.entities {
            if let Entity::Player(player) = entity {
                return Some(player);
            }
        }
        None
    }
}

pub struct SceneManager {
    scenes: Vec<Scene>,
    current_scene_name: Option<&'static str>,
    current_scene: Option<Scene>,
}

impl Updatable for SceneManager {
    fn update(&mut self, dt: f32) {}
}

impl Drawable for SceneManager {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        if let Some(scene) = self.current_scene_name {
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
        Self {
            scenes: Vec::new(),
            current_scene: None,
            current_scene_name: None,
        }
    }

    pub fn add_scene(&mut self, scene: Scene) {
        self.scenes.push(scene);
    }

    pub fn set_current_scene(&mut self, scene_name: &'static str) {
        self.current_scene_name = Some(scene_name);
    }

    pub fn get_current_scene(&mut self) -> Option<&mut Scene> {
        self.scenes.iter_mut().find(|s| s.name == self.current_scene_name.unwrap())
    }

    pub fn update(&mut self, dt: f32) {
        if let Some(scene) = self.current_scene_name {
            for s in &mut self.scenes {
                if s.name == scene {
                    s.update(dt);
                }
            }
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        if let Some(scene) = self.current_scene_name {
            for s in &self.scenes {
                if s.name == scene {
                    s.draw(canvas);
                }
            }
        }
    }
}
