pub use winit::event::MouseButton;
pub use winit::event::VirtualKeyCode as KeyCode;

use crate::{render::RenderContext, Context};
use std::collections::HashSet;
use winit::event::ModifiersState;

#[derive(Default)]
pub(crate) struct InputContext {
    pub keyboard: KeyboardContext,
    pub mouse: MouseContext,
}

#[derive(Default)]
pub(crate) struct MouseContext {
    on_screen: bool,
    pos: (f64, f64),
    mouse_delta: (f64, f64),
    pressed: HashSet<MouseButton>,
    previous_pressed: HashSet<MouseButton>,
    scroll_delta: (f64, f64),
}

impl MouseContext {
    /// Returns true if Button is down
    /// Accepts repeating
    fn button_pressed(&self, keycode: MouseButton) -> bool {
        self.pressed.contains(&keycode)
    }

    /// Returns true if Button was pressed this frame
    /// Does not accept repeating
    fn button_just_pressed(&self, keycode: MouseButton) -> bool {
        self.pressed.contains(&keycode) && !self.previous_pressed.contains(&keycode)
    }

    /// Returns true is MouseButton was released this frame
    fn button_released(&self, keycode: MouseButton) -> bool {
        !self.pressed.contains(&keycode) && self.previous_pressed.contains(&keycode)
    }

    /// Returns if mouse is on screen or not
    fn on_screen(&self) -> bool {
        self.on_screen
    }

    /// Returns the current physical coordinates for the mouse
    fn mouse_pos_physical(&self) -> (f64, f64) {
        self.pos
    }

    /// Returns the current pixel under the mouse
    fn mouse_pos_pixel(&self, ctx: &RenderContext) -> (u32, u32) {
        // When holding the mouse button down pos can get bigger than physical size
        // So clamp to avoid out of bounds
        let window_size = ctx.window.inner_size();
        let relative_x = self.pos.0 / window_size.width as f64;
        let relative_y = self.pos.1 / window_size.height as f64;
        let pixel_x = relative_x * ctx.canvas.width as f64;
        let pixel_y = relative_y * ctx.canvas.height as f64;
        (pixel_x as u32, pixel_y as u32)
    }

    /// Returns the (dx, dy) change in mouse position
    fn mouse_delta(&self) -> (f64, f64) {
        self.mouse_delta
    }

    fn scroll_delta(&self) -> (f64, f64) {
        self.scroll_delta
    }
}

impl MouseContext {
    /// Sets mouse off screen
    pub(crate) fn set_on_screen(&mut self, on_screen: bool) {
        self.on_screen = on_screen;
    }

    // Sets the current position of the mouse
    pub(crate) fn set_pos(&mut self, x: f64, y: f64) {
        self.pos = (x, y);
    }

    /// Sets the (dx, dy) change in mouse position
    pub(crate) fn set_mouse_delta(&mut self, change: (f64, f64)) {
        self.mouse_delta = change;
    }

    pub(crate) fn set_scroll_delta(&mut self, change: (f64, f64)) {
        self.scroll_delta = change;
    }

    /// Sets button for current frame
    pub(crate) fn press_button(&mut self, keycode: MouseButton) {
        self.pressed.insert(keycode);
    }

    /// Release button
    pub(crate) fn release_button(&mut self, keycode: MouseButton) {
        self.pressed.remove(&keycode);
    }

    /// Save current buttons in previous
    /// Should be called each frame
    pub(crate) fn save_buttons(&mut self) {
        self.previous_pressed = self.pressed.clone()
    }
}

//
// Mouse commands
//

/// Returns the mouse delta for the current frame
pub fn mouse_delta(ctx: &Context) -> (f32, f32) {
    let (dx, dy) = ctx.input.mouse.mouse_delta();
    (dx as f32, dy as f32)
}

/// Returns if mouse is on screen or not
pub fn mouse_on_screen(ctx: &Context) -> bool {
    ctx.input.mouse.on_screen()
}

/// Returns the current physical coordinates for the mouse
pub fn mouse_pos_physical(ctx: &Context) -> (f64, f64) {
    ctx.input.mouse.mouse_pos_physical()
}

/// Returns the current pixel under the mouse
pub fn mouse_pos_pixel(ctx: &Context) -> (u32, u32) {
    ctx.input.mouse.mouse_pos_pixel(&ctx.render)
}

/// Returns true if MouseButton is pressed
/// Accepts repeating
pub fn mouse_button_pressed(ctx: &Context, keycode: MouseButton) -> bool {
    ctx.input.mouse.button_pressed(keycode)
}

/// Returns true if MouseButton was pressed this frame
pub fn mouse_button_just_pressed(ctx: &Context, keycode: MouseButton) -> bool {
    ctx.input.mouse.button_just_pressed(keycode)
}

/// Returns true if MouseButton was released this frame
pub fn mouse_button_released(ctx: &Context, keycode: MouseButton) -> bool {
    ctx.input.mouse.button_released(keycode)
}

/// Returns the scroll delta for the current frame
pub fn scroll_delta(ctx: &Context) -> (f32, f32) {
    let (dx, dy) = ctx.input.mouse.scroll_delta();
    (dx as f32, dy as f32)
}

#[derive(Default)]
pub(crate) struct KeyboardContext {
    pressed: HashSet<KeyCode>,
    previous_pressed: HashSet<KeyCode>,
    pressed_modifiers: HashSet<KeyModifier>,
    previous_pressed_modifiers: HashSet<KeyModifier>,
}

#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
pub enum KeyModifier {
    Shift,
    Ctrl,
    Alt,
    Logo,
}

// Getting keys
impl KeyboardContext {
    /// Returns true if KeyCode is down
    /// Accepts repeating
    fn key_pressed(&self, keycode: KeyCode) -> bool {
        self.pressed.contains(&keycode)
    }

    /// Returns true if KeyCode was pressed this frame
    /// Does not accepts repeating
    fn key_just_pressed(&self, keycode: KeyCode) -> bool {
        self.pressed.contains(&keycode) && !self.previous_pressed.contains(&keycode)
    }

    /// Returns true is KeyCode was released this frame
    fn key_released(&self, keycode: KeyCode) -> bool {
        !self.pressed.contains(&keycode) && self.previous_pressed.contains(&keycode)
    }

    fn modifier_pressed(&self, modifier: KeyModifier) -> bool {
        self.pressed_modifiers.contains(&modifier)
    }

    fn modifier_just_pressed(&self, modifier: KeyModifier) -> bool {
        self.pressed_modifiers.contains(&modifier)
            && !self.previous_pressed_modifiers.contains(&modifier)
    }

    fn modifier_released(&self, modifier: KeyModifier) -> bool {
        !self.pressed_modifiers.contains(&modifier)
            && self.previous_pressed_modifiers.contains(&modifier)
    }
}

impl KeyboardContext {
    /// Sets key for current frame
    pub(crate) fn set_key(&mut self, keycode: KeyCode) {
        self.pressed.insert(keycode);
    }

    /// Release key
    pub(crate) fn release_key(&mut self, keycode: KeyCode) {
        self.pressed.remove(&keycode);
    }

    pub fn modifiers_changed(&mut self, state: ModifiersState) {
        self.pressed_modifiers.clear();
        if state.shift() {
            self.pressed_modifiers.insert(KeyModifier::Shift);
        }
        if state.ctrl() {
            self.pressed_modifiers.insert(KeyModifier::Ctrl);
        }
        if state.alt() {
            self.pressed_modifiers.insert(KeyModifier::Alt);
        }
        if state.logo() {
            self.pressed_modifiers.insert(KeyModifier::Logo);
        }
    }

    /// Save current keys in previous
    /// Should be called each frame
    pub(crate) fn save_keys(&mut self) {
        self.previous_pressed = self.pressed.clone();
    }

    /// Save current keys modifiers in previous
    /// Should be called each frame
    pub(crate) fn save_modifiers(&mut self) {
        self.previous_pressed_modifiers = self.pressed_modifiers.clone();
    }
}

//
// Keyboard commands
//

/// Returns true if KeyCode is pressed
/// Accepts repeating
pub fn key_pressed(ctx: &Context, keycode: KeyCode) -> bool {
    ctx.input.keyboard.key_pressed(keycode)
}

/// Returns true if KeyCode was pressed this frame
pub fn key_just_pressed(ctx: &Context, keycode: KeyCode) -> bool {
    ctx.input.keyboard.key_just_pressed(keycode)
}

/// Returns true is KeyCode was released this frame
pub fn key_released(ctx: &Context, keycode: KeyCode) -> bool {
    ctx.input.keyboard.key_released(keycode)
}

/// Returns true if KeyModifer is pressed
/// Accepts repeating
pub fn modifier_pressed(ctx: &Context, key_modifier: KeyModifier) -> bool {
    ctx.input.keyboard.modifier_pressed(key_modifier)
}

/// Returns true if KeyModifer was pressed this frame
pub fn modifer_just_pressed(ctx: &Context, key_modifier: KeyModifier) -> bool {
    ctx.input.keyboard.modifier_just_pressed(key_modifier)
}

/// Returns true if KeyModifier was released this frame
pub fn modifer_released(ctx: &Context, key_modifier: KeyModifier) -> bool {
    ctx.input.keyboard.modifier_released(key_modifier)
}

#[cfg(test)]
mod tests {
    use winit::event::ModifiersState;

    use crate::input::KeyCode;
    use crate::input::KeyModifier;
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

    #[test]
    fn modifer_pressed_test() {
        let mut kc = KeyboardContext::default();

        // Press Shift
        kc.modifiers_changed(ModifiersState::SHIFT);

        assert!(kc.modifier_pressed(KeyModifier::Shift));
        assert!(!kc.modifier_pressed(KeyModifier::Ctrl));

        kc.save_modifiers();

        // Press Shift and Ctrl
        kc.modifiers_changed(ModifiersState::SHIFT | ModifiersState::CTRL);

        assert!(kc.modifier_pressed(KeyModifier::Shift));
        assert!(kc.modifier_pressed(KeyModifier::Ctrl));

        kc.save_modifiers();

        // Release Shift
        kc.modifiers_changed(ModifiersState::CTRL);

        assert!(!kc.modifier_pressed(KeyModifier::Shift));
        assert!(kc.modifier_pressed(KeyModifier::Ctrl));
    }

    #[test]
    fn modifier_just_pressed_test() {
        let mut kc = KeyboardContext::default();
        // Press shift
        kc.modifiers_changed(ModifiersState::SHIFT);

        assert!(kc.modifier_just_pressed(KeyModifier::Shift));

        kc.save_modifiers();

        // Release shift
        kc.modifiers_changed(ModifiersState::from_bits(0).unwrap());

        assert!(!kc.modifier_just_pressed(KeyModifier::Shift));
    }

    #[test]
    fn modifier_released_test() {
        let mut kc = KeyboardContext::default();

        // Press shift
        kc.modifiers_changed(ModifiersState::SHIFT);

        assert!(!kc.modifier_released(KeyModifier::Shift));
        assert!(!kc.modifier_released(KeyModifier::Ctrl));

        kc.save_modifiers();

        // Release shift
        kc.modifiers_changed(ModifiersState::from_bits(0).unwrap());

        assert!(kc.modifier_released(KeyModifier::Shift));
        assert!(!kc.modifier_released(KeyModifier::Ctrl));
    }
}
// use winit::event::MouseButton;
// pub use winit::event::VirtualKeyCode as KeyCode;
//
// use std::collections::HashSet;
//
// use crate::render::RenderContext;
//
// #[derive(Default)]
// pub struct InputContext {
//     pub keyboard: KeyboardContext,
//     pub mouse: MouseContext,
// }
//
// #[derive(Default)]
// pub struct MouseContext {
//     on_screen: bool,
//     pos: (f64, f64),
//     delta: (f64, f64),
//     pressed: HashSet<MouseButton>,
//     previous_pressed: HashSet<MouseButton>,
// }
//
// impl MouseContext {
//     /// Returns if mouse is on screen or not
//     pub fn on_screen(&self) -> bool {
//         self.on_screen
//     }
//
//     /// Returns the current physical coordinates for the mouse
//     pub fn last_physical_pos(&self) -> (f64, f64) {
//         self.pos
//     }
//
//     /// Returns the current pixel under the mouse
//     pub fn last_pixel_pos(&self, ctx: &RenderContext) -> (u32, u32) {
//         // When holding the mouse button down pos can get bigger than physical size
//         // So clamp to avoid out of bounds
//         let relative_x = self.pos.0 / ctx.window_size.width as f64;
//         let relative_y = self.pos.1 / ctx.window_size.height as f64;
//         let pixel_x = relative_x * ctx.canvas.width as f64;
//         let pixel_y = relative_y * ctx.canvas.height as f64;
//         (pixel_x as u32, pixel_y as u32)
//     }
//
//     /// Returns true if Button is down
//     /// Accepts repeating
//     pub fn button_pressed(&self, keycode: MouseButton) -> bool {
//         self.pressed.contains(&keycode)
//     }
//
//     /// Returns true if Button was pressed this frame
//     /// Does not accept repeating
//     pub fn button_just_pressed(&self, keycode: MouseButton) -> bool {
//         self.pressed.contains(&keycode) && !self.previous_pressed.contains(&keycode)
//     }
//
//     /// Returns true is MouseButton was released this frame
//     pub fn button_released(&self, keycode: MouseButton) -> bool {
//         !self.pressed.contains(&keycode) && self.previous_pressed.contains(&keycode)
//     }
//
//     /// Returns the (dx, dy) change in mouse position
//     pub fn mouse_delta(&self) -> (f64, f64) {
//         self.delta
//     }
//
//     /// Sets the (dx, dy) change in mouse position
//     pub(crate) fn set_mouse_delta(&mut self, change: (f64, f64)) {
//         self.delta = change;
//     }
//
//     /// Sets mouse off screen
//     pub(crate) fn set_off_screen(&mut self) {
//         self.on_screen = false;
//     }
//
//     // Sets the current position of the mouse
//     pub(crate) fn set_pos(&mut self, pos: (f64, f64), ctx: &RenderContext) {
//         self.pos = pos;
//
//         // Check if mouse is on screen
//         // When holding mouse button CursorLeft event will not be called so need check here
//         if pos.0 >= 0.0
//             && pos.0 < ctx.window_size.width as f64
//             && pos.1 >= 0.0
//             && pos.1 < ctx.window_size.height as f64
//         {
//             self.on_screen = true;
//         } else {
//             self.on_screen = false;
//         }
//     }
//
//     /// Save current buttons in previous
//     /// Should be called each frame
//     pub(crate) fn save_buttons(&mut self) {
//         self.previous_pressed = self.pressed.clone()
//     }
//
//     /// Sets button for current frame
//     pub(crate) fn set_buttons(&mut self, keycode: MouseButton) {
//         self.pressed.insert(keycode);
//     }
//
//     /// Release button
//     pub(crate) fn release_button(&mut self, keycode: MouseButton) {
//         self.pressed.remove(&keycode);
//     }
// }
//
// #[derive(Default)]
// pub struct KeyboardContext {
//     pressed: HashSet<KeyCode>,
//     previous_pressed: HashSet<KeyCode>,
// }
//
// // Getting keys
// impl KeyboardContext {
//     /// Returns true if KeyCode is down
//     /// Accepts repeating
//     pub fn key_pressed(&self, keycode: KeyCode) -> bool {
//         self.pressed.contains(&keycode)
//     }
//
//     /// Returns true if KeyCode was pressed this frame
//     /// Does not accepts repeating
//     pub fn key_just_pressed(&self, keycode: KeyCode) -> bool {
//         self.pressed.contains(&keycode) && !self.previous_pressed.contains(&keycode)
//     }
//
//     /// Returns true is KeyCode was released this frame
//     pub fn key_released(&self, keycode: KeyCode) -> bool {
//         !self.pressed.contains(&keycode) && self.previous_pressed.contains(&keycode)
//     }
//
//     /// Save current keys in previous
//     /// Should be called each frame
//     pub(crate) fn save_keys(&mut self) {
//         self.previous_pressed = self.pressed.clone()
//     }
//
//     /// Sets key for current frame
//     pub(crate) fn set_key(&mut self, keycode: KeyCode) {
//         self.pressed.insert(keycode);
//     }
//
//     /// Release key
//     pub(crate) fn release_key(&mut self, keycode: KeyCode) {
//         self.pressed.remove(&keycode);
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use crate::input::KeyCode;
//     use crate::input::KeyboardContext;
//
//     #[test]
//     fn key_pressed_test() {
//         let mut kc = KeyboardContext::default();
//
//         kc.set_key(KeyCode::A);
//
//         assert!(kc.key_pressed(KeyCode::A));
//         assert!(!kc.key_pressed(KeyCode::B));
//
//         kc.save_keys();
//         kc.set_key(KeyCode::B);
//
//         assert!(kc.key_pressed(KeyCode::A));
//         assert!(kc.key_pressed(KeyCode::B));
//
//         kc.save_keys();
//         kc.release_key(KeyCode::A);
//
//         assert!(!kc.key_pressed(KeyCode::A));
//         assert!(kc.key_pressed(KeyCode::B));
//     }
//
//     #[test]
//     fn key_just_pressed_test() {
//         let mut kc = KeyboardContext::default();
//         kc.set_key(KeyCode::A);
//
//         assert!(kc.key_just_pressed(KeyCode::A));
//
//         kc.save_keys();
//         kc.set_key(KeyCode::A);
//
//         assert!(!kc.key_just_pressed(KeyCode::A));
//     }
//
//     #[test]
//     fn key_released_test() {
//         let mut kc = KeyboardContext::default();
//         kc.set_key(KeyCode::A);
//
//         assert!(!kc.key_released(KeyCode::A));
//
//         kc.save_keys();
//         kc.release_key(KeyCode::A);
//
//         assert!(kc.key_released(KeyCode::A));
//     }
// }
