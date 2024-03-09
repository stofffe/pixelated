use pixel_renderer::{
    app::Callbacks,
    canvas, input, media,
    window::{self},
    Context, KeyCode,
};

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

struct Game {}

impl Callbacks for Game {
    fn init(&self, ctx: &mut Context) {
        canvas::resize(ctx, WIDTH, HEIGHT);
        window::set_resizeable(ctx, true);
    }

    fn update(&mut self, ctx: &mut Context, _dt: f32) -> bool {
        canvas::clear_screen(ctx);

        let (px, py) = (75, 75);
        let (wx, wy) = (100, 100);
        for y in 0..wx {
            for x in 0..wy {
                canvas::write_pixel(ctx, x + px, y + py, &[0, 255, 255]);
            }
        }

        if input::key_just_pressed(ctx, KeyCode::S) {
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
    pixel_renderer::app::run(app);
}
