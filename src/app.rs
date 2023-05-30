use crate::{
    context::Context, input::InputContext, render::RenderContext, time::TimeContext, window,
};
use winit::event_loop::EventLoop;

/// User callbaks
pub trait Callbacks {
    /// Called before initalization
    fn init(&self, _ctx: &mut Context) {}

    /// Called once per frame before render
    /// Return value determines wether to exit game or not
    /// dt: Time since last frame in seconds
    fn update(&mut self, _ctx: &mut Context, _dt: f32) -> bool {
        false
    }
}

/// Config for initalizing window and game loop
// pub struct Config {
//     pub canvas_width: u32,
//     pub canvas_height: u32,
//     pub window_width: u32,
//     pub window_height: u32,
//     pub resizeable: bool,
//     pub fullscreen: bool,
//     pub scale_canvas_to_window: bool,
// }

// impl Default for Config {
//     fn default() -> Self {
//         Self {
//             canvas_width: 512,
//             canvas_height: 512,
//             window_height: 512,
//             window_width: 512,
//             resizeable: false,
//             fullscreen: false,
//             scale_canvas_to_window: false,
//         }
//     }
// }

/// Main App
/// Contains all data to run application
pub(crate) struct App<C: Callbacks> {
    pub(crate) callbacks: C,
}

/// Functions implemented on App
impl<C> App<C>
where
    C: Callbacks + 'static,
{
    /// Main loop which is called from window event loop
    /// Returns true if app should exit
    pub(crate) fn update(&mut self, ctx: &mut Context) -> bool {
        let dt = ctx.time.update_time();

        // Update callback
        if self.callbacks.update(ctx, dt) {
            return true;
        }

        ctx.input.keyboard.save_keys();
        ctx.input.mouse.save_buttons();
        ctx.input.mouse.set_mouse_change((0.0, 0.0));

        false
    }
}

/// Runs the event loop
/// Calls back to user defined functions thorugh Callback trait
pub fn run<C: Callbacks + 'static>(callbacks: C)
where
    C: Callbacks + 'static,
{
    env_logger::init();
    let app = App { callbacks };

    let (mut ctx, event_loop) = pollster::block_on(build_context());

    app.callbacks.init(&mut ctx);

    pollster::block_on(window::run_window(event_loop, app, ctx));
}

// TODO contex builder?
async fn build_context() -> (Context, EventLoop<()>) {
    let (window, event_loop) = window::new_window();

    let time = TimeContext::default();
    let input = InputContext::default();
    let render = RenderContext::new(window).await;
    let context = Context {
        render,
        time,
        input,
    };

    (context, event_loop)
}
