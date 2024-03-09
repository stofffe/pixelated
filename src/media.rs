// Upload screenshots

use crate::Context;
use image::{ImageResult, RgbaImage};

/// Can take screenshots of a canvas
pub(crate) struct ScreenshotUploader {
    width: u32,
    height: u32,
}

impl ScreenshotUploader {
    /// Create ScreenshotUploader with specific width and height
    pub(crate) fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    /// Export current state of canvas to a image at the specified path
    pub(crate) fn export_to_file(&self, pixels: &[u8], path: &str) -> ImageResult<()> {
        let mut img = RgbaImage::new(self.width, self.height);

        img.copy_from_slice(pixels);

        img.save(path)
    }
}

//
// Commands
//

/// Export the current canvas to a png at the desired path
pub fn export_screenshot(ctx: &Context, path: &str) -> ImageResult<()> {
    ctx.render
        .screenshot_uploader
        .export_to_file(&ctx.render.canvas.pixels, path)
}
