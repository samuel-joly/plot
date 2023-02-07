pub mod line;
pub mod text;

pub trait Drawable {
    fn draw(&mut self, size:(u32,u32)) -> Vec<u32>;
    fn is_mut(&self) -> bool;
    fn get_mut_pixels(&self) -> Vec<u32>;
    fn set_mut_pixels(&mut self, mut_pixels: Vec<u32>);
    fn is_scaled(&self) -> bool {
        false
    }
    fn is_scalable(&self) -> bool {
        false
    }
    fn set_is_scaled(&mut self, scaled:bool);
}

//pub enum Drawable {
//    Line{shape:Line},
//    Text{shape:Text},
//}

//impl Drawable<T> {
//    pub fn draw_shape(&mut self, graph: &mut Graph<T>) {
//        match self {
//            Drawable::Line{shape:line} => {
//                if line.scaled == false {
//                    line.from = (
//                        (line.og_from.0 as f32 * graph.scale.factor_x).floor() as i32,
//                        (line.og_from.1 as f32 * graph.scale.factor_y).floor() as i32,
//                    );
//                    line.to = (
//                        (line.og_to.0 as f32 * graph.scale.factor_x).floor() as i32,
//                        (line.og_to.1 as f32 * graph.scale.factor_y).floor() as i32,
//                    );
//                    line.scaled = true;
//                }
//                line.draw(graph)
//            }
//            Drawable::Text{shape:text} => text.draw(graph),
//        }
//    }
//}
