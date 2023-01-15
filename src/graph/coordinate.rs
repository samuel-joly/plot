use super::Graph;
use std::io::Error;

#[derive(Debug)]
pub struct Coordinate {
    euclidian: (i32, i32),
    index: u32,
}

impl Coordinate {
    pub fn new() -> Coordinate {
        Coordinate {
            euclidian: (0, 0),
            index: 0,
        }
    }

    pub fn get_pos(&self) -> (i32, i32) {
        self.euclidian
    }

    pub fn get_index(&self) -> u32 {
        self.index
    }

    pub fn from_pos(graph: &Graph, pos: (i32, i32)) -> Result<Coordinate, Error> {
        if pos.0 >= graph.width as i32 / 2 || pos.0 <= -(graph.width as i32 / 2) {
            panic!(
                "X coordinate should be less than {} and more than {}",
                graph.width / 2,
                -(graph.width as i32 / 2)
            );
        }

        if pos.1 >= graph.height as i32 / 2 || pos.1 <= -(graph.height as i32 / 2) {
            panic!(
                "Y coordinate should be less than {} and more than {}",
                graph.height / 2,
                -(graph.height as i32 / 2)
            );
        }

        let index = ((graph.width / 2) as i32 + pos.0)
            + (graph.width/2 * graph.height/2) as i32
            + (graph.width as i32 * pos.1);

        Ok(Coordinate {
            euclidian: pos,
            index: index as u32,
        })
    }

    pub fn from_index(graph: &Graph, index: u32) -> Result<Coordinate, Error> {
        if index >= (graph.width * graph.height) {
            panic!(
                "Index should not be more than {} and less than 0",
                graph.width * graph.height
            );
        }

        let y = index / graph.width;
        let x = index % graph.width;
        Ok(Coordinate {
            euclidian: (x as i32, y as i32),
            index,
        })
    }

    pub fn substr(graph: &Graph, coordinate: &Coordinate, coordinate_2: &Coordinate) -> Coordinate {
        let euclidian = (
            (coordinate.euclidian.0 - coordinate_2.euclidian.0),
            (coordinate.euclidian.1 - coordinate_2.euclidian.1),
        );

        Coordinate {
            euclidian,
            index: Coordinate::from_pos(graph, euclidian).unwrap().index,
        }
    }
}
