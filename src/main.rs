use softbuffer::GraphicsContext;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn draw_plane(width: u32, height: u32, graphics_context: &mut GraphicsContext) {
    let buffer = (0..((width * height) as usize))
        .map(|index| {
            let y = index / (width as usize);
            let x = index % (width as usize);

            let black = 0x00 as u32;
            let white = 0xFFFFFF as u32;

            if x > ((width as usize) / 2) - 2
                && x < ((width as usize) / 2) + 2
            {
                black
            } else if y > ((height as usize) / 2) - 2
                && y < ((height as usize) / 2) + 2
            {
                black
            } else {
                white
            }
        })
        .collect::<Vec<_>>();

    graphics_context.set_buffer(&buffer, width as u16, height as u16);
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

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };
                draw_plane(width, height, &mut graphics_context);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
            }
            Event::MainEventsCleared => {}
            _ => {}
        }
    });
}
