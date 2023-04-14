use pixel_renderer::{
    app::{Callbacks, Config},
    context::Context,
    input::KeyCode,
};

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
        let canvas = &mut ctx.render.canvas;
        let keyboard = &ctx.input.keyboard;

        // Move box
        self.box_x += BOX_VELOCITY;
        if self.box_x > 240.0 {
            self.box_x = 10.0;
        }

        canvas.clear_screen();

        // Display box
        let box_x = self.box_x as u32;
        let box_y = self.box_y as u32;
        for y in box_y..box_y + BOX_HEIGHT {
            for x in box_x..box_x + BOX_WIDTH {
                canvas.write_pixel(x, y, &[0, 255, 255, 255]);
            }
        }

        if keyboard.key_pressed(KeyCode::R) {
            ctx.render.gif_uploader.record(canvas);
            println!("recording frame");
        }
        if keyboard.key_just_pressed(KeyCode::G) {
            let path = "examples/outputs/gif.gif";
            println!("creating gif");
            ctx.render.gif_uploader.export_to_gif(path);
            ctx.render.gif_uploader.clear();
            println!("saved gif to {}", path);
        }
        if keyboard.key_just_pressed(KeyCode::S) {
            let path = "examples/outputs/gif.png";
            ctx.render
                .screenshot_uploader
                .export_to_file(canvas, path)
                .unwrap();
            println!("saved screenshot to {}", path);
        }
        if keyboard.key_just_pressed(KeyCode::C) {
            ctx.render.gif_uploader.clear();
            println!("cleared frames");
        }

        false
    }
}

fn main() {
    let app = Game {
        box_x: 10.0,
        box_y: 128.0 - 5.0,
    };
    let config = Config {
        width: WIDTH,
        height: HEIGHT,
        resizeable: false,
    };
    println!("S: to screenshot");
    println!("R: to record frames");
    println!("G: to create gif from frames");
    println!("C: to clear frames");
    pixel_renderer::app::run(app, config)
}
