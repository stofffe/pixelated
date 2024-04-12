use pixelated::{canvas, input, media, window, Callbacks, Context};

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

struct Game {}

impl Callbacks for Game {
    fn init(&self, ctx: &mut Context) {
        canvas::resize(ctx, WIDTH, HEIGHT);
        window::window_ref(ctx).set_resizable(true);
    }

    fn update(&mut self, ctx: &mut Context) -> bool {
        canvas::clear_screen(ctx, &[0, 0, 0]);

        let (px, py) = (75, 75);
        let (wx, wy) = (100, 100);
        for y in 0..wx {
            for x in 0..wy {
                canvas::write_pixel_rgb(ctx, x + px, y + py, &[0, 255, 255]);
            }
        }

        if input::key_just_pressed(ctx, input::KeyCode::KeyS) {
            let path = "examples/outputs/minimal.png";
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
