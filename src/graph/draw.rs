use super::scale::Scale;

pub mod line;
pub mod text;

pub trait Drawable {
    fn draw(&mut self, size:(u32,u32)) -> Vec<(u32, u32)>;
    fn get_mut_pixels(&self) -> Vec<u32>;
    fn set_mut_pixels(&mut self, mut_pixels: Vec<u32>);
    fn is_mut(&self) -> bool {
        false
    }
    fn is_scalable(&self) -> bool {
        false
    }
    fn is_scaled(&self) -> bool {
        false
    }
    fn scale(&mut self, _scale:&Scale) {
        ();
    }
    fn set_is_scaled(&mut self, _is_scaled:bool) {
        ()
    }
}
