use softbuffer::GraphicsContext;
use winit::{
    event::{DeviceEvent, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

struct Graph {
    width: u32,
    height: u32,
    buffer: Vec<u32>,
}

impl Graph {
    fn draw_plane(&mut self, gc: &mut GraphicsContext) {
        let buffer = (0..((self.width * self.height) as usize))
            .map(|index| {
                let y = index / (self.width as usize);
                let x = index % (self.width as usize);

                let black = 0x00;
                let white = 0xFFFFFF;
                let grey = 0xD0D0D0;

                if x > ((self.width as usize) / 2) - 2 && x < (self.width as usize / 2) + 2 {
                    black
                } else if y > (self.height as usize / 2) - 2 && y < (self.height as usize / 2) + 2
                {
                    black
                } else {
                    white
                }
            })
            .collect::<Vec<_>>();

        gc.set_buffer(&buffer, self.width as u16, self.height as u16);
        self.buffer = buffer;
    }
}

fn _tst_color_bits(r: u8, g: u8, b: u8) {
    println!(
        "{:b}\n{:b}\n{:b}\n{:b}",
        r,
        g,
        b,
        (b | g << 8 | r << 16) as u32
    );
}
fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Plot 0.1")
        .build(&event_loop)
        .unwrap();
    //window.with_window_icon("Make_an_icon");
    let mut graphics_context = unsafe { GraphicsContext::new(&window, &window) }.unwrap();
    let mut canvas = Graph {
        width: 0,
        height: 0,
        buffer: Vec::new(),
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let size = window.inner_size();
                canvas.width = size.width;
                canvas.height = size.height;

                canvas.draw_plane(&mut graphics_context);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
            }
            Event::MainEventsCleared => {}

            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                println!("INPT\tMotion: {:?}", delta);
            }
            Event::DeviceEvent {
                event: DeviceEvent::MouseWheel { delta },
                ..
            } => {
                println!("INPT\tScroll: {:?}", delta);
            }
            Event::DeviceEvent {
                event: DeviceEvent::Button { button, state },
                ..
            } => {
                println!("BTN\t {:?}, {:?}", button, state);
            }
            _ => {}
        }
    });
}
