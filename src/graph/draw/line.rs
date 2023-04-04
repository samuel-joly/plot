use super::Drawable;
use crate::graph::{coordinate::Coordinate, scale::Scale};

/// Colored straight line
#[derive(Debug)]
pub struct Line {
    /// `from` and `to` are define as tupe(x,y)
    pub from: (i32, i32),
    pub to: (i32, i32),
    pub og_from: (i32, i32),
    pub og_to: (i32, i32),
    /// Color for the line, `softbuffer` color formatting is used
    pub color: u32,
    pub is_drawed: bool,
    pub is_mut: bool,
    pub is_scalable: bool,
    pub scaled: bool,
    pub mut_pixel: Vec<u32>,
}

impl Line {
    pub fn from(
        start_pos: (i32, i32),
        end_pos: (i32, i32),
        color: u32,
        is_mut: bool,
        scalable: bool,
    ) -> Line {
        Line {
            from: start_pos,
            to: end_pos,
            og_from: start_pos,
            og_to: end_pos,
            color,
            is_drawed: false,
            is_mut,
            is_scalable: scalable,
            scaled: false,
            mut_pixel: vec![],
        }
    }

    pub fn _new() -> Line {
        Line {
            from: (0, 0),
            to: (0, 0),
            og_from: (0, 0),
            og_to: (0, 0),
            color: 0xFFFFFF as u32,
            is_drawed: false,
            is_mut: false,
            is_scalable: false,
            scaled: false,
            mut_pixel: vec![],
        }
    }

    pub fn to_coords(&self, size: (u32, u32)) -> (Coordinate, Coordinate) {
        let start_pos: Coordinate = Coordinate::from_pos((size.0, size.1), self.from).unwrap();
        let end_pos: Coordinate = Coordinate::from_pos((size.0, size.1), self.to).unwrap();
        (start_pos, end_pos)
    }

    pub fn dimension(&self, size: (u32, u32)) -> (i32, i32) {
        let c: (Coordinate, Coordinate) = self.to_coords(size);
        let coord = Coordinate::substr(&c.0, &c.1);
        (-coord.0, -coord.1)
    }
}

impl Drawable for Line {
    fn draw(&mut self, size: (u32, u32)) -> Vec<(u32, u32)> {
        self.mut_pixel = vec![];
        let width = size.0;
        let height = size.1;

        let line_dimension = self.dimension((width, height));
        let pix = std::cmp::max(line_dimension.0.abs(), line_dimension.1.abs());

        let mut ret = vec![];

        for i in 0..pix {
            let x = self.from.0 + line_dimension.0 * i / pix;
            let y = self.from.1 + line_dimension.1 * i / pix;

            let coord = Coordinate::from_pos((width, height), (x, y)).unwrap();
            self.mut_pixel.push(coord.get_index());
            ret.push((coord.get_index(), self.color));
        }
        ret
    }

    fn is_mut(&self) -> bool {
        self.is_mut
    }

    fn get_mut_pixels(&self) -> Vec<u32> {
        self.mut_pixel.clone()
    }

    fn set_mut_pixels(&mut self, mut_pixels: Vec<u32>) {
        self.mut_pixel = mut_pixels;
    }

    fn is_scalable(&self) -> bool {
        self.is_scalable
    }

    fn is_scaled(&self) -> bool {
        self.scaled
    }

    fn scale(&mut self, scale: &Scale) {
        if self.scaled == false {
            self.from = (
                (self.og_from.0 as f32 * scale.factor_x).floor() as i32,
                (self.og_from.1 as f32 * scale.factor_y).floor() as i32,
            );
            self.to = (
                (self.og_to.0 as f32 * scale.factor_x).floor() as i32,
                (self.og_to.1 as f32 * scale.factor_y).floor() as i32,
            );
            self.scaled = true;
        }
    }

    fn set_is_scaled(&mut self, is_scaled: bool) {
        self.scaled = is_scaled;
    }
}
