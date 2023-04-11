pub struct Canvas {
    pub(crate) pixels: Vec<u8>,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) clear_color: [u8; 4],
}

impl Canvas {
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

    pub fn capacity(&self) -> u32 {
        self.width * self.height * 4
    }

    pub fn clear_screen(&mut self) {
        for pixel in self.pixels.chunks_mut(4) {
            pixel[0] = self.clear_color[0];
            pixel[1] = self.clear_color[1];
            pixel[2] = self.clear_color[2];
            pixel[3] = self.clear_color[3];
        }
    }
}
