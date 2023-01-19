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

    pub fn draw_line(&mut self, line: &mut Line) -> () {
        //dbg!(&line);
        let mut stop_val = false;

        let coord_start: Coordinate;
        let coord_end: Coordinate;
        match Coordinate::from_pos(&self, line.from) {
            Some(coord) => coord_start = coord,
            None => {
                coord_start = Coordinate::new();
                stop_val = true;
            }
        };
        match Coordinate::from_pos(&self, line.to) {
            Some(coord) => coord_end = coord,
            None => {
                coord_end = Coordinate::new();
                stop_val = true;
            }
        }

        if stop_val {
            return;
        }

        let dimension = line.dimension(&self);
        line.is_dimension_even(dimension);

        let mut equalize: bool = false;
        let mut new_start_x = coord_start.get_pos().0;
        let mut new_start_y = coord_start.get_pos().1;

        dbg!("--",line.increment, line.increment_rest, line.equalizer, dimension );

        let mut pos_line_width = dimension.0.abs();
        let mut x_increment: bool = false;
        for i in 0..pos_line_width {
            if line.equalizer != 0.0 && 0 == (i + 1) % line.equalizer as i32 {
                equalize = true;
                if 0 > line.equalizer as i32 {
                    x_increment = true;
                } else {
                    line.increment += 1;
                }
                //println!("incerment {} for {}",line.increment, i);
                pos_line_width -= line.increment;
            }

            if new_start_x < coord_end.get_pos().0 {
                if x_increment {
                    new_start_x += 1
                }
                new_start_x += 1
            } else {
                if x_increment {
                    new_start_x -= 1
                }
                new_start_x -= 1
            };

            for _ in 0..line.increment {
                if new_start_y < coord_end.get_pos().1 {
                    new_start_y += 1;
                } else {
                    new_start_y -= 1;
                };
                line.increment_rest -= 1;
                //println!("{}-{} {} {}", i,j, new_start_x, new_start_y);

                match Coordinate::from_pos(&self, (new_start_x, new_start_y)) {
                    Some(new_coord) => {
                        drop(std::mem::replace(
                            &mut self.buffer[new_coord.get_index() as usize],
                            line.color,
                        ));
                    }
                    None => (),
                };
            }

            if equalize {
                equalize = false;
                if 0 > line.equalizer as i32 {
                    x_increment = false;
                } else {
                    line.increment -= 1;
                }
            }
        }
        dbg!(line.increment_rest);
        return;
    }

    pub fn draw_axis(&mut self) {
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
