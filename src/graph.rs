use softbuffer::GraphicsContext;
use winit::dpi::PhysicalSize;
mod offset;

use offset::Offset;

#[derive(Debug)]
pub struct Graph {
    width: u32,
    height: u32,
    buffer: Vec<u32>,
    pub offset: Offset
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            width: 0,
            height: 0,
            buffer: Vec::new(),
            offset: Offset::new(),
        }
    }

    pub fn draw(&mut self, gc: &mut GraphicsContext) {
        gc.set_buffer(&self.buffer, self.width as u16, self.height as u16);
    }

    pub fn init_grid(&mut self) {
        self.buffer = (0..((self.width * self.height) as usize))
            .map(|index| {
                let (y, x) = self.get_pos(index);

                let width = (self.width as i32 - self.offset.x) as usize;
                let height = (self.height as i32 - self.offset.y) as usize;

                let black = 0x00;
                let white = 0xFFFFFF;

                // Draw center lines
                if x > (width/ 2) - 2 && x < (width/ 2) + 2 {
                    black
                } else if y > (height/ 2) - 2 && y < (height/ 2) + 2 {
                    black
                } else {
                    white
                }
            })
            .collect::<Vec<_>>();
    }

    fn get_pos(&self, index: usize) -> (usize, usize) {
        (index / (self.width as usize), index % (self.width as usize))
    }

    pub fn set_size(&mut self, size: PhysicalSize<u32>) {
        self.width = size.width;
        self.height = size.height;
    }

}
