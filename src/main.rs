mod graph;
use graph::{Graph, drawable::Drawable, drawable::line::Line };
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
    graphic.scale.set_scale(20000.0, 10000.0);

    let red = 0xCC0000;
    let green = 0x00CC00;
    //let blue = 0x00000CC;
    //    let purple = 0xCC000CC;

    let mut c_position: PhysicalPosition<f64> = PhysicalPosition::new(0.0, 0.0);

    let mut lines: Vec<Drawable> = vec![
        Drawable::Line(Line::from((-500, -500), (500, -500), red, true)),
        Drawable::Line(Line::from((500, 500), (-500, 500), red, true)),
        Drawable::Line(Line::from((-500, -500), (-500, 500), red, true)),
        Drawable::Line(Line::from((500, 500), (500, -500), red, true)),
        Drawable::Line(Line::from((500, 500), (-500, -500), green, true)),
        Drawable::Line(Line::from((500, -500), (-500, 500), green, true)),
    ];

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(..),
                ..
            } => {
                graphic.scale.set_size(window.inner_size());
                graphic.fill_buffer(0x00 as u32, graphic.scale.width, graphic.scale.height);
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
                    graphic.clear_mut_pixels();
                    graphic.draw(&mut lines);
                    graphic.draw_scale();
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
                        for l in lines.iter_mut() {
                            match l {
                                Drawable::Line(line) => line.scaled = false,
                                _ => (),
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
