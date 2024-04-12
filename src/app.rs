use crate::{
    context::Context, input::InputContext, render::RenderContext, time::TimeContext, window,
};
use winit::event_loop::EventLoop;

/// User callbacks
///
/// Will be called from the main event loop
pub trait Callbacks {
    /// Called before initalization
    #[allow(unused_variables)]
    fn init(&self, ctx: &mut Context) {}

    /// Called once per frame before render
    ///
    /// Return value determines wether to exit game or not
    #[allow(unused_variables)]
    fn update(&mut self, ctx: &mut Context) -> bool {
        false
    }
}

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
        // Update time
        ctx.time.update_time();

        // Update callback
        if self.callbacks.update(ctx) {
            return true;
        }

        // Reset input
        ctx.input.keyboard.save_keys();
        ctx.input.keyboard.save_modifiers();
        ctx.input.mouse.save_buttons();
        ctx.input.mouse.set_mouse_delta((0.0, 0.0));

        false
    }
}

/// Runs the event loop
///
/// Calls user defined functions thorugh Callback trait
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
