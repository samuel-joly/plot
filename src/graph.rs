use softbuffer::GraphicsContext;
use winit::dpi::PhysicalSize;
mod offset;

use offset::Offset;

#[derive(Debug)]
pub struct Graph {
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<u32>,
    pub offset: Offset,
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

    pub fn whiteboard(&mut self) {
        self.buffer = (0..((self.width * self.height) as usize))
            .map(|_| 0x00 as u32)
            .collect::<Vec<_>>();
    }

    pub fn axis(&mut self) {
        for i in 1..self.width {
            let index = (self.width * self.height) / 2;

            drop(std::mem::replace(
                &mut self.buffer[(index + i as u32) as usize],
                0xFFFFFF as u32,
            ));
            drop(std::mem::replace(
                &mut self.buffer[(index + self.width + i as u32) as usize],
                0xFFFFFF as u32,
            ));
        }

        for i in 0..self.height {
            let half_width = self.width / 2;
            drop(std::mem::replace(
                &mut self.buffer[(self.width * i + half_width) as usize],
                0xFFFFFF as u32,
            ));
            drop(std::mem::replace(
                &mut self.buffer[(self.width * i + half_width + 1) as usize],
                0xFFFFFF as u32,
            ));
        }
    }

    fn _get_pos(&self, index: usize) -> (usize, usize) {
        (index / (self.width as usize), index % (self.width as usize))
    }

    pub fn set_size(&mut self, size: PhysicalSize<u32>) {
        self.width = size.width;
        self.height = size.height;
    }
}
