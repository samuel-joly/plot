use super::Graph;
use std::io::Error;

#[derive(Debug)]
pub struct Coordinate {
    cartesian: (i32, i32),
    index: u32,
}

impl Coordinate {
    pub fn new() -> Coordinate {
        Coordinate {
            cartesian: (0, 0),
            index: 0,
        }
    }

    pub fn get_pos(&self) -> (i32, i32) {
        self.cartesian
    }

    pub fn get_index(&self) -> u32 {
        self.index
    }

    pub fn from_pos(graph: &Graph, pos: (i32, i32)) -> Result<Coordinate,Error> {
        if graph.width == 0 || graph.height == 0 {
            return Ok(Coordinate::new());
        }
        if pos.0 > graph.width as i32 / 2 || pos.0 < -(graph.width as i32 / 2) {
            return Ok(Coordinate::new());
        }

        if pos.1 > graph.height as i32 / 2 || pos.1 < -(graph.height as i32 / 2) {
            return Ok(Coordinate::new());
        }

        let index = ((graph.width / 2) as i32 + pos.0)
            + (((graph.width * graph.height) / 2) as i32)
            - graph.width as i32 * pos.1;

        Ok(Coordinate {
            cartesian: pos,
            index: index as u32,
        })
    }

    pub fn from_index(graph: &Graph, index: u32) -> Result<Coordinate, Error> {
        if index >= (graph.width * graph.height) {
            return Ok(Coordinate::new());
        }

        let y = (index / graph.width) as i32 - (graph.height/2) as i32;
        let x = (index % graph.width) as i32 - graph.width as i32/2;
        Ok(Coordinate {
            cartesian: (x as i32, y as i32),
            index,
        })
    }

    pub fn substr(coordinate: &Coordinate, coordinate_2: &Coordinate) -> (i32,i32) {
        let cartesian = (
            (coordinate.cartesian.0 - coordinate_2.cartesian.0),
            (coordinate.cartesian.1 - coordinate_2.cartesian.1),
        );
        cartesian
    }
}
