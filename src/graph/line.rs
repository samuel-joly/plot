use super::{coordinate::Coordinate, Graph};
use std::io::Error;

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
    pub scaled: bool,
}

impl Line {
    pub fn from(start_pos: (i32, i32), end_pos: (i32, i32), color: u32, is_mut: bool) -> Line {
        Line {
            from: start_pos,
            to: end_pos,
            og_from: start_pos,
            og_to: end_pos,
            color,
            is_drawed: false,
            is_mut,
            scaled: false,
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
            scaled: false,
        }
    }

    pub fn draw(&mut self, graph: &mut Graph) -> () {
        let coord_start = Coordinate::from_pos((graph.scale.width, graph.scale.height), self.from)
            .unwrap()
            .get_pos();
        let line_dimension = self.dimension(&graph);

        let pix = std::cmp::max(line_dimension.0.abs(), line_dimension.1.abs());
        for i in 0..pix {
            let x = coord_start.0 + line_dimension.0 * i / pix;
            let y = coord_start.1 + line_dimension.1 * i / pix;

            let coord =
                Coordinate::from_pos((graph.scale.width, graph.scale.height), (x, y)).unwrap();
            if self.is_mut {
                graph.mut_pixels.push(coord.get_index());
            }
            drop(std::mem::replace(
                &mut graph.buffer[coord.get_index() as usize],
                self.color,
            ));
        }
    }

    pub fn to_coords(&self, graph: &Graph) -> Result<(Coordinate, Coordinate), Error> {
        let start_pos: Coordinate =
            Coordinate::from_pos((graph.scale.width, graph.scale.height), self.from)?;
        let end_pos: Coordinate =
            Coordinate::from_pos((graph.scale.width, graph.scale.height), self.to)?;
        Ok((start_pos, end_pos))
    }

    pub fn dimension(&self, graph: &Graph) -> (i32, i32) {
        let c: (Coordinate, Coordinate);
        let mut stop_val = false;
        match self.to_coords(graph) {
            Ok(val) => c = val,
            Err(..) => {
                c = (Coordinate::new(), Coordinate::new());
                stop_val = true
            }
        }
        if stop_val {
            return (0, 0);
        }
        let coord = Coordinate::substr(&c.0, &c.1);

        (-coord.0, -coord.1)
    }
}
