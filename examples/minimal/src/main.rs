use pixel_renderer::{
    app::{Callbacks, Config},
    canvas::Canvas,
    input::{InputContext, KeyCode},
};

struct Game {}

impl Callbacks for Game {
    fn update(&mut self, canvas: &mut Canvas, input: &InputContext, dt: f32) -> bool {
        let color: [u8; 4] = [0, 255, 255, 255];

        canvas.clear_screen();

        if input.keyboard.key_pressed(KeyCode::Space) {
            for y in (200..1024).step_by(4) {
                for x in (50..80).step_by(4) {
                    canvas.write_pixel(x, y, &color);
                }
            }
        } else {
            for y in (40..100).step_by(4) {
                for x in (50..80).step_by(4) {
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
    };
    pixel_renderer::app::run(app, config)
}
