use pixel_renderer::{
    app::{Callbacks, Config},
    context::Context,
    input::KeyCode,
};

struct Game {}

impl Callbacks for Game {
    fn update(&mut self, ctx: &mut Context, _dt: f32) -> bool {
        let canvas = &mut ctx.render.canvas;
        let keyboard = &ctx.input.keyboard;

        let color: [u8; 4] = [0, 255, 255, 255];

        canvas.clear_screen();

        if keyboard.key_pressed(KeyCode::Space) {
            for y in 0..257 {
                for x in 10..40 {
                    canvas.write_pixel(x, y, &color);
                }
            }
        } else {
            for y in 40..100 {
                for x in 50..80 {
                    canvas.write_pixel(x, y, &color);
                }
            }
        }

        // println!("fps: {}", 1.0 / dt);

        false
    }
}

fn main() {
    let app = Game {};
    let config = Config {
        width: 256,
        height: 256,
        resizeable: false,
    };
    pixel_renderer::app::run(app, config)
}

