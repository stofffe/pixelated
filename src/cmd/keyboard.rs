use crate::context::Context;
use crate::input::KeyCode;

/// Returns true if KeyCode is down
/// Accepts repeating
pub fn key_pressed(ctx: &Context, keycode: KeyCode) -> bool {
    ctx.input.keyboard.key_pressed(keycode)
}

/// Returns true if KeyCode was pressed this frame
/// Does not accepts repeating
pub fn key_just_pressed(ctx: &Context, keycode: KeyCode) -> bool {
    ctx.input.keyboard.key_just_pressed(keycode)
}

/// Returns true is KeyCode was released this frame
pub fn key_released(ctx: &Context, keycode: KeyCode) -> bool {
    ctx.input.keyboard.key_released(keycode)
}
