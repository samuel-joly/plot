pub mod line;
pub mod text;

use crate::Graph;
use crate::graph::drawable::{line::Line, text::Text};

pub enum Drawable {
    Line(Line),
    Text(Text),
}

impl Drawable {
    pub fn draw_shape(&mut self, graph: &mut Graph) {
        match self {
            Drawable::Line(line) => {
                if line.scaled == false {
                    line.from = (
                        (line.og_from.0 as f32 * graph.scale.factor_x).floor() as i32,
                        (line.og_from.1 as f32 * graph.scale.factor_y).floor() as i32,
                    );
                    line.to = (
                        (line.og_to.0 as f32 * graph.scale.factor_x).floor() as i32,
                        (line.og_to.1 as f32 * graph.scale.factor_y).floor() as i32,
                    );
                    line.scaled = true;
                }
                line.draw(graph)
            }
            Drawable::Text(text) => text.draw(graph),
        }
    }
}
