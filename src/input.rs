pub use winit::event::VirtualKeyCode as KeyCode;

use std::collections::HashSet;

pub struct InputContext {
    pub keyboard: KeyboardContext,
}

impl Default for InputContext {
    fn default() -> Self {
        let keyboard = KeyboardContext::default();
        Self { keyboard }
    }
}

pub struct KeyboardContext {
    pressed: HashSet<KeyCode>,
    previous_pressed: HashSet<KeyCode>,
}

impl Default for KeyboardContext {
    fn default() -> Self {
        let pressed = HashSet::<KeyCode>::new();
        let previous_pressed = HashSet::<KeyCode>::new();

        Self {
            pressed,
            previous_pressed,
        }
    }
}

// Getting keys
impl KeyboardContext {
    /// Returns true if KeyCode is down
    /// Accepts repeating
    pub fn key_pressed(&self, keycode: KeyCode) -> bool {
        self.pressed.contains(&keycode)
    }

    /// Returns true if KeyCode was pressed this frame
    /// Does not accepts repeating
    pub fn key_just_pressed(&self, keycode: KeyCode) -> bool {
        self.pressed.contains(&keycode) && !self.previous_pressed.contains(&keycode)
    }

    /// Returns true is KeyCode was released this frame
    pub fn key_released(&self, keycode: KeyCode) -> bool {
        !self.pressed.contains(&keycode) && self.previous_pressed.contains(&keycode)
    }

    /// Save current keys in previous
    pub(crate) fn save_keys(&mut self) {
        self.previous_pressed = self.pressed.clone()
    }

    /// Sets key for current frame
    pub(crate) fn set_key(&mut self, keycode: KeyCode) {
        self.pressed.insert(keycode);
    }

    /// Release key
    pub(crate) fn release_key(&mut self, keycode: KeyCode) {
        self.pressed.remove(&keycode);
    }
}

#[cfg(test)]
mod tests {
    use crate::input::KeyCode;
    use crate::input::KeyboardContext;

    #[test]
    fn key_pressed_test() {
        let mut kc = KeyboardContext::default();

        kc.set_key(KeyCode::A);

        assert!(kc.key_pressed(KeyCode::A));
        assert!(!kc.key_pressed(KeyCode::B));

        kc.save_keys();
        kc.set_key(KeyCode::B);

        assert!(kc.key_pressed(KeyCode::A));
        assert!(kc.key_pressed(KeyCode::B));

        kc.save_keys();
        kc.release_key(KeyCode::A);

        assert!(!kc.key_pressed(KeyCode::A));
        assert!(kc.key_pressed(KeyCode::B));
    }

    #[test]
    fn key_just_pressed_test() {
        let mut kc = KeyboardContext::default();
        kc.set_key(KeyCode::A);

        assert!(kc.key_just_pressed(KeyCode::A));

        kc.save_keys();
        kc.set_key(KeyCode::A);

        assert!(!kc.key_just_pressed(KeyCode::A));
    }

    #[test]
    fn key_released_test() {
        let mut kc = KeyboardContext::default();
        kc.set_key(KeyCode::A);

        assert!(!kc.key_released(KeyCode::A));

        kc.save_keys();
        kc.release_key(KeyCode::A);

        assert!(kc.key_released(KeyCode::A));
    }
}
