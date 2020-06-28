use sdl::event::Key;

pub const CONTROL_UP: Key = Key::Up;
pub const CONTROL_RIGHT: Key = Key::Right;
pub const CONTROL_DOWN: Key = Key::Down;
pub const CONTROL_LEFT: Key = Key::Left;
pub const CONTROL_A: Key = Key::LCtrl;
pub const CONTROL_B: Key = Key::LAlt;
pub const CONTROL_X: Key = Key::Space;
pub const CONTROL_Y: Key = Key::LShift;
pub const CONTROL_TRIGGER_LEFT: Key = Key::Tab;
pub const CONTROL_TRIGGER_RIGHT: Key = Key::Backspace;
pub const CONTROL_SELECT: Key = Key::Escape;
pub const CONTROL_START: Key = Key::Return;

#[derive(Default)]
pub struct PressedKeys {
    pressed_keys: std::collections::HashMap<isize, bool>,
}

impl PressedKeys {
    pub fn process_key(&mut self, key: &sdl::event::Event) {
        if let sdl::event::Event::Key(k, pressed, _, _) = key {
            // Key(_, false, _) means key up, true key down
            self.pressed_keys
                .insert(*k as isize, *pressed)
                .map_or_else(|| (), |_| ())
        }
    }

    pub fn is_pressed(&self, key: Key) -> bool {
        *self.pressed_keys.get(&(key as isize)).unwrap_or(&false)
    }
}
