use pixelated::{canvas, input};
use pixelated::{Callbacks, Context};
use winit::event::MouseButton;

const WIDTH: u32 = 50;
const HEIGHT: u32 = 50;

struct Conways {
    width: u32,
    height: u32,
    cells: Vec<bool>,
    buffer_cells: Vec<bool>,
}

impl Conways {
    fn new(width: u32, height: u32) -> Self {
        let len = width * height;
        let cells = vec![false; len as usize];
        let buffer_cells = cells.clone();

        Self {
            width,
            height,
            cells,
            buffer_cells,
        }
    }

    fn update(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let alive = self.is_alive(x, y);
                let count = self.count_neighbours(x, y);

                match (alive, count) {
                    (true, 0..=1) => self.kill_buffer_cell(x, y),
                    (true, 4..=8) => self.kill_buffer_cell(x, y),
                    (false, 3) => self.create_buffer_cell(x, y),
                    _ => {}
                }
            }
        }
        self.cells = self.buffer_cells.clone();
    }

    fn is_alive(&self, x: u32, y: u32) -> bool {
        let index = (x + y * self.width) as usize;
        self.cells[index]
    }
    fn create_cell(&mut self, x: u32, y: u32) {
        let index = (x + y * self.width) as usize;
        self.cells[index] = true;
        self.buffer_cells[index] = true;
    }
    fn kill_cell(&mut self, x: u32, y: u32) {
        let index = (x + y * self.width) as usize;
        self.cells[index] = false;
        self.buffer_cells[index] = false;
    }

    fn create_buffer_cell(&mut self, x: u32, y: u32) {
        let index = (x + y * self.width) as usize;
        self.buffer_cells[index] = true;
    }
    fn kill_buffer_cell(&mut self, x: u32, y: u32) {
        let index = (x + y * self.width) as usize;
        self.buffer_cells[index] = false;
    }

    fn count_neighbours(&self, x: u32, y: u32) -> u32 {
        let mut count = 0;
        let min_y = if y == 0 { 0 } else { y - 1 };
        let min_x = if x == 0 { 0 } else { x - 1 };
        let max_y = if y == HEIGHT - 1 { HEIGHT - 1 } else { y + 1 };
        let max_x = if x == WIDTH - 1 { WIDTH - 1 } else { x + 1 };

        for y2 in min_y..=max_y {
            for x2 in min_x..=max_x {
                if self.is_alive(x2, y2) && (x2, y2) != (x, y) {
                    count += 1
                }
            }
        }

        count
    }
}

impl Callbacks for Conways {
    fn init(&self, ctx: &mut Context) {
        canvas::resize(ctx, WIDTH, HEIGHT);
    }

    fn update(&mut self, ctx: &mut Context) -> bool {
        let mouse_pos = input::mouse_pos_pixel(ctx);

        // Input and update
        if input::mouse_on_screen(ctx) {
            if input::mouse_button_pressed(ctx, MouseButton::Left) {
                self.create_cell(mouse_pos.0, mouse_pos.1);
            }
            if input::mouse_button_pressed(ctx, MouseButton::Right) {
                self.kill_cell(mouse_pos.0, mouse_pos.1);
            }
        }

        if input::key_pressed(ctx, input::KeyCode::Space) {
            self.update();
        }

        // Draw
        canvas::clear_screen(ctx);
        for y in 0..self.height {
            for x in 0..self.width {
                if self.is_alive(x, y) {
                    canvas::write_pixel_rgb(ctx, x, y, &[255, 255, 255]);
                }
            }
        }
        if input::mouse_on_screen(ctx) {
            canvas::write_pixel_rgba(ctx, mouse_pos.0, mouse_pos.1, &[255, 255, 255, 255 / 2]);
        }

        false
    }
}

fn main() {
    let app = Conways::new(WIDTH, HEIGHT);
    println!("Space: to advance");
    println!("LMB: to place");
    println!("RMB: to remove");
    pixelated::run(app)
}
