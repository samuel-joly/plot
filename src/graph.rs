use line::Line;
use offset::Offset;
use winit::dpi::PhysicalSize;

use self::coordinate::Coordinate;

mod coordinate;
pub mod line;
mod offset;

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

    pub fn init_buffer(&mut self) {
        self.buffer = (0..((self.width * self.height) as usize))
            .map(|_| 0x00 as u32)
            .collect::<Vec<_>>();
    }

    pub fn draw_line(&mut self, line: &mut Line) {
        let coord_start = Coordinate::from_pos(&self, line.from).unwrap();
        let coord_end = Coordinate::from_pos(&self, line.to).unwrap();

        let dimension = line.dimension(&self);
        line.is_dimension_even(&self);

        let mut equalize: bool = false;
        let mut new_start_x: i32;
        let mut new_start_y: i32;
        let pos_line_width = dimension.0.abs();

        new_start_x = coord_start.get_pos().0;
        new_start_y = coord_start.get_pos().1;

//        dbg!(line.increment, line.increment_rest, line.equalizer );

        for i in 0..pos_line_width {
            if line.equalizer != 0 && 0 == (i+1) % line.equalizer as i32 {
                equalize = true;
                line.increment += 1;
            }

            if new_start_x < coord_end.get_pos().0 {
                new_start_x += 1
            } else {
                new_start_x -= 1
            };

            for _ in 0..line.increment {
                if new_start_y < coord_end.get_pos().1 {
                    new_start_y += 1;
                } else {
                    new_start_y -= 1;
                };

                let new_coordinate = Coordinate::from_pos(&self, (new_start_x, new_start_y)).unwrap();
                drop(std::mem::replace(
                    &mut self.buffer[new_coordinate.get_index() as usize],
                    line.color,
                ));
            }
            if equalize {
                line.increment -= 1;
                equalize = false;
            }
        }
    }

    pub fn draw(&mut self) {
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

    pub fn set_size(&mut self, size: PhysicalSize<u32>) {
        self.width = size.width;
        self.height = size.height;
    }
}
