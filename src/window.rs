#![allow(clippy::single_match, clippy::collapsible_match)]

use crate::{
    app::{App, Callbacks},
    context::Context,
};
use winit::{
    event::{DeviceEvent, ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub(crate) fn new_window() -> (winit::window::Window, winit::event_loop::EventLoop<()>) {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .build(&event_loop)
        .expect("could not build window");

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
        } => {
            if window_id == ctx.render.window.id() {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        ctx.render.resize_window(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        ctx.render.resize_window(**new_inner_size);
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        ctx.input.mouse.set_pos(position.x, position.y);
                    }
                    WindowEvent::CursorLeft { .. } => ctx.input.mouse.set_on_screen(false),
                    WindowEvent::CursorEntered { .. } => ctx.input.mouse.set_on_screen(true),
                    WindowEvent::MouseInput { state, button, .. } => match state {
                        ElementState::Pressed => ctx.input.mouse.press_button(*button),
                        ElementState::Released => ctx.input.mouse.release_button(*button),
                    },
                    WindowEvent::MouseWheel { delta, .. } => {
                        let (x, y) = match delta {
                            winit::event::MouseScrollDelta::LineDelta(x, y) => {
                                (*x as f64, *y as f64)
                            }
                            winit::event::MouseScrollDelta::PixelDelta(pos) => (pos.x, pos.y),
                        };
                        ctx.input.mouse.set_scroll_delta((x, y));
                    }
                    WindowEvent::ModifiersChanged(modifiers) => {
                        ctx.input.keyboard.modifiers_changed(*modifiers)
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
                }
            }
        }
        Event::DeviceEvent { ref event, .. } => match event {
            DeviceEvent::MouseMotion { delta } => ctx.input.mouse.set_mouse_delta(*delta),
            _ => {}
        },
        Event::RedrawRequested(window_id) if window_id == ctx.render.window.id() => {
            let new_size = ctx.render.window.inner_size();
            match ctx.render.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => ctx.render.resize_window(new_size),
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

//
// Commands
//

/// Enables/Disables vsync
pub fn set_vsync(ctx: &mut Context, vsync: bool) {
    let present_mode = if vsync {
        wgpu::PresentMode::AutoVsync
    } else {
        wgpu::PresentMode::AutoNoVsync
    };
    ctx.render.reconfigure_present_mode(present_mode);
}

/// Enables/Disables borderless windowed mode
pub fn set_fullscreen(ctx: &mut Context, fullscreen: bool) {
    let fullscreen_mode = if fullscreen {
        Some(winit::window::Fullscreen::Borderless(None))
    } else {
        None
    };
    ctx.render.window.set_fullscreen(fullscreen_mode);
}

/// Enables/Disables window resizing
pub fn set_resizeable(ctx: &mut Context, resizable: bool) {
    ctx.render.window.set_resizable(resizable);
}

/// Sets the inner size of the window
pub fn set_size(ctx: &mut Context, size: (u32, u32)) {
    ctx.render
        .window
        .set_inner_size(winit::dpi::PhysicalSize::new(size.0, size.1));
}

/// Enables/Disables the cursor
/// If disabled: Turns off cursor and locks cursor to middle of window
pub fn set_cursor_enabled(ctx: &mut Context, enabled: bool) {
    // TODO handle error
    ctx.render.window.set_cursor_visible(enabled);
    let grab_mode = if enabled {
        winit::window::CursorGrabMode::None
    } else {
        winit::window::CursorGrabMode::Locked
    };
    ctx.render
        .window
        .set_cursor_grab(grab_mode)
        .expect("could not set cursor grab mode");
}
