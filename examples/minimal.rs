use pixel_renderer::{
    app::Callbacks,
    cmd::{canvas, keyboard, media},
    Context, KeyCode,
};

struct Game {}

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

impl Callbacks for Game {
    fn update(&mut self, ctx: &mut Context, _dt: f32) -> bool {
        canvas::clear_screen(ctx);

        for y in 75..175 {
            for x in 75..175 {
                canvas::write_pixel(ctx, x, y, &[0, 255, 255]);
            }
        }

        if keyboard::key_just_pressed(ctx, KeyCode::S) {
            let path = "examples/outputs/minimal.png";
            media::export_screenshot(ctx, path).unwrap();
            println!("saved screenshot to {}", path);
        }

        false
    }

    fn init(&self, ctx: &mut Context) {
        canvas::resize(ctx, WIDTH, HEIGHT);
    }
}

fn main() {
    let app = Game {};
    println!("S: to screenshot");
    pixel_renderer::app::run(app)
}
