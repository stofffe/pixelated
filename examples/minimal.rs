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

        for y in 57..157 {
            for x in 10..40 {
                canvas.write_pixel(x, y, &[0, 255, 255, 255]);
            }
        }

        if keyboard.key_just_pressed(KeyCode::S) {
            let path = "examples/outputs/minimal.png";
            ctx.render
                .screenshot_uploader
                .export_to_file(canvas, path)
                .unwrap();
            println!("saved screenshot to {}", path);
        }

        false
    }
}

fn main() {
    let app = Game {};
    let config = Config {
        width: WIDTH,
        height: HEIGHT,
        resizeable: false,
    };
    println!("S: to screenshot");
    pixel_renderer::app::run(app, config)
}
