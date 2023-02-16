mod graph;
use graph::Graph;
use softbuffer::GraphicsContext;
use winit::{
    dpi::PhysicalPosition,
    event::{DeviceEvent, ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Plot 0.1")
        .build(&event_loop)
        .unwrap();
    let mut graphics_context = unsafe { GraphicsContext::new(&window, &window) }.unwrap();
    let mut graphic = Graph::new();
    graphic.background = 0x000000;
    graphic.foreground = 0xFFFFFF;
    graphic.scale.set_scale(2000.0, 800.0);

    let mut c_position: PhysicalPosition<f64> = PhysicalPosition::new(0.0, 0.0);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(..),
                ..
            } => {
                graphic.scale.set_size(window.inner_size());
                graphic.fill_buffer(graphic.background);
                graphic.mut_pixels = vec![];
            }

            Event::RedrawRequested(window_id) if window_id == window.id() => {
                if graphic.scale.width == 0 {
                } else {
                    graphics_context.set_buffer(
                        &graphic.buffer,
                        graphic.scale.width as u16,
                        graphic.scale.height as u16,
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
                if graphic.scale.width != 0 {
                    graphic.draw_shapes();
                    graphic.draw_scale();
                    graphic.draw_mouse_coordinates(c_position);
                    graphic.draw_mouse_axis(c_position);
                }
            }
            Event::DeviceEvent {
                event: DeviceEvent::MouseWheel { delta },
                ..
            } => {
                match delta {
                    winit::event::MouseScrollDelta::PixelDelta(p) => {
                        dbg!(p);
                    }
                    winit::event::MouseScrollDelta::LineDelta(_x, y) => {
                        if y < 0.0 {
                            graphic.scale.set_scale(
                                graphic.scale.current_interval_x.floor()
                                    - (graphic.scale.original_interval_x * 0.2).floor(),
                                graphic.scale.current_interval_y.floor()
                                    - (graphic.scale.original_interval_y * 0.2).floor(),
                            );
                        } else {
                            graphic.scale.set_scale(
                                graphic.scale.current_interval_x.floor()
                                    + (graphic.scale.original_interval_x * 0.2).floor(),
                                graphic.scale.current_interval_y.floor()
                                    + (graphic.scale.original_interval_y * 0.2).floor(),
                            );
                        }
                        for l in graphic.shapes.iter_mut(){
                            if l.is_scalable() {
                                if l.is_scaled() {
                                    l.set_is_scaled(false);
                                }
                            }
                        }
                    }
                }
                window.request_redraw();
            }

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
                window.request_redraw();
            }

            _ => {}
        }
    });
}
