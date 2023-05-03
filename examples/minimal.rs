use pixel_renderer::{
    app::{Callbacks, Config},
    context::Context,
    input::KeyCode,
};

struct Game {}

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

impl Callbacks for Game {
    fn update(&mut self, ctx: &mut Context, _dt: f32) -> bool {
        let canvas = &mut ctx.render.canvas;
        let keyboard = &ctx.input.keyboard;

        canvas.clear_screen();

        for y in 75..175 {
            for x in 75..175 {
                canvas.write_pixel(x, y, &[0, 255, 255]);
            }
        }

        if keyboard.key_just_pressed(KeyCode::S) {
            let path = "examples/outputs/minimal.png";
            canvas.export_screenshot(path).unwrap();
            println!("saved screenshot to {}", path);
        }

        false
    }

    fn config(&self) -> Config {
        Config {
            canvas_width: WIDTH,
            canvas_height: HEIGHT,
            ..Default::default()
        }
    }
}

fn main() {
    let app = Game {};
    println!("S: to screenshot");
    pixel_renderer::app::run(app)
}
