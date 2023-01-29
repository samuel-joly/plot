mod graph;
use crate::graph::line::Line;
use graph::Drawable;
use softbuffer::GraphicsContext;
use winit::{
    dpi::PhysicalPosition,
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
    let mut graphics_context = unsafe { GraphicsContext::new(&window, &window) }.unwrap();
    let mut graphic = Graph::new(900,400);

    let red = 0xCC0000;
    let green = 0x00CC00;
    let blue = 0x00000CC;
    let purple = 0xCC000CC;

    let mut c_position: PhysicalPosition<f64> = PhysicalPosition::new(0.0, 0.0);

    let lines: Vec<Drawable> = vec![
        Drawable::Line(Line::from((-300, -300), (300, 300), green, false)),
        Drawable::Line(Line::from((-300, 300), (300, -300), blue, false)),
        Drawable::Line(Line::from((-200, 112), (0, 285), purple, false)),
        Drawable::Line(Line::from((-200, 112), (0, -285), purple, false)),
        Drawable::Line(Line::from((200, 112), (0, -285), red, false)),
        Drawable::Line(Line::from((200, 112), (0, 285), red, false)),
    ];

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(..),
                ..
            } => {
                graphic.set_size(window.inner_size());
                graphic.mut_pixels = vec![];
                graphic.init_buffer(0x00 as u32, graphic.width, graphic.height);
                graphic.draw(&lines);
            }

            Event::RedrawRequested(window_id) if window_id == window.id() => {
                if graphic.width == 0 {
                } else {
                    graphic.draw(&graphic.mouse_coordinates(c_position));

                    graphics_context.set_buffer(
                        &graphic.buffer,
                        graphic.width as u16,
                        graphic.height as u16,
                    );
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
            }

            Event::MainEventsCleared => {
                graphic.draw(&lines);
                graphic.draw_axis();
                graphic.draw_scale();
            }
            Event::DeviceEvent {
                event: DeviceEvent::MouseWheel { .. },
                ..
            } => {}

            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                if state == ElementState::Pressed && button == MouseButton::Left {
                } else if state == ElementState::Pressed && button == MouseButton::Right {
                } else {
                }
            }

            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                c_position = position;
                graphic.clear_mut_pixels();
                window.request_redraw();
            }

            _ => {}
        }
    });
}
