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

    pub fn init_buffer(&mut self, color:u32, width:u32, height:u32) {
        self.buffer = (0..((width * height) as usize))
            .map(|_| color)
            .collect::<Vec<_>>();
    }

    pub fn draw_line(&mut self, line: &mut Line) -> () {
        let mut stop_val = false;
        let coord_start: Coordinate;
        match Coordinate::from_pos(&self, line.from) {
            Some(coord) => coord_start = coord,
            None => {
                coord_start = Coordinate::new();
                stop_val = true;
            }
        };

        if stop_val {
            return;
        }

        let dimension = line.dimension(&self);

        let mut new_start_x = coord_start.get_pos().0;
        let mut new_start_y = coord_start.get_pos().1;

        let direction_x = if dimension.0 > 0 { 1 } else if dimension.0 < 0 { -1 } else { 0 };
        let direction_y = if dimension.1 > 0 { 1 } else if dimension.1 < 0 { -1 } else { 0 };

        let mut repeater = 1;

        let diff_dim = dimension.0.abs() - dimension.1.abs();
        let equalizer:i32;
        let mut equalized:bool = false;

        if diff_dim > 0 {
            equalizer = dimension.0.abs() / diff_dim.abs();
            dbg!(dimension.0.abs() % diff_dim.abs());
        } else if diff_dim < 0 {
            equalizer = dimension.1.abs() / diff_dim.abs();
            dbg!(dimension.1.abs() % diff_dim.abs());
        } else {
            equalizer = 0;
        }
        dbg!(dimension, diff_dim,  equalizer, new_start_x, new_start_y);

        for i in 0..dimension.0.abs() {
            if equalizer > 0 && i % equalizer == 0 {
                equalized = true;
                repeater += 1;
            }

            new_start_x += direction_x;
            for _ in 0..repeater {
                new_start_y += direction_y;
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
            if equalized {
                repeater -= 1;
                equalized = false;
            }
        }
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
