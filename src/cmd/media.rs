use image::ImageResult;

use crate::context::Context;

/// Export the current canvas to a png at the desired path
pub fn export_screenshot(ctx: &Context, path: &str) -> ImageResult<()> {
    ctx.render
        .screenshot_uploader
        .export_to_file(&ctx.render.canvas.pixels, path)
}

/// Record the current canvas as a frame to the gif buffer
pub fn record_gif_frame(ctx: &mut Context) {
    ctx.render
        .gif_uploader
        .record(ctx.render.canvas.pixels.clone());
}

/// Export the currently recorded frames to the desired location
pub fn export_gif(ctx: &mut Context, path: &str) {
    ctx.render.gif_uploader.export_to_gif(path);
}

/// Clear the currently recorded canvas frames
pub fn clear_gif_frames(ctx: &mut Context) {
    ctx.render.gif_uploader.clear();
}
