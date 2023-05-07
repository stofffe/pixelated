use winit::event::MouseButton;
pub use winit::event::VirtualKeyCode as KeyCode;

use std::collections::HashSet;

use crate::render::RenderContext;

#[derive(Default)]
pub struct InputContext {
    pub keyboard: KeyboardContext,
    pub mouse: MouseContext,
}

#[derive(Default)]
pub struct MouseContext {
    on_screen: bool,
    pos: (f64, f64),
    pressed: HashSet<MouseButton>,
    previous_pressed: HashSet<MouseButton>,
}

impl MouseContext {
    /// Returns if mouse is on screen or not
    pub fn on_screen(&self) -> bool {
        self.on_screen
    }

    /// Returns the current physical coordinates for the mouse
    pub fn last_physical_pos(&self) -> (f64, f64) {
        self.pos
    }

    /// Returns the current pixel under the mouse
    pub fn last_pixel_pos(&self, ctx: &RenderContext) -> (u32, u32) {
        // When holding the mouse button down pos can get bigger than physical size
        // So clamp to avoid out of bounds
        let relative_x = self.pos.0 / ctx.size.width as f64;
        let relative_y = self.pos.1 / ctx.size.height as f64;
        let pixel_x = relative_x * ctx.canvas.width as f64;
        let pixel_y = relative_y * ctx.canvas.height as f64;
        (pixel_x as u32, pixel_y as u32)
    }

    /// Returns true if Button is down
    /// Accepts repeating
    pub fn button_pressed(&self, keycode: MouseButton) -> bool {
        self.pressed.contains(&keycode)
    }

    /// Returns true if Button was pressed this frame
    /// Does not accepts repeating
    pub fn button_just_pressed(&self, keycode: MouseButton) -> bool {
        self.pressed.contains(&keycode) && !self.previous_pressed.contains(&keycode)
    }

    /// Returns true is MouseButton was released this frame
    pub fn button_released(&self, keycode: MouseButton) -> bool {
        !self.pressed.contains(&keycode) && self.previous_pressed.contains(&keycode)
    }

    /// Sets mouse off screen
    pub(crate) fn set_off_screen(&mut self) {
        self.on_screen = false;
    }

    // Sets the current position of the mouse
    pub(crate) fn set_pos(&mut self, pos: (f64, f64), ctx: &RenderContext) {
        self.pos = pos;

        // Check if mouse is on screen
        // When holding mouse button CursorLeft event will not be called so need check here
        if pos.0 >= 0.0
            && pos.0 < ctx.size.width as f64
            && pos.1 >= 0.0
            && pos.1 < ctx.size.height as f64
        {
            self.on_screen = true;
        } else {
            self.on_screen = false;
        }
    }

    /// Save current buttons in previous
    /// Should be called each frame
    pub(crate) fn save_buttons(&mut self) {
        self.previous_pressed = self.pressed.clone()
    }

    /// Sets button for current frame
    pub(crate) fn set_buttons(&mut self, keycode: MouseButton) {
        self.pressed.insert(keycode);
    }

    /// Release button
    pub(crate) fn release_button(&mut self, keycode: MouseButton) {
        self.pressed.remove(&keycode);
    }
}

#[derive(Default)]
pub struct KeyboardContext {
    pressed: HashSet<KeyCode>,
    previous_pressed: HashSet<KeyCode>,
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
    /// Should be called each frame
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
