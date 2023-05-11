use crate::context::Context;

/// Write pixel data to a coordinate (r,g,b,a)
/// Overwrites previous pixel
pub fn write_pixel(ctx: &mut Context, x: u32, y: u32, color: &[u8; 3]) {
    ctx.render.canvas.write_pixel(x, y, color);
}

/// Write pixel data to a coordinate (r,g,b,a)
/// Panics if trying to write outside canvas
/// RGBA must be in range [0,1]
pub fn write_pixel_f32(ctx: &mut Context, x: u32, y: u32, color: &[f32; 3]) {
    ctx.render.canvas.write_pixel_f32(x, y, color);
}

/// Write pixel data to a coordinate (r,g,b,a)
/// Non premultiplied alpha blending
pub fn write_pixel_blend(ctx: &mut Context, x: u32, y: u32, color: &[u8; 4]) {
    ctx.render.canvas.write_pixel_blend(x, y, color);
}

/// Write pixel data to a coordinate (r,g,b,a)
/// Non premultiplied alpha blending
/// RGBA must be in range [0,1]
pub fn write_pixel_blend_f32(ctx: &mut Context, x: u32, y: u32, color: &[f32; 4]) {
    ctx.render.canvas.write_pixel_blend_f32(x, y, color);
}

/// Get pixel data for a coordianate
/// Panics if trying to access outside canvas
pub fn get_pixel(ctx: &Context, x: u32, y: u32) -> [u8; 3] {
    ctx.render.canvas.get_pixel(x, y)
}

/// Get pixel data for a coordianate
/// Panics if trying to access outside canvas
pub fn get_pixel_alpha(ctx: &Context, x: u32, y: u32) -> [u8; 4] {
    ctx.render.canvas.get_pixel_alpha(x, y)
}

/// Resizes the canvas and the media uploaders
/// Clears screen to ```clear_color```
pub fn resize(ctx: &mut Context, width: u32, height: u32) {
    ctx.render.canvas.resize(width, height);
    ctx.render.screenshot_uploader.resize(width, height);
    ctx.render.gif_uploader.resize(width, height);
}

/// Set canvas clear color (r,g,b,a)
pub fn set_clear_color(ctx: &mut Context, color: &[u8; 3]) {
    ctx.render.canvas.set_clear_color(color);
}

/// Set canvas clear color (r,g,b)
/// Values must lie in range [0,1]
pub fn set_clear_color_f32(ctx: &mut Context, color: &[f32; 3]) {
    ctx.render.canvas.set_clear_color_f32(color);
}

/// Clears all pixels in canvas to clear color
pub fn clear_screen(ctx: &mut Context) {
    ctx.render.canvas.clear_screen();
}

/// Get canvas capacity
pub fn capacity(ctx: &Context) -> u32 {
    ctx.render.canvas.capacity()
}

/// Get canvas width
pub fn width(ctx: &Context) -> u32 {
    ctx.render.canvas.width
}

/// Get canvas height
pub fn height(ctx: &Context) -> u32 {
    ctx.render.canvas.height
}

/// Get current pixel buffer
pub fn pixel_buffer(ctx: &Context) -> Vec<u8> {
    ctx.render.canvas.get_pixel_buffer()
}
