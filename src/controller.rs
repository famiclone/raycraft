use sdl2::keyboard::Keycode;

pub struct KeyboardState {
    pressed_keys: Vec<Keycode>,
}

impl KeyboardState {
    pub fn new() -> Self {
        KeyboardState {
            pressed_keys: Vec::new(),
        }
    }

    pub fn handle_key_down(&mut self, keycode: Keycode) {
        if !self.pressed_keys.contains(&keycode) {
            self.pressed_keys.push(keycode);
        }
    }

    pub fn handle_key_up(&mut self, keycode: Keycode) {
        if let Some(index) = self.pressed_keys.iter().position(|&k| k == keycode) {
            self.pressed_keys.remove(index);
        }
    }

    pub fn is_key_pressed(&self, keycode: Keycode) -> bool {
        self.pressed_keys.contains(&keycode)
    }
}

