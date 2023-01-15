use line::Line;
use offset::Offset;
use winit::dpi::PhysicalSize;

use self::coordinate::Coordinate;

mod coordinate;
mod line;
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

    pub fn draw_line(&mut self) {
        let line = Line::from((-100, 200), (200, -200), 0x00CC00 as u32);

        let coord_start = Coordinate::from_pos(&self, (-100, 100)).unwrap();
        let coord_end = Coordinate::from_pos(&self, (200, -200)).unwrap();

        let (line_width, line_height) = {
            let coord = Coordinate::substr(&self, &coord_start, &coord_end);
            (coord.get_pos().0, coord.get_pos().1)
        };

        let mut increment: f64 = (line_width / line_height) as f64;
        if increment < 1.0 {
            increment = (line_height / line_width) as f64;
        }
        dbg!(
            &self.width,
            &self.height,
            &line,
            &coord_start,
            &coord_end,
            line_width,
            line_height,
            increment.abs()
        );

        for i in 0..line_width.abs() {
            let new_start_x = coord_start.get_pos().0 + i;
            println!("--");
            for j in 0..increment.abs() as i32 {
                let new_start_y = coord_start.get_pos().1 + (j+1)*i;
                let new_coordinate =
                    Coordinate::from_pos(&self, (new_start_x, new_start_y)).unwrap();
                dbg!(&new_coordinate, j,i);
                drop(std::mem::replace(
                    &mut self.buffer[new_coordinate.get_index() as usize],
                    line.color,
                ));
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
