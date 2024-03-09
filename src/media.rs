// Upload screenshots and gifs

use std::fs::File;

use gif::{Encoder, Frame, Repeat};
use image::{ImageResult, RgbaImage};

use crate::Context;

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

/// Can record gif of canvas frames
#[derive(Default)]
pub(crate) struct GifUploader {
    frames: Vec<Vec<u8>>,
    width: u32,
    height: u32,
}

impl GifUploader {
    /// Create GifUploader with specific width and height
    /// frame_skip specifies how many frames to step by when exporting to gif
    pub(crate) fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            frames: Vec::default(),
        }
    }

    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    /// Record the current canvas into a frame
    pub(crate) fn record(&mut self, pixels: Vec<u8>) {
        self.frames.push(pixels);
    }

    /// Export the current frames to a file at specified path
    // TODO need to be mut
    pub(crate) fn export_to_gif(&mut self, path: &str) {
        let file = File::create(path).unwrap();
        let mut encoder = Encoder::new(&file, self.width as u16, self.height as u16, &[]).unwrap();

        encoder.set_repeat(Repeat::Infinite).unwrap();

        for frame in self.frames.iter_mut() {
            let mut rgba = RgbaImage::new(self.width, self.height);
            rgba.copy_from_slice(frame);

            let mut gif_frame = Frame::from_rgba(self.width as u16, self.height as u16, frame);
            gif_frame.delay = 1;
            encoder.write_frame(&gif_frame).unwrap();
        }
    }

    /// Clear current frames
    pub(crate) fn clear(&mut self) {
        self.frames.clear();
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
