#![allow(clippy::single_match, clippy::collapsible_match)]

use crate::{
    app::{App, Callbacks},
    context::Context,
};
use winit::{
    event::{DeviceEvent, ElementState, Event, WindowEvent},
    event_loop::EventLoop,
    keyboard::PhysicalKey,
    window::WindowBuilder,
};

pub(crate) fn new_window() -> (winit::window::Window, winit::event_loop::EventLoop<()>) {
    let event_loop = EventLoop::new().expect("could not create event loop");

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
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let res = event_loop.run(move |event, target| match event {
        Event::WindowEvent { ref event, .. } => match event {
            WindowEvent::RedrawRequested => {
                if app.update(&mut ctx) {
                    target.exit();
                }
                match ctx.render.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => {
                        ctx.render.resize_window(ctx.render.window.inner_size())
                    }
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        target.exit();
                    }
                    Err(e) => eprintln!("{:?}", e),
                }
            }

            WindowEvent::CloseRequested => {
                target.exit();
            }
            WindowEvent::Resized(new_size) => {
                ctx.render.resize_window(*new_size);
            }
            // Mouse input
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
                    winit::event::MouseScrollDelta::LineDelta(x, y) => (*x as f64, *y as f64),
                    winit::event::MouseScrollDelta::PixelDelta(pos) => (pos.x, pos.y),
                };
                ctx.input.mouse.set_scroll_delta((x, y));
            }
            // Keyboard input
            WindowEvent::ModifiersChanged(modifiers) => {
                ctx.input.keyboard.modifiers_changed(modifiers)
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let (key, pressed) = (event.physical_key, event.state.is_pressed());
                match (key, pressed) {
                    (PhysicalKey::Code(code), true) => ctx.input.keyboard.set_key(code),
                    (PhysicalKey::Code(code), false) => ctx.input.keyboard.release_key(code),
                    (PhysicalKey::Unidentified(code), _) => {
                        log::error!("pressed/released unidentified key {:?}", code)
                    }
                };
            }
            _ => {}
        },
        Event::DeviceEvent { ref event, .. } => match event {
            DeviceEvent::MouseMotion { delta } => ctx.input.mouse.set_mouse_delta(*delta),
            _ => {}
        },
        Event::AboutToWait => {
            ctx.render.window.request_redraw();
        }
        _ => {}
    });

    match res {
        Ok(_) => {}
        Err(err) => log::error!("error in event loop: {err}"),
    }
}

//
// Commands
//

/// Returns a reference to the window
///
/// Window contains useful functions such as fullscreen, cursor control and resizing window
pub fn window(ctx: &mut Context) -> &winit::window::Window {
    &ctx.render.window
}

/// Enable/Disable vsync
pub fn set_vsync(ctx: &mut Context, vsync: bool) {
    let present_mode = if vsync {
        wgpu::PresentMode::AutoVsync
    } else {
        wgpu::PresentMode::AutoNoVsync
    };
    ctx.render.reconfigure_present_mode(present_mode);
}

// /// Enable/Disable borderless windowed mode
// pub fn set_fullscreen(ctx: &mut Context, fullscreen: bool) {
//     let fullscreen_mode = if fullscreen {
//         Some(winit::window::Fullscreen::Borderless(None))
//     } else {
//         None
//     };
//     ctx.render.window.set_fullscreen(fullscreen_mode);
// }

// /// Enable/Disable window resizing
// pub fn set_resizeable(ctx: &mut Context, resizable: bool) {
//     ctx.render.window.set_resizable(resizable);
// }

// /// Enable/Disable the cursor
// ///
// /// If disabled: Turns off cursor graphics and locks cursor to middle of window
// pub fn set_cursor_enabled(ctx: &mut Context, enabled: bool) {
//     // TODO handle error
//     ctx.render.window.set_cursor_visible(enabled);
//     let grab_mode = if enabled {
//         winit::window::CursorGrabMode::None
//     } else {
//         winit::window::CursorGrabMode::Locked
//     };
//     ctx.render
//         .window
//         .set_cursor_grab(grab_mode)
//         .expect("could not set cursor grab mode");
// }
