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

    pub fn from_pos(graph: &Graph, pos: (i32, i32)) -> Option<Coordinate> {
//        dbg!(graph.width, graph.height);
        if pos.0 >= graph.width as i32 / 2 || pos.0 <= -(graph.width as i32 / 2) {
            return None;
        }

        if pos.1 >= graph.height as i32 / 2 || pos.1 <= -(graph.height as i32 / 2) {
            return None;
        }

        let index = ((graph.width / 2) as i32 + pos.0)
            + (((graph.width * graph.height) / 2) as i32)
            - graph.width as i32 * pos.1;

//        dbg!(
//            (graph.width / 2) as i32 + pos.0,
//            ((graph.width * graph.height) / 2) as i32,
//            graph.width as i32 * pos.1,
//            (((graph.width * graph.height) / 2) as i32) + (graph.width as i32 * pos.1),
//            graph.width,
//            graph.height,
//            graph.width * graph.height
//        );
//
//        println!(
//            "Printing point at ({},{}) for index {}",
//            pos.0, pos.1, index
//        );

        Some(Coordinate {
            cartesian: pos,
            index: index as u32,
        })
    }

    pub fn _from_index(graph: &Graph, index: u32) -> Result<Coordinate, Error> {
        if index >= (graph.width * graph.height) {
            panic!(
                "Index should not be more than {} and less than 0",
                graph.width * graph.height
            );
        }

        let y = index / graph.width;
        let x = index % graph.width;
        Ok(Coordinate {
            cartesian: (x as i32, y as i32),
            index,
        })
    }

    pub fn substr(graph: &Graph, coordinate: &Coordinate, coordinate_2: &Coordinate) -> Coordinate {
        let cartesian = (
            (coordinate.cartesian.0 - coordinate_2.cartesian.0),
            (coordinate.cartesian.1 - coordinate_2.cartesian.1),
        );

        Coordinate {
            cartesian,
            index: Coordinate::from_pos(graph, cartesian).unwrap().index,
        }
    }
}
