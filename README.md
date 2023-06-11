# pixel-renderer

Simple pixel renderer using a pixel buffer which is then converted to a texture and rendered using wgpu. Main focus is simplicity
and being able to easily play around with.

## Minimal example

```rust
const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

struct Game {}

impl Callbacks for Game {
    fn update(&mut self, ctx: &mut Context, _dt: f32) -> bool {
        canvas::clear_screen(ctx);

        let (px, py) = (75, 75);
        let (wx, wy) = (100, 100);
        for y in 0..wx {
            for x in 0..wy {
                canvas::write_pixel(ctx, x + px, y + py, &[0, 255, 255]);
            }
        }

        // Return true to exit
        false
    }

    fn init(&self, ctx: &mut Context) {
        canvas::resize(ctx, WIDTH, HEIGHT);
    }
}

fn main() {
    let app = Game {};
    pixel_renderer::app::run(app)
}

```

See full example in examples/minimal.rs
