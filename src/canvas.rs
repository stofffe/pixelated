pub struct Canvas {
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub clear_color: [u8; 4],
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        // Pixels
        let capacity = width * height * 4;
        let mut pixels = Vec::with_capacity(capacity as usize);
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
        let index = (y * self.width + x) as usize;
        self.pixels[index] = color[0];
        self.pixels[index + 1] = color[1];
        self.pixels[index + 2] = color[2];
        self.pixels[index + 3] = color[3];
    }

    pub fn clear_screen(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = (y * self.width + x) as usize;
                self.pixels[index] = self.clear_color[0];
                self.pixels[index + 1] = self.clear_color[1];
                self.pixels[index + 2] = self.clear_color[2];
                self.pixels[index + 3] = self.clear_color[3];
            }
        }
    }
}
