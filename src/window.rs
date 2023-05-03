use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{
    app::{App, Callbacks, Config},
    context::Context,
};

pub(crate) fn new_window(
    config: &Config,
) -> (winit::window::Window, winit::event_loop::EventLoop<()>) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(winit::dpi::LogicalSize::new(
            config.canvas_width,
            config.canvas_height,
        ))
        .with_resizable(config.resizeable)
        .build(&event_loop)
        .unwrap();

    (window, event_loop)
}

pub(crate) async fn run_window<C: Callbacks + 'static>(
    event_loop: EventLoop<()>,
    mut app: App<C>,
    mut ctx: Context,
) {
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == ctx.render.window.id() => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(physical_size) => {
                ctx.render.resize(*physical_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                ctx.render.resize(**new_inner_size);
            }
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(keycode) = input.virtual_keycode {
                    match input.state {
                        ElementState::Pressed => ctx.input.keyboard.set_key(keycode),
                        ElementState::Released => ctx.input.keyboard.release_key(keycode),
                    }
                }
            }
            _ => {}
        },
        Event::RedrawRequested(window_id) if window_id == ctx.render.window.id() => {
            match ctx.render.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => ctx.render.resize(ctx.render.size),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            if app.update(&mut ctx) {
                *control_flow = ControlFlow::Exit;
            }
            ctx.render.window.request_redraw();
        }
        _ => {}
    });
}
