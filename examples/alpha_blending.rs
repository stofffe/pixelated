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

        canvas.set_clear_color(&[255, 255, 255]);
        canvas.clear_screen();

        // Write
        for y in 25..125 {
            for x in 125..225 {
                canvas.write_pixel_blend(x, y, &[255, 0, 0, 255 / 2]);
            }
        }

        // Write opaque green pixel
        for y in 50..150 {
            for x in 50..150 {
                canvas.write_pixel_blend(x, y, &[0, 255, 0, 255 / 2]);
            }
        }

        // Write aplha blue pixel
        for y in 100..200 {
            for x in 100..200 {
                canvas.write_pixel_blend(x, y, &[0, 0, 255, 255 / 2]);
            }
        }

        if keyboard.key_just_pressed(KeyCode::S) {
            let path = "examples/outputs/alpha_blending.png";
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
