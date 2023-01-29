use std::io::Error;

use super::{coordinate::Coordinate, Graph};

/// Colored straight line
#[derive(Debug)]
pub struct Line {
    /// `from` and `to` are define as tupe(x,y)
    pub from: (i32, i32),
    pub to: (i32, i32),

    /// Color for the line, `softbuffer` color formatting is used
    pub color: u32,

    pub is_drawed: bool,

    pub is_mut: bool,
}

impl Line {
    pub fn from(start_pos: (i32, i32), end_pos: (i32, i32), color: u32, is_mut: bool) -> Line {
        Line {
            from: start_pos,
            to: end_pos,
            color,
            is_drawed: false,
            is_mut,
        }
    }

    pub fn _new() -> Line {
        Line {
            from: (0, 0),
            to: (0, 0),
            color: 0xFFFFFF as u32,
            is_drawed: false,
            is_mut: false,
        }
    }

    pub fn draw(&self, graph: &mut Graph) -> () {
        let coord_start = Coordinate::from_pos(&graph, self.from).unwrap();

        let new_start_x = coord_start.get_pos().0;
        let new_start_y = coord_start.get_pos().1;

        let dimension = self.dimension(&graph);

        if graph.height == 0 || graph.width == 0 {return;}
        let pix = std::cmp::max(dimension.0.abs(), dimension.1.abs());
        for i in 0..pix{
            let x = new_start_x + dimension.0 * i / pix;
            let y = new_start_y + dimension.1 * i / pix;
            let coord = Coordinate::from_pos(&graph, (x, y)).unwrap();
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
        let start_pos: Coordinate;
        let end_pos: Coordinate;
        start_pos = Coordinate::from_pos(graph, self.from)?;
        end_pos = Coordinate::from_pos(graph, self.to)?;
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
