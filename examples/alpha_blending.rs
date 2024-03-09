use pixelated::{canvas, input, media, Callbacks, Context};
use winit::event::VirtualKeyCode as KeyCode;

struct Game {}

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

impl Callbacks for Game {
    fn init(&self, ctx: &mut Context) {
        canvas::resize(ctx, WIDTH, HEIGHT);
    }

    fn update(&mut self, ctx: &mut Context, _dt: f32) -> bool {
        canvas::set_clear_color(ctx, &[255, 255, 255]);
        canvas::clear_screen(ctx);

        // Write
        for y in 25..125 {
            for x in 125..225 {
                canvas::write_pixel_rgba(ctx, x, y, &[255, 0, 0, 255 / 2]);
            }
        }

        // Write opaque green pixel
        for y in 50..150 {
            for x in 50..150 {
                canvas::write_pixel_rgba(ctx, x, y, &[0, 255, 0, 255 / 2]);
            }
        }

        // Write aplha blue pixel
        for y in 100..200 {
            for x in 100..200 {
                canvas::write_pixel_rgba(ctx, x, y, &[0, 0, 255, 255 / 2]);
            }
        }

        if input::key_just_pressed(ctx, KeyCode::S) {
            let path = "examples/outputs/alpha_blending.png";
            media::export_screenshot(ctx, path).unwrap();
            println!("saved screenshot to {}", path);
        }

        false
    }
}

fn main() {
    let app = Game {};
    println!("S: to screenshot");
    pixelated::run(app);
}
