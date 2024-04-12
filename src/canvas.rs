use crate::Context;

pub const DEFAULT_CLEAR_COLOR: [u8; 3] = [0, 0, 0]; // Black
pub const DEFAULT_CANVAS_WIDTH: u32 = 512;
pub const DEFAULT_CANVAS_HEIGHT: u32 = 512;

/// Represent the screen of pixels
pub(crate) struct Canvas {
    pub(crate) pixels: Vec<u8>,
    pub(crate) width: u32,
    pub(crate) height: u32,
    last_clear_color: [u8; 3],
}

impl Canvas {
    /// Create new canvas with specified width and height
    pub(crate) fn new(width: u32, height: u32) -> Self {
        let capacity = width * height * 4;
        let pixels = vec![0; capacity as usize];
        let last_clear_color = DEFAULT_CLEAR_COLOR;
        Self {
            pixels,
            width,
            height,
            last_clear_color,
        }
    }

    /// Resizes the canvas
    /// Clears screen to ```clear_color```
    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        let capacity = width * height * 4;

        self.pixels.resize(capacity as usize, 0);
        self.width = width;
        self.height = height;

        let color = self.last_clear_color;
        self.clear_screen(&color);
    }

    /// Clone pixel buffer
    pub(crate) fn get_pixel_buffer(&self) -> Vec<u8> {
        self.pixels.clone()
    }

    /// Get pixel data for a coordianate
    /// Panics if trying to access outside canvas
    pub(crate) fn pixel_rgb(&self, x: u32, y: u32) -> [u8; 3] {
        assert_pixel(x, y, self.width, self.height);

        let index = (y * 4 * self.width + x * 4) as usize;
        [
            self.pixels[index],
            self.pixels[index + 1],
            self.pixels[index + 2],
        ]
    }

    /// Get pixel data for a coordianate
    /// Panics if trying to access outside canvas
    pub(crate) fn pixel_rgb_f32(&self, x: u32, y: u32) -> [f32; 3] {
        assert_pixel(x, y, self.width, self.height);

        let index = (y * 4 * self.width + x * 4) as usize;
        [
            self.pixels[index] as f32 / 255.0,
            self.pixels[index + 1] as f32 / 255.0,
            self.pixels[index + 2] as f32 / 255.0,
        ]
    }

    /// Get pixel data for a coordianate
    /// Panics if trying to access outside canvas
    pub(crate) fn pixel_rgba(&self, x: u32, y: u32) -> [u8; 4] {
        assert_pixel(x, y, self.width, self.height);

        let index = (y * 4 * self.width + x * 4) as usize;
        [
            self.pixels[index],
            self.pixels[index + 1],
            self.pixels[index + 2],
            self.pixels[index + 3],
        ]
    }

    /// Get pixel data for a coordianate
    /// Panics if trying to access outside canvas
    pub(crate) fn pixel_rgba_f32(&self, x: u32, y: u32) -> [f32; 4] {
        assert_pixel(x, y, self.width, self.height);

        let index = (y * 4 * self.width + x * 4) as usize;
        [
            self.pixels[index] as f32 / 255.0,
            self.pixels[index + 1] as f32 / 255.0,
            self.pixels[index + 2] as f32 / 255.0,
            self.pixels[index + 3] as f32 / 255.0,
        ]
    }

    /// Write pixel data to a coordinate (r,g,b,a)
    /// Overwrites previous pixel
    pub(crate) fn write_pixel(&mut self, x: u32, y: u32, color: &[u8; 3]) {
        assert_pixel(x, y, self.width, self.height);

        let index = (y * 4 * self.width + x * 4) as usize;
        self.pixels[index] = color[0];
        self.pixels[index + 1] = color[1];
        self.pixels[index + 2] = color[2];
        self.pixels[index + 3] = 255;
    }

    /// Write pixel data to a coordinate (r,g,b,a)
    /// Panics if trying to write outside canvas
    /// RGBA must be in range [0,1]
    pub(crate) fn write_pixel_f32(&mut self, x: u32, y: u32, color: &[f32; 3]) {
        assert_pixel(x, y, self.width, self.height);
        assert_rgb(color);

        let color = [
            (color[0] * 255.0) as u8,
            (color[1] * 255.0) as u8,
            (color[2] * 255.0) as u8,
        ];

        self.write_pixel(x, y, &color);
    }

    /// Write pixel data to a coordinate (r,g,b,a)
    /// Non premultiplied alpha blending
    pub(crate) fn write_pixel_blend(&mut self, x: u32, y: u32, color: &[u8; 4]) {
        assert_pixel(x, y, self.width, self.height);

        // Alpha blending where a is over b
        // https://en.wikipedia.org/wiki/Alpha_compositing
        let prev_color = self.pixel_rgba(x, y);
        let a = &[
            color[0] as f32 / 255.0,
            color[1] as f32 / 255.0,
            color[2] as f32 / 255.0,
            color[3] as f32 / 255.0,
        ];
        let b = &[
            prev_color[0] as f32 / 255.0,
            prev_color[1] as f32 / 255.0,
            prev_color[2] as f32 / 255.0,
            prev_color[3] as f32 / 255.0,
        ];

        let alpha_a = a[3];
        let alpha_b = b[3];
        let alpha_over = alpha_a + alpha_b * (1.0 - alpha_a);

        let result_r = (a[0] * alpha_a + b[0] * alpha_b * (1.0 - alpha_a)) / alpha_over;
        let result_g = (a[1] * alpha_a + b[1] * alpha_b * (1.0 - alpha_a)) / alpha_over;
        let result_b = (a[2] * alpha_a + b[2] * alpha_b * (1.0 - alpha_a)) / alpha_over;
        let result_a = alpha_over;

        let index = (y * 4 * self.width + x * 4) as usize;
        self.pixels[index] = (result_r * 255.0) as u8;
        self.pixels[index + 1] = (result_g * 255.0) as u8;
        self.pixels[index + 2] = (result_b * 255.0) as u8;
        self.pixels[index + 3] = (result_a * 255.0) as u8;
    }

    /// Write pixel data to a coordinate (r,g,b,a)
    /// Non premultiplied alpha blending
    /// RGBA must be in range [0,1]
    pub(crate) fn write_pixel_blend_f32(&mut self, x: u32, y: u32, color: &[f32; 4]) {
        assert_pixel(x, y, self.width, self.height);
        assert_rgba(color);

        let color = [
            (color[0] * 255.0) as u8,
            (color[1] * 255.0) as u8,
            (color[2] * 255.0) as u8,
            (color[3] * 255.0) as u8,
        ];

        self.write_pixel_blend(x, y, &color);
    }

    /// Clears all pixels in canvas to clear color
    pub(crate) fn clear_screen(&mut self, color: &[u8; 3]) {
        for pixel in self.pixels.chunks_mut(4) {
            pixel[0] = color[0];
            pixel[1] = color[1];
            pixel[2] = color[2];
            pixel[3] = 255;
        }
        self.last_clear_color = *color;
    }

    /// Clears all pixels in canvas to clear color
    pub(crate) fn clear_screen_f32(&mut self, color: &[f32; 3]) {
        let color = &[
            (color[0] * 255.0) as u8,
            (color[1] * 255.0) as u8,
            (color[2] * 255.0) as u8,
        ];
        self.clear_screen(color);
    }
}

/// Asserts a pixel is inside the screen
fn assert_pixel(x: u32, y: u32, width: u32, height: u32) {
    debug_assert!(
        x < width && y < height,
        "pixel ({}, {}) not in range [{}, {})",
        x,
        y,
        width,
        height
    );
}

/// Asserts color components are in range [0,1]
fn assert_rgb(color: &[f32; 3]) {
    debug_assert!(
        (0.0..=1.0).contains(&color[0]),
        "r ({}) not in range [0,1]",
        color[0]
    );
    debug_assert!(
        (0.0..=1.0).contains(&color[1]),
        "g ({}) not in range [0,1]",
        color[1]
    );
    debug_assert!(
        (0.0..=1.0).contains(&color[2]),
        "b ({}) not in range [0,1]",
        color[2]
    );
}

/// Asserts color components are in range [0,1]
fn assert_rgba(color: &[f32; 4]) {
    assert_rgb(&[color[0], color[1], color[2]]);
    debug_assert!(
        (0.0..=1.0).contains(&color[3]),
        "a ({}) not in range [0,1]",
        color[3]
    );
}

// Commands

/// Mutable reference to pixel array
///
/// Stored as list of u8, chunks of 4 represent RGBA
pub fn pixels_ref(ctx: &mut Context) -> &mut Vec<u8> {
    &mut ctx.render.canvas.pixels
}

/// Copy of pixel buffer
///
/// Stored as list of u8, chunks of 4 represent RGBA
pub fn pixels_copy(ctx: &Context) -> Vec<u8> {
    ctx.render.canvas.get_pixel_buffer()
}

/// Write color to pixel at (x, y)
///
/// Color: Full opacity RGB \[0,255\]
///
/// Panics if trying to write outside canvas
pub fn write_pixel_rgb(ctx: &mut Context, x: u32, y: u32, color: &[u8; 3]) {
    ctx.render.canvas.write_pixel(x, y, color);
}

/// Write color to pixel at (x, y)
///
/// Color: Full opacity RGB \[0,1\]
///
/// Panics if trying to write outside canvas
pub fn write_pixel_rgb_f32(ctx: &mut Context, x: u32, y: u32, color: &[f32; 3]) {
    ctx.render.canvas.write_pixel_f32(x, y, color);
}

/// Write color to pixel at (x, y)
///
/// Color: Non premultiplied RGBA \[0,255\]
///
/// Panics if trying to write outside canvas
pub fn write_pixel_rgba(ctx: &mut Context, x: u32, y: u32, color: &[u8; 4]) {
    ctx.render.canvas.write_pixel_blend(x, y, color);
}

/// Write color to pixel at (x, y)
///
/// Color: Non premultiplied RGBA \[0,1\]
///
/// Panics if trying to write outside canvas
pub fn write_pixel_rgba_f32(ctx: &mut Context, x: u32, y: u32, color: &[f32; 4]) {
    ctx.render.canvas.write_pixel_blend_f32(x, y, color);
}

/// Color at pixel (x, y)
///
/// Color: RGB \[0,255\]
///
/// Panics if trying to access outside canvas
pub fn pixel_rgb(ctx: &Context, x: u32, y: u32) -> [u8; 3] {
    ctx.render.canvas.pixel_rgb(x, y)
}

/// Color at pixel (x, y)
///
/// Color: RGB \[0,1\]
///
/// Panics if trying to access outside canvas
pub fn pixel_rgb_f32(ctx: &Context, x: u32, y: u32) -> [f32; 3] {
    ctx.render.canvas.pixel_rgb_f32(x, y)
}

/// Color at pixel (x, y)
///
/// Color: RGBA \[0,255\]
///
/// Panics if trying to access outside canvas
pub fn pixel_rgba(ctx: &Context, x: u32, y: u32) -> [u8; 4] {
    ctx.render.canvas.pixel_rgba(x, y)
}

/// Color at pixel (x, y)
///
/// Color: RGBA \[0,1\]
///
/// Panics if trying to access outside canvas
pub fn pixel_rgba_f32(ctx: &Context, x: u32, y: u32) -> [f32; 4] {
    ctx.render.canvas.pixel_rgba_f32(x, y)
}

/// Resizes the canvas
///
/// Clears screen to clear color
pub fn resize(ctx: &mut Context, width: u32, height: u32) {
    ctx.render.canvas.resize(width, height);
    ctx.render.screenshot_uploader.resize(width, height);
    ctx.render.resize_canvas_texture(width, height);
}

/// Clears all pixels to clear color
pub fn clear_screen(ctx: &mut Context, color: &[u8; 3]) {
    ctx.render.canvas.clear_screen(color);
}

/// Clears all pixels in canvas to clear color
///
/// Color: RGB \[0,1\]
pub fn clear_screen_f32(ctx: &mut Context, color: &[f32; 3]) {
    ctx.render.canvas.clear_screen_f32(color);
}

/// Canvas width
pub fn width(ctx: &Context) -> u32 {
    ctx.render.canvas.width
}

/// Canvas height
pub fn height(ctx: &Context) -> u32 {
    ctx.render.canvas.height
}

///
/// Tests
///

#[cfg(test)]
mod tests {
    use super::Canvas;

    #[test]
    #[should_panic]
    fn test_get_outside_canvas_panics() {
        let canvas = Canvas::new(256, 256);
        canvas.pixel_rgb(256, 10);
    }

    #[test]
    #[should_panic]
    fn test_write_outside_canvas_panics() {
        let mut canvas = Canvas::new(256, 256);
        canvas.write_pixel(256, 10, &[255, 255, 255]);
    }

    #[test]
    #[should_panic]
    fn test_invalid_rgba() {
        let mut canvas = Canvas::new(256, 256);
        canvas.write_pixel_f32(256, 10, &[1.0, 1.2, 0.0]);
    }

    #[test]
    fn test_write_then_get_pixel() {
        let color = [255, 0, 255];
        let mut canvas = Canvas::new(256, 256);
        canvas.write_pixel(11, 10, &color);

        let output = canvas.pixel_rgb(11, 10);

        assert_eq!(color, output);
    }

    #[test]
    fn test_resize() {
        let mut canvas = Canvas::new(256, 256);
        canvas.write_pixel(255, 200, &[255, 255, 255]);
        canvas.pixel_rgb(255, 200);

        canvas.resize(512, 512);
        canvas.write_pixel(500, 230, &[255, 255, 255]);
        canvas.pixel_rgb(500, 230);
    }
}
