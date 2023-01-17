use super::{coordinate::Coordinate, Graph};

/// Colored straight line
#[derive(Debug)]
pub struct Line {
    /// `from` and `to` are define as tupe(x,y)
    pub from: (i32, i32),
    pub to: (i32, i32),

    /// Color for the line, `softbuffer` color formatting is used
    pub color: u32,

    /// Graphics for un-even line (kind of very low level anti-aliasing)
    pub increment: i32,
    pub increment_rest: i32,
    pub equalizer:u32,

    /// Is it not clear enough ?
    pub is_drawed: bool,
}

impl Line {
    pub fn from(start_pos: (i32, i32), end_pos: (i32, i32), color: u32) -> Line {
        Line {
            from: start_pos,
            to: end_pos,
            color,
            is_drawed: false,
            increment:0,
            increment_rest:0,
            equalizer:0,
        }
    }

    pub fn _new() -> Line {
        Line {
            from: (0, 0),
            to: (0, 0),
            color: 0xFFFFFF as u32,
            is_drawed: false,
            increment:0,
            increment_rest:0,
            equalizer:0,
        }
    }

    pub fn to_coords(&self, graph: &Graph) -> (Coordinate, Coordinate) {
        (
            Coordinate::from_pos(graph, self.from).unwrap(),
            Coordinate::from_pos(graph, self.to).unwrap(),
        )
    }

    pub fn dimension(&self, graph: &Graph) -> (i32,i32) {
        let c = self.to_coords(graph);
        let coord = Coordinate::substr(graph, &c.0, &c.1);
        (coord.get_pos().0, coord.get_pos().1)
    }

    pub fn is_dimension_even(&mut self, graph: &Graph) {
        let (line_width, line_height) = self.dimension(graph);
        let u_line_width = line_width.abs();
        let u_line_height = line_height.abs();

        if line_width >= line_height {
            self.increment = u_line_width / u_line_height;
            self.increment_rest = u_line_width % u_line_height;
            if self.increment_rest != 0 {
                self.equalizer = (u_line_width / self.increment_rest) as u32;
            } else {
                self.equalizer = 0;
            }
        } else {
            self.increment = u_line_height / u_line_width;
            self.increment_rest = u_line_height % u_line_width;
            if self.increment_rest != 0 {
                self.equalizer = (u_line_height / self.increment_rest) as u32;
            } else {
                self.equalizer = 0;
            }
        }
    }
}
