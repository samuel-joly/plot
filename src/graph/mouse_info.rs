use crate::graph::{coordinate::Coordinate, draw::text::Text};
use ab_glyph::FontRef;
use winit::dpi::PhysicalPosition;

use super::{scale::Scale, Graph, draw::text::TextCompiler};
pub struct Mouse {
    pub position: PhysicalPosition<f64>,
    pub axis_pixels: Vec<u32>,
    pub position_pixels: Vec<(u32, u32)>,
}

impl Mouse {
    pub fn new() -> Mouse {
        Mouse {
            position: PhysicalPosition::new(0.0, 0.0),
            axis_pixels: vec![],
            position_pixels: vec![],
        }
    }

    pub fn draw_mouse_axis(&mut self, scale: &Scale) {
        self.axis_pixels = vec![];
        let x = self.position.x as i32 - (scale.width as i32 / 2);
        let y = (scale.height / 2) as i32 - self.position.y as i32;
        let mouse_coord = Coordinate::from_pos((scale.width, scale.height), (x, y))
            .unwrap()
            .get_index();

        for i in 0..20 {
            self.axis_pixels
                .push(((mouse_coord % scale.width) + (i * scale.width as u32)) as u32);
            self.axis_pixels
                .push(((mouse_coord - (mouse_coord % scale.width)) + i) as u32);
        }
    }

    pub fn draw_mouse_position(&mut self, scale: &Scale, font: &TextCompiler) {
        self.position_pixels = vec![];
        let x = self.position.x as f32 - (scale.width / 2) as f32;
        let y = (scale.height / 2) as f32 - self.position.y as f32;

        let x_txt = format!(
            "x:{}",
            f32::trunc((x as f32 / scale.factor_x) * 100.0) / 100.0
        )
        .to_string();
        let y_txt = format!(
            "y:{}",
            f32::trunc((y as f32 / scale.factor_y) * 100.0) / 100.0
        )
        .to_string();

        let x_coord = Coordinate::from_pos(
            (scale.width, scale.height),
            ((x.floor() as i32 + 3), (y.floor() as i32) + 26),
        )
        .unwrap();
        let y_coord = Coordinate::from_pos(
            (scale.width, scale.height),
            ((x.floor() as i32) + 3, (y.floor() as i32) + 10),
        )
        .unwrap();

        let mut mouse_txt_x = Text::from(x_txt, x_coord, Some(true)).unwrap();
        let mut mouse_txt_y = Text::from(y_txt, y_coord, Some(true)).unwrap();

        for (pixel, color) in mouse_txt_x.draw((scale.width, scale.height), font) {
            self.position_pixels.push((pixel, color));
        }
        for (pixel, color) in mouse_txt_y.draw((scale.width, scale.height), font) {
            self.position_pixels.push((pixel, color));
        }
    }
}
