pub mod coordinate;
pub mod drawable;
pub mod scale;

use crate::graph::{
    drawable::{text::Text, Drawable},
    scale::Scale,
    coordinate::Coordinate,
};

use winit::{
    dpi::PhysicalPosition,
};

#[derive(Debug)]
pub struct Graph {
    pub buffer: Vec<u32>,
    pub mut_pixels: Vec<u32>,
    pub scale: Scale,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            buffer: Vec::new(),
            mut_pixels: vec![],
            scale: Scale::new(),
        }
    }

    pub fn fill_buffer(&mut self, color: u32, width: u32, height: u32) {
        self.buffer = (0..((width * height) as usize))
            .map(|_| color)
            .collect::<Vec<u32>>();
    }

    pub fn draw(&mut self, shapes: &mut Vec<Drawable>) {
        for shape in shapes {
            shape.draw_shape(self);
        }
    }

    pub fn draw_scale(&mut self) {
        let mut scale_shape: Vec<Drawable> = vec![];
        let width: u32 = self.scale.width;
        let height: u32 = self.scale.height;

        for i in 1..10 {
            let mut coord_y = i * ((height as f32 / 10.0) * width as f32).floor() as u32;
            coord_y -= coord_y % width;
            coord_y += width;
            let coord_x = (i as f32 * (width as f32 / 10.0)).floor() as u32;

            let line_x = Coordinate::from_index((width, height), coord_x)
                .unwrap()
                .get_pos();
            let line_y = Coordinate::from_index((width, height), coord_y)
                .unwrap()
                .get_pos();

            let mut index_x = coord_x;
            let mut index_y = coord_y;

            for _ in 0..4 {
                drop(std::mem::replace(
                    &mut self.buffer[index_y as usize],
                    0xFFFFFF as u32,
                ));
                index_y += 1;
                drop(std::mem::replace(
                    &mut self.buffer[index_x as usize],
                    0xFFFFFF as u32,
                ));
                index_x += width;
            }

            if i % 2 == 0 {
                let x_val = f32::trunc((line_x.0 as f32 / self.scale.factor_x) * 100.0) / 100.0;
                let y_val = f32::trunc((-line_y.1 as f32 / self.scale.factor_y) * 100.0) / 100.0;

                let text_x = x_val.to_string();
                let text_y = y_val.to_string();

                let coord_text_x = Coordinate::from_index(
                    (width, height),
                    index_x - (6 * text_x.len() as u32) + (5 * width),
                )
                .unwrap();
                let coord_text_y =
                    Coordinate::from_index((width, height), index_y - (3 * width)).unwrap();

                let scale_text_x = Text::from(text_x, 0xFFFFFF as u32, coord_text_x, true).unwrap();
                let scale_text_y = Text::from(text_y, 0xFFFFFF as u32, coord_text_y, true).unwrap();
                scale_shape.push(Drawable::Text(scale_text_x));
                scale_shape.push(Drawable::Text(scale_text_y));
            }
        }
        self.draw(&mut scale_shape);
    }

    pub fn draw_mouse_axis(&mut self, mouse_position: PhysicalPosition<f64>) -> () {
        let x = mouse_position.x as i32 - (self.scale.width as i32 / 2);
        let y = (self.scale.height / 2) as i32 - mouse_position.y as i32;
        let mouse_coord = Coordinate::from_pos((self.scale.width, self.scale.height), (x, y))
            .unwrap()
            .get_index();

        for i in 0..20 {
            let x_index =
                ((mouse_coord % self.scale.width) + (i * self.scale.width as u32)) as usize;
            drop(std::mem::replace(
                &mut self.buffer[x_index],
                0xFFFFFF as u32,
            ));
            self.mut_pixels.push(x_index as u32);
            let y_index = (mouse_coord - (mouse_coord % self.scale.width)) + i;
            drop(std::mem::replace(
                &mut self.buffer[y_index as usize],
                0xFFFFFF as u32,
            ));
            self.mut_pixels.push(y_index as u32);
        }
        self.draw(&mut self.mouse_coordinates(mouse_position));
    }

    pub fn mouse_coordinates(&self, mouse_position: PhysicalPosition<f64>) -> Vec<Drawable> {
        let x = mouse_position.x as f32 - (self.scale.width / 2) as f32;
        let y = (self.scale.height / 2) as f32 - mouse_position.y as f32;

        let x_txt = format!(
            "x:{}",
            f32::trunc((x as f32 / self.scale.factor_x) * 100.0) / 100.0
        )
        .to_string();
        let y_txt = format!(
            "y:{}",
            f32::trunc((y as f32 / self.scale.factor_y) * 100.0) / 100.0
        )
        .to_string();

        let x_coord = Coordinate::from_pos(
            (self.scale.width, self.scale.height),
            ((x.floor() as i32 + 3), (y.floor() as i32) + 26),
        )
        .unwrap();
        let y_coord = Coordinate::from_pos(
            (self.scale.width, self.scale.height),
            ((x.floor() as i32) + 3, (y.floor() as i32) + 10),
        )
        .unwrap();

        let mouse_txt_x = Text::from(x_txt, 0x00CC00 as u32, x_coord, true).unwrap();
        let mouse_txt_y = Text::from(y_txt, 0x00CC00 as u32, y_coord, true).unwrap();

        vec![Drawable::Text(mouse_txt_x), Drawable::Text(mouse_txt_y)]
    }

    pub fn clear_mut_pixels(&mut self) {
        for index in &self.mut_pixels {
            drop(std::mem::replace(
                &mut self.buffer[*index as usize],
                0x00 as u32,
            ));
        }
        self.mut_pixels = vec![];
    }
}
