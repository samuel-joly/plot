use winit::dpi::PhysicalSize;

use crate::graph::{
    coordinate::Coordinate,
    draw::{text::Text, Drawable},
};

use super::color::Color;

#[derive(Debug)]
pub enum Orientation {
    _FromZero,
    _ToZero,
    Centered,
}

#[derive(Debug)]
pub struct Scale {
    pub factor_x: f32,
    pub factor_y: f32,

    pub width: u32,
    pub height: u32,

    pub original_interval_x: f32,
    pub original_interval_y: f32,

    pub current_interval_x: f32,
    pub current_interval_y: f32,

    pub orientation: Orientation,
}

impl Scale {
    pub fn new() -> Scale {
        Scale {
            factor_x: 0.00,
            factor_y: 0.00,
            width: 0,
            height: 0,
            original_interval_x: 0.00,
            original_interval_y: 0.00,
            current_interval_x: 0.00,
            current_interval_y: 0.00,
            orientation: Orientation::Centered,
        }
    }

    pub fn _from(x: f32, y: f32, width: u32, height: u32, orientation: Orientation) -> Scale {
        let sc = Scale {
            factor_x: 0.0,
            factor_y: 0.0,
            width,
            height,
            original_interval_x: x,
            original_interval_y: y,
            current_interval_x: x,
            current_interval_y: y,
            orientation,
        };
        sc
    }

    pub fn set_size(&mut self, size: PhysicalSize<u32>) {
        self.width = size.width;
        self.height = size.height;
        self.set_scale_factor();
    }

    pub fn set_scale(&mut self, x_interval: f32, y_interval: f32) {
        self.current_interval_x = x_interval;
        self.current_interval_y = y_interval;
        if self.original_interval_x == 0.0 && self.original_interval_y == 0.0 {
            self.original_interval_x = x_interval;
            self.original_interval_y = y_interval;
        }

        self.set_scale_factor()
    }

    fn set_scale_factor(&mut self) {
        self.factor_x = self.width as f32 / self.current_interval_x as f32;
        self.factor_y = self.height as f32 / self.current_interval_y as f32;
    }

    pub fn draw(&self, background_color: u32, foreground_color:u32) -> Vec<(u32, u32)> {
        let mut interval_texts = vec![];
        let width: u32 = self.width;
        let height: u32 = self.height;

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
                interval_texts.push((index_y, foreground_color));
                interval_texts.push((index_x, foreground_color));
                index_y += 1;
                index_x += width;
            }

            if i % 2 == 0 {
                let x_val = f32::trunc((line_x.0 as f32 / self.factor_x) * 100.0) / 100.0;
                let y_val = f32::trunc((-line_y.1 as f32 / self.factor_y) * 100.0) / 100.0;

                let text_x = x_val.to_string();
                let text_y = y_val.to_string();

                let coord_text_x = Coordinate::from_index(
                    (width, height),
                    index_x - (6 * text_x.len() as u32) + (5 * width),
                )
                .unwrap();
                let coord_text_y =
                    Coordinate::from_index((width, height), index_y - (3 * width)).unwrap();

                let text_alt = Color::create_color(100,100,100).unwrap();
                let mut scale_text_x = Text::from(
                    text_x,
                    coord_text_x,
                    Some(true),
                    foreground_color,
                    background_color,
                    text_alt
                )
                .unwrap();
                let mut scale_text_y = Text::from(
                    text_y,
                    coord_text_y,
                    Some(true),
                    foreground_color,
                    background_color,
                    text_alt
                )
                .unwrap();

                for index in scale_text_x.draw((self.width, self.height)) {
                    interval_texts.push((index.0, index.1));
                }
                for index in scale_text_y.draw((self.width, self.height)) {
                    interval_texts.push((index.0, index.1));
                }
            }
        }
        interval_texts
    }
}
