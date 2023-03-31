mod graph;
use graph::{coordinate::Coordinate, draw::circle::Circle, Graph};
use softbuffer::GraphicsContext;
use winit::{
    event::{DeviceEvent, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Plot 0.9")
        .build(&event_loop)
        .unwrap();
    let mut graphics_context = unsafe { GraphicsContext::new(&window, &window) }.unwrap();
    let mut graphic = Graph::new();
    graphic.background = 0x000000;
    graphic.foreground = 0xFFFFFF;
    graphic.scale.set_scale(1000.0, 400.0);
    let circle = Circle::from(100, Coordinate::new(), true);
    graphic.shapes = vec![Box::new(circle)];

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(..),
                ..
            } => {
                graphic.scale.set_size(window.inner_size());
                graphic.fill_buffer(graphic.background);
            }

            Event::RedrawRequested(window_id) if window_id == window.id() => {
                if graphic.scale.width == 0 {
                } else {
                    graphic.draw_shapes();
                    graphic.draw_scale();
                    graphic.draw_mouse_info();
                    graphics_context.set_buffer(
                        &graphic.buffer,
                        graphic.scale.width as u16,
                        graphic.scale.height as u16,
                    );
                }
            }
            Event::MainEventsCleared => if graphic.scale.width != 0 {},
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
            }

            Event::DeviceEvent {
                event: DeviceEvent::MouseWheel { delta },
                ..
            } => match delta {
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
                    for l in graphic.shapes.iter_mut() {
                        if l.is_scalable() && l.is_scaled() {
                            l.set_is_scaled(false);
                        }
                    }
                    window.request_redraw();
                }
            },

            Event::WindowEvent {
                event: WindowEvent::MouseInput { .. },
                ..
            } => {
                //                if state == ElementState::Pressed && button == MouseButton::Left {
                //                } else if state == ElementState::Pressed && button == MouseButton::Right {
                //                } else {
                //                }
            }

            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                graphic.mouse.position = position;
                window.request_redraw();
            }

            _ => {}
        }
    });
}

fn _courbe(width: u32, height: u32, scale_x: f32, scale_y: f32) -> Vec<u32> {
    let mut dots: Vec<u32> = vec![];
    let mut data: f64;
    let mut val: f64;
    for i in 0..100 {
        val = -5.0 + (i as f64 / 50.0);
        if i != 0 {
            data = val.cos();
        } else {
            continue;
        }
        dots.push(
            Coordinate::from_pos(
                (width, height),
                (
                    (val * scale_x as f64) as i32,
                    (data * scale_y as f64) as i32,
                ),
            )
            .unwrap()
            .get_index(),
        );
    }
    dots
}
