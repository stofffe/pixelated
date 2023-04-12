/// Represent the screen of pixels
pub struct Canvas {
    pub(crate) pixels: Vec<u8>,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) clear_color: [u8; 4],
}

impl Canvas {
    /// Create new canvas with specified width and height
    pub fn new(width: u32, height: u32) -> Self {
        // Pixels
        let capacity = width * height * 4;
        let mut pixels = Vec::new();
        pixels.resize(capacity as usize, 0);

        // Clear color
        let clear_color = [0, 0, 0, 255];

        Self {
            pixels,
            clear_color,
            width,
            height,
        }
    }

    /// Clone pixel buffer
    pub fn get_pixel_buffer(&self) -> Vec<u8> {
        self.pixels.clone()
    }

    /// Get pixel data for a coordianate
    /// Panics if trying to access outside canvas
    pub fn get_pixel(&self, x: u32, y: u32) -> [u8; 4] {
        if x > self.width - 1 || y > self.height - 1 {
            log::error!(
                "trying to get pixel outside screen, x: {}, y: {}, width: 0-{} (exclusive), height: 0-{} (exclusive)",
                x,
                y,
                self.width ,
                self.height 
            );
            panic!() // TODO improve info
        }

        let index = (y * 4 * self.width + x * 4) as usize;
        [self.pixels[index], self.pixels[index+1], self.pixels[index+2], self.pixels[index+3]]
    }

    /// Write pixel data to a coordinate
    /// Panics if trying to write outside canvas
    pub fn write_pixel(&mut self, x: u32, y: u32, color: &[u8; 4]) {
        if x > self.width - 1 || y > self.height - 1 {
            log::error!(
                "trying to draw outside screen, x: {}, y: {}, width: 0-{} (exclusive), height: 0-{} (exclusive)",
                x,
                y,
                self.width ,
                self.height 
            );
            panic!() // TODO improve info
        }

        let index = (y * 4 * self.width + x * 4) as usize;
        self.pixels[index] = color[0];
        self.pixels[index + 1] = color[1];
        self.pixels[index + 2] = color[2];
        self.pixels[index + 3] = color[3];
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

    /// Get canvas capacity
    pub fn capacity(&self) -> u32 {
        self.width * self.height * 4
    }

    /// Set canvas clear color
    pub fn set_clear_color(&mut self, color: [u8; 4]) {
        self.clear_color = color;
    }
}


// TODO not super fast
// Export current canvas to the specified path
// pub fn export_to_file(&self, path: &str) -> ImageResult<()> {
//     let mut img  = RgbaImage::new(self.width, self.height);
//
//     img.copy_from_slice(&self.pixels);
//
//     img.save(path)
// }
