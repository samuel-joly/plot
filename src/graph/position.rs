use super::Graph;
use std::io::Error;

#[derive(Debug)]
pub struct GraphPosition {
    euclidian: (i32, i32),
    index: u32,
}

impl GraphPosition {
    pub fn from_pos(graph: &Graph, pos: (i32, i32)) -> Result<GraphPosition, Error> {
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

        let index = pos.0 + (pos.1 * graph.width as i32);
        Ok(GraphPosition {
            euclidian: pos,
            index: index as u32,
        })
    }

    pub fn from_index(graph: &Graph, index: u32) -> Result<GraphPosition, Error> {
        if index >= (graph.width * graph.height) {
            panic!(
                "Index should not be more than {} and less than 0",
                graph.width * graph.height
            );
        }

        let y = index / graph.width;
        let x = index % graph.width;
        Ok(GraphPosition {
            euclidian: (x as i32, y as i32),
            index,
        })
    }
}
