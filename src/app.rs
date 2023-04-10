use crate::{canvas::Canvas, render};

/// User callbaks
pub trait Callbacks {
    /// Called once per frame before render
    /// Return value determines wether to exit game or not
    fn update(&mut self, canvas: &mut Canvas) -> bool {
        false
    }

    fn dimensions(&self) -> (u32, u32);
}

/// Main App
/// Contains all data to run application
pub struct App<C: Callbacks> {
    pub(crate) callbacks: C,
}

/// Functions implemented on App
impl<C> App<C>
where
    C: Callbacks + 'static,
{
    /// Main loop which is called from window event loop
    /// Returns true if app should exit
    pub(crate) fn update(&mut self, canvas: &mut Canvas) -> bool {
        // Update callback
        if self.callbacks.update(canvas) {
            return true;
        }
        false
    }
}

/// Runts event loop with callbacks
/// Calls back to user defined functions thorugh Callback trait
pub fn run<C>(callbacks: C)
where
    C: Callbacks + 'static,
{
    // Run app
    let app = App { callbacks };
    pollster::block_on(render::run_window(app));
}
