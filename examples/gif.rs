use pixel_renderer::{canvas, input, media, Callbacks, Context};
use winit::event::VirtualKeyCode as KeyCode;

struct Game {
    box_x: f32,
    box_y: f32,
}

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;
const BOX_WIDTH: u32 = 10;
const BOX_HEIGHT: u32 = 10;
const BOX_VELOCITY: f32 = 1.0;

impl Callbacks for Game {
    fn update(&mut self, ctx: &mut Context, _dt: f32) -> bool {
        // Move box
        self.box_x += BOX_VELOCITY;
        if self.box_x > 240.0 {
            self.box_x = 10.0;
        }

        canvas::clear_screen(ctx);

        // Display box
        let box_x = self.box_x as u32;
        let box_y = self.box_y as u32;
        for y in box_y..box_y + BOX_HEIGHT {
            for x in box_x..box_x + BOX_WIDTH {
                canvas::write_pixel_rgb(ctx, x, y, &[0, 255, 255]);
            }
        }

        if input::key_pressed(ctx, KeyCode::R) {
            media::record_gif_frame(ctx);
            println!("recording frame");
        }
        if input::key_just_pressed(ctx, KeyCode::G) {
            let path = "examples/outputs/gif.gif";
            println!("creating gif");
            media::export_gif(ctx, path);
            media::clear_gif_frames(ctx);
            println!("saved gif to {}", path);
        }
        if input::key_just_pressed(ctx, KeyCode::S) {
            let path = "examples/outputs/gif.png";
            media::export_screenshot(ctx, path).unwrap();
            println!("saved screenshot to {}", path);
        }
        if input::key_just_pressed(ctx, KeyCode::C) {
            media::clear_gif_frames(ctx);
            println!("cleared frames");
        }

        false
    }

    fn init(&self, ctx: &mut Context) {
        canvas::resize(ctx, WIDTH, HEIGHT);
    }
}

fn main() {
    let app = Game {
        box_x: 10.0,
        box_y: 128.0 - 5.0,
    };
    println!("S: to screenshot");
    println!("R: to record frames");
    println!("G: to create gif from frames");
    println!("C: to clear frames");
    pixel_renderer::run(app)
}
