use pixel_renderer::{app::Callbacks, canvas::Canvas};

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

struct Game {}

impl Callbacks for Game {
    fn update(&mut self, canvas: &mut Canvas) -> bool {
        let color: [u8; 4] = [0, 255, 255, 255];

        for y in (40..100).step_by(4) {
            for x in (50..80).step_by(4) {
                canvas.write_pixel(y, x, &color);
            }
        }
        false
    }
    fn dimensions(&self) -> (u32, u32) {
        (WIDTH, HEIGHT)
    }
}

fn main() {
    let app = Game {};
    pixel_renderer::app::run(app)
}
