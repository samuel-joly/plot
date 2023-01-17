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

    pub fn draw_line(&mut self, line: &Line) {
        let coord_start = Coordinate::from_pos(&self, line.from).unwrap();
        let coord_end = Coordinate::from_pos(&self, line.to).unwrap();

        let (line_width, line_height) = {
            let coord = Coordinate::substr(&self, &coord_start, &coord_end);
            (coord.get_pos().0, coord.get_pos().1)
        };

        let u_line_width = line_width.abs();
        let u_line_height = line_height.abs();

        let mut increment: i32;
        let increment_rest: i32;
        let equalizer: f64;
        let mut no_equalize: bool = false;
        if line_width >= line_height {
            increment = u_line_width / u_line_height;
            increment_rest = u_line_width % u_line_height;
            if increment_rest != 0 {
                equalizer = (u_line_width / increment_rest) as f64;
            } else {
                equalizer = 0.0;
                no_equalize = true;
            }
        } else {
            increment = u_line_height / u_line_width;
            increment_rest = u_line_height % u_line_width;
            if increment_rest != 0 {
                equalizer = (u_line_height / increment_rest) as f64;
            } else {
                equalizer = 0.0;
                no_equalize = true;
            }
        }

        let mut equalize: bool = false;
        let mut new_start_x: i32;
        let mut new_start_y: i32;

        new_start_x = coord_start.get_pos().0;
        new_start_y = coord_start.get_pos().1;

        for i in 0..u_line_width {
            if !no_equalize && 0 == (i - 1) % equalizer as i32 {
                equalize = true;
                increment += 1;
            }

            if new_start_x < coord_end.get_pos().0 {
                new_start_x += 1
            } else {
                new_start_x -= 1
            };
            if new_start_y < coord_end.get_pos().1 {
                new_start_y += increment;
            } else {
                new_start_y -= increment;
            };

            let new_coordinate = Coordinate::from_pos(&self, (new_start_x, new_start_y)).unwrap();
            drop(std::mem::replace(
                &mut self.buffer[new_coordinate.get_index() as usize],
                line.color,
            ));
            if equalize {
                increment -= 1;
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
