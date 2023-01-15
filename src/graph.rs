use winit::dpi::PhysicalSize;
mod offset;
mod line;
mod graphPosition;

use offset::Offset;

#[derive(Debug)]
pub struct Graph {
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<u32>,
    pub offset: Offset,
    pub x_axis: Line,
    pub y_axis: Line,
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
        let half_size = (self.width * self.height) / 2;
        for i_column in 1..self.width {
            let next_index = ((half_size + i_column as u32) as i32) as usize;
            drop(std::mem::replace(
                &mut self.buffer[next_index],
                0xFFFFFF as u32,
            ));
        }

        let half_width = self.width / 2;
        for i_row in 0..self.height {
            let next_index = ((self.width * i_row) + half_width) as usize;
            drop(std::mem::replace(
                &mut self.buffer[next_index],
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
