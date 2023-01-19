mod graph;
use crate::graph::line::Line;

use softbuffer::GraphicsContext;
use winit::{
    event::{DeviceEvent, ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::graph::Graph;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Plot 0.1")
        .build(&event_loop)
        .unwrap();
    //window.with_window_icon("Make_an_icon");
    let mut graphics_context = unsafe { GraphicsContext::new(&window, &window) }.unwrap();
    let mut canvas = Graph::new();

    let mut is_pressed_first: bool = false;
    let mut is_pressed = false;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(..),
                ..
            } => {
                canvas.set_size(window.inner_size());
                canvas.init_buffer();
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                canvas.draw_axis();
                let mut line_c = Line::from((-100,-100), (100,100), 0xCCCC00 as u32);
                let mut line_ctb = Line::from((-260, 300), (0,0), 0x00CC00 as u32);
                let mut line_ctr = Line::from((260, 300), (0,0), 0x00CC00 as u32);
                let mut line_ctl = Line::from((-260, 300), (260,300), 0x00CC00 as u32);
                canvas.draw_line(&mut line_c);
                canvas.draw_line(&mut line_ctb);
                canvas.draw_line(&mut line_ctr);
                canvas.draw_line(&mut line_ctl);
                graphics_context.set_buffer(
                    &canvas.buffer,
                    canvas.width as u16,
                    canvas.height as u16,
                );
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
            }

            Event::MainEventsCleared => {}
            Event::DeviceEvent {
                event: DeviceEvent::MouseWheel { .. },
                ..
            } => {}

            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                if state == ElementState::Pressed && button == MouseButton::Left {
                    is_pressed = true;
                    is_pressed_first = true;
                } else {
                    is_pressed = false;
                }
            }

            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                if is_pressed_first && is_pressed {
                    canvas
                        .offset
                        .prepare_movement(position.x as u32, position.y as u32);
                    is_pressed_first = false;
                } else if is_pressed {
                    canvas
                        .offset
                        .diff_drag_to_offset(position.x as i32, position.y as i32);
                    window.request_redraw();
                }
            }

            _ => {}
        }
    });
}
