const DEFAULT_CLEAR_COLOR: [u8; 4] = [0, 0, 0, 255];

/// Represent the screen of pixels
pub struct Canvas {
    pub(crate) pixels: Vec<u8>,
    pub(crate) width: u32,
    pub(crate) height: u32,
    clear_color: [u8; 4],
}

impl Canvas {
    /// Create new canvas with specified width and height
    pub fn new(width: u32, height: u32) -> Self {
        let capacity = width * height * 4;
        let mut pixels = Vec::new();
        pixels.resize(capacity as usize, 0);

        Self {
            pixels,
            width,
            height,
            clear_color: DEFAULT_CLEAR_COLOR,
        }
    }

    /// Resizes the canvas
    /// Clears screen to ```clear_color```
    pub fn resize(&mut self, width: u32, height: u32) {
        let capacity = width * height * 4;

        self.pixels.resize(capacity as usize, 0);
        self.width = width;
        self.height = height;

        self.clear_screen();
    }

    /// Clone pixel buffer
    pub fn get_pixel_buffer(&self) -> Vec<u8> {
        self.pixels.clone()
    }

    /// Get pixel data for a coordianate
    /// Panics if trying to access outside canvas
    pub fn get_pixel(&self, x: u32, y: u32) -> [u8; 3] {
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
    pub fn get_pixel_alpha(&self, x: u32, y: u32) -> [u8; 4] {
        assert_pixel(x, y, self.width, self.height);

        let index = (y * 4 * self.width + x * 4) as usize;
        [
            self.pixels[index],
            self.pixels[index + 1],
            self.pixels[index + 2],
            self.pixels[index + 3],
        ]
    }

    /// Write pixel data to a coordinate (r,g,b,a)
    /// Overwrites previous pixel
    pub fn write_pixel(&mut self, x: u32, y: u32, color: &[u8; 3]) {
        assert_pixel(x, y, self.width, self.height);

        let index = (y * 4 * self.width + x * 4) as usize;
        self.pixels[index] = color[0];
        self.pixels[index + 1] = color[1];
        self.pixels[index + 2] = color[2];
        self.pixels[index + 3] = 255;
    }

    /// Write pixel data to a coordinate (r,g,b,a)
    /// Non premultiplied alpha blending
    pub fn write_pixel_blend(&mut self, x: u32, y: u32, color: &[u8; 4]) {
        assert_pixel(x, y, self.width, self.height);

        // Alpha blending where a is over b
        // https://en.wikipedia.org/wiki/Alpha_compositing
        let prev_color = self.get_pixel_alpha(x, y);
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
    /// Panics if trying to write outside canvas
    /// RGBA must be in range [0,1]
    pub fn write_pixel_f32(&mut self, x: u32, y: u32, color: &[f32; 4]) {
        assert_pixel(x, y, self.width, self.height);
        assert_rgba(color);

        let index = (y * 4 * self.width + x * 4) as usize;
        self.pixels[index] = (color[0] * 255.0) as u8;
        self.pixels[index + 1] = (color[1] * 255.0) as u8;
        self.pixels[index + 2] = (color[2] * 255.0) as u8;
        self.pixels[index + 3] = (color[3] * 255.0) as u8;
    }

    /// Clears all pixels in canvas to clear color
    pub fn clear_screen(&mut self) {
        for pixel in self.pixels.chunks_mut(4) {
            pixel[0] = self.clear_color[0];
            pixel[1] = self.clear_color[1];
            pixel[2] = self.clear_color[2];
            pixel[3] = self.clear_color[3];
        }
    }

    /// Set canvas clear color (r,g,b,a)
    pub fn set_clear_color(&mut self, color: &[u8; 3]) {
        self.clear_color[0] = color[0];
        self.clear_color[1] = color[1];
        self.clear_color[2] = color[2];
        self.clear_color[3] = 255;
    }

    /// Set canvas clear color (r,g,b)
    /// Values must lie in range [0,1]
    pub fn set_clear_color_f32(&mut self, color: &[f32; 3]) {
        assert_rgb(color);

        self.clear_color[0] = (color[0] * 255.0) as u8;
        self.clear_color[1] = (color[1] * 255.0) as u8;
        self.clear_color[2] = (color[2] * 255.0) as u8;
        self.clear_color[3] = 255;
    }

    /// Get canvas capacity
    pub fn capacity(&self) -> u32 {
        self.width * self.height * 4
    }

    /// Get canvas width
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get canvas height
    pub fn height(&self) -> u32 {
        self.height
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

// Tests
#[cfg(test)]
mod tests {
    use super::Canvas;

    #[test]
    #[should_panic]
    fn test_get_outside_canvas_panics() {
        let canvas = Canvas::new(256, 256);
        canvas.get_pixel(256, 10);
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
        canvas.write_pixel_f32(256, 10, &[1.0, 1.2, 0.0, 1.0]);
    }

    #[test]
    fn test_write_then_get_pixel() {
        let color = [255, 0, 255];
        let mut canvas = Canvas::new(256, 256);
        canvas.write_pixel(11, 10, &color);

        let output = canvas.get_pixel(11, 10);

        assert_eq!(color, output);
    }

    #[test]
    fn test_resize() {
        let mut canvas = Canvas::new(256, 256);
        canvas.write_pixel(255, 200, &[255, 255, 255]);
        canvas.get_pixel(255, 200);

        canvas.resize(512, 512);
        canvas.write_pixel(500, 230, &[255, 255, 255]);
        canvas.get_pixel(500, 230);
    }
}

// /// Custom iterator for canvas
// /// Returns (x, y) position and pixel data
// pub struct CanvasIterator<'a> {
//     canvas: &'a mut Canvas,
//     x: u32,
//     y: u32,
// }
//
// impl<'a> Iterator for CanvasIterator<'a> {
//     type Item = (u32, u32, &'a mut (u8, u8, u8, u8));
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let index = (self.y * 4 * self.canvas.width + self.x * 4) as usize;
//
//         if index as u32 + 3 >= self.canvas.capacity() {
//             return None;
//         }
//
//         let pixel = &mut self.canvas.pixels[index..(index + 4)];
//         let pixels = &mut (
//             pixel[index],
//             pixel[index + 1],
//             pixel[index + 2],
//             pixel[index + 3],
//         );
//         let result = (self.x, self.y, pixels);
//
//         if self.y >= self.canvas.width {
//             self.x = 0;
//             self.y += 1;
//         } else {
//             self.x += 1;
//         }
//
//         Some(result)
//     }
// }
//
// impl<'a> IntoIterator for &'a mut Canvas {
//     type Item = (u32, u32, &'a mut (u8, u8, u8, u8));
//     type IntoIter = CanvasIterator<'a>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         CanvasIterator {
//             canvas: self,
//             x: 0,
//             y: 0,
//         }
//     }
// }

// TODO implement iterator?
// for (x,y,pixel) in canvas.iter() {
//
// }

// TODO not super fast
// Export current canvas to the specified path
// pub fn export_to_file(&self, path: &str) -> ImageResult<()> {
//     let mut img  = RgbaImage::new(self.width, self.height);
//
//     img.copy_from_slice(&self.pixels);
//
//     img.save(path)
// }

// /// Set canvas clear color (r,g,b)
// /// Values must lie in range (0,1) (inclusive)
// pub fn set_clear_color_rgb(&mut self, color: &[f32; 3]) {
//     assert!(
//         (0.0..=1.0).contains(&color[0]),
//         "r ({}) not in range [0,1]",
//         color[0]
//     );
//     assert!(
//         (0.0..=1.0).contains(&color[1]),
//         "g ({}) not in range [0,1]",
//         color[1]
//     );
//     assert!(
//         (0.0..=1.0).contains(&color[2]),
//         "b ({}) not in range [0,1]",
//         color[2]
//     );
//
//     self.clear_color[0] = (color[0] * 255.0) as u8;
//     self.clear_color[1] = (color[1] * 255.0) as u8;
//     self.clear_color[2] = (color[2] * 255.0) as u8;
// }

// assert!(
//     (0.0..=1.0).contains(&color[0]),
//     "r ({}) not in range [0,1]",
//     color[0]
// );
// assert!(
//     (0.0..=1.0).contains(&color[1]),
//     "g ({}) not in range [0,1]",
//     color[1]
// );
// assert!(
//     (0.0..=1.0).contains(&color[3]),
//     "b ({}) not in range [0,1]",
//     color[2]
// );
// assert!(
//     (0.0..=1.0).contains(&color[3]),
//     "a ({}) not in range [0,1]",
//     color[3]
// );
