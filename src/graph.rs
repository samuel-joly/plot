pub mod color;
pub mod coordinate;
pub mod draw;
pub mod scale;

use crate::graph::{
    coordinate::Coordinate,
    draw::{text::Text, Drawable},
    scale::Scale,
};

use winit::dpi::PhysicalPosition;

use self::color::Color;

pub struct Graph {
    pub buffer: Vec<u32>,
    pub mut_pixels: Vec<u32>,
    pub scale: Scale,
    pub shapes: Vec<Box<dyn Drawable>>,
    pub mouse_text: (Box<dyn Drawable>, Box<dyn Drawable>),
    pub background: u32,
    pub foreground: u32,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            buffer: Vec::new(),
            mut_pixels: vec![],
            scale: Scale::new(),
            shapes: vec![],
            mouse_text: (Box::new(Text::new()), Box::new(Text::new())),
            background: 0x000000,
            foreground: 0xFFFFFF,
        }
    }

    pub fn draw_shapes(&mut self) {
        for shape in self.shapes.iter_mut() {
            if shape.is_mut() {
                for index in shape.get_mut_pixels() {
                    drop(std::mem::replace(
                        &mut self.buffer[index as usize],
                        self.background,
                    ));
                }
            }
            if shape.is_scalable() {
                shape.scale(&self.scale);
            }
            for index in shape.draw((self.scale.width, self.scale.height)) {
                drop(std::mem::replace(
                    &mut self.buffer[index.0 as usize],
                    index.1,
                ));
            }
        }
    }

    pub fn draw_scale(&mut self) {
        self.clear_scale();
        for index in self.scale.draw(self.background, self.foreground) {
            self.mut_pixels.push(index.0);
            drop(std::mem::replace(
                &mut self.buffer[index.0 as usize],
                index.1,
            ));
        }
    }

    pub fn clear_scale(&mut self) {
        for index in &self.mut_pixels {
            drop(std::mem::replace(
                &mut self.buffer[*index as usize],
                self.background,
            ));
        }
        self.mut_pixels = vec![];
    }

    pub fn fill_buffer(&mut self, color: u32) {
        self.buffer = (0..((self.scale.width * self.scale.height) as usize))
            .map(|_| color)
            .collect::<Vec<u32>>();
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
            let y_index = (mouse_coord - (mouse_coord % self.scale.width)) + i;
            drop(std::mem::replace(
                &mut self.buffer[x_index],
                self.foreground,
            ));
            drop(std::mem::replace(
                &mut self.buffer[y_index as usize],
                self.foreground,
            ));
            self.mut_pixels.push(x_index as u32);
            self.mut_pixels.push(y_index as u32);
        }
    }

    pub fn draw_mouse_coordinates(&mut self, mouse_position: PhysicalPosition<f64>) {
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

        let mouse_txt_alt = Color::create_color(122, 122, 122).unwrap();
        let mouse_txt_x = Text::from(
            x_txt,
            x_coord,
            Some(true),
            self.foreground,
            self.background,
            mouse_txt_alt,
        )
        .unwrap();
        let mouse_txt_y = Text::from(
            y_txt,
            y_coord,
            Some(true),
            self.foreground,
            self.background,
            mouse_txt_alt,
        )
        .unwrap();

        self.mouse_text = (Box::new(mouse_txt_x),Box::new(mouse_txt_y));
        for (pixel, color) in self.mouse_text.0.draw((self.scale.width, self.scale.height)) {
            self.mut_pixels.push(pixel);
            drop(std::mem::replace(&mut self.buffer[pixel as usize], color));
        }
        for (pixel, color) in self.mouse_text.1.draw((self.scale.width, self.scale.height)) {
            self.mut_pixels.push(pixel);
            drop(std::mem::replace(&mut self.buffer[pixel as usize], color));
        }
    }
}
