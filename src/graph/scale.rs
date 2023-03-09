use ab_glyph::FontRef;
use winit::dpi::PhysicalSize;

use crate::graph::{coordinate::Coordinate, draw::text::Text};

#[derive(Debug)]
pub enum Orientation {
    _FromZero,
    _ToZero,
    Even,
}

#[derive(Debug)]
pub enum Display {
    _Relative,
    Absolute,
}

#[derive(Debug)]
pub enum Position {
    Centered,
    LeftTop,
    RightBottom,
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
    pub position: Position,
    pub display: Display,

    pub mut_pixels: Vec<u32>,
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
            orientation: Orientation::Even,
            position: Position::Centered,
            display: Display::Absolute,
            mut_pixels: vec![],
        }
    }

    pub fn _from(
        x: f32,
        y: f32,
        width: u32,
        height: u32,
        orientation: Orientation,
        position: Position,
        display: Display,
    ) -> Scale {
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
            position,
            display,
            mut_pixels: vec![],
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
        if self.height % 2 != 0 {
            self.height += 1;
        }
        if self.width % 2 != 0 {
            self.width += 1;
        }
        self.factor_x = self.width as f32 / self.current_interval_x as f32;
        self.factor_y = self.height as f32 / self.current_interval_y as f32;
    }

    pub fn draw(&mut self, foreground_color: u32, font: &FontRef) -> Vec<(u32, u32)> {
        let mut interval_texts = vec![];
        let width: u32 = self.width;
        let height: u32 = self.height;
        match self.position {
            Position::Centered => {
                for i in 0..height {
                    interval_texts.push(((width / 2) + width * i, foreground_color));
                }
                for i in 0..width {
                    interval_texts.push((((height * width) / 2) + i, foreground_color));
                }
            }
            _ => {}
        }

        for i in 1..10 {
            let mut coord_y = i * ((height as f32 / 10.0) * width as f32).floor() as u32;
            // Weird equalization of y axis, element shift by an unindentified value each increment
            coord_y -= coord_y % width;
            coord_y += width;
            let mut coord_x = (i * (width / 10)) as u32;

            match self.position {
                Position::Centered => {
                    coord_x += (height / 2) * width;
                    coord_y += width / 2;
                }
                Position::LeftTop => {}
                Position::RightBottom => {
                    coord_x = (height * width) - coord_x;
                    coord_y += width - 1;
                }
            }

            self.draw_interval(coord_x, coord_y, &mut interval_texts, foreground_color);
            if i % 2 == 0 {
                self.draw_interval_text(coord_x, coord_y, &mut interval_texts, font);
            }
        }
        interval_texts
    }

    pub fn draw_interval(
        &self,
        mut index_x: u32,
        mut index_y: u32,
        interval_texts: &mut Vec<(u32, u32)>,
        foreground_color: u32,
    ) {
        for _ in 0..9 {
            interval_texts.push((index_y - 4, foreground_color));
            interval_texts.push((index_x - (4 * self.width), foreground_color));
            match self.position {
                Position::RightBottom => {
                    index_y -= 1;
                    index_x -= self.width;
                }
                _ => {
                    index_y += 1;
                    index_x += self.width;
                }
            };
        }
    }

    pub fn draw_interval_text(
        &mut self,
        coord_x: u32,
        coord_y: u32,
        interval_texts: &mut Vec<(u32, u32)>,
        font: &FontRef,
    ) {
        let line_x = Coordinate::from_index((self.width, self.height), coord_x)
            .unwrap()
            .get_pos();
        let line_y = Coordinate::from_index((self.width, self.height), coord_y)
            .unwrap()
            .get_pos();

        let text_x = (f32::trunc((line_x.0 as f32 / self.factor_x) * 100.0) / 100.0).to_string();
        let text_y = (f32::trunc((-line_y.1 as f32 / self.factor_y) * 100.0) / 100.0).to_string();

        let mut ctx = coord_x - (6 * text_x.len() as u32);
        let mut cty = coord_y - 3 * self.width + 10;

        match self.position {
            Position::RightBottom => {
                ctx -= 15 * self.width;
                cty -= 11 * text_y.len() as u32;
            }
            _ => {
                ctx += 11 * self.width;
            }
        }

        let coord_text_x = Coordinate::from_index((self.width, self.height), ctx).unwrap();
        let mut scale_text_x = Text::from(text_x, coord_text_x, Some(true)).unwrap();

        let coord_text_y = Coordinate::from_index((self.width, self.height), cty).unwrap();
        let mut scale_text_y = Text::from(text_y, coord_text_y, Some(true)).unwrap();

        for index in scale_text_x.draw((self.width, self.height), 17.0, font) {
            interval_texts.push((index.0, index.1));
            self.mut_pixels.push(index.0);
        }
        for index in scale_text_y.draw((self.width, self.height), 17.0, font) {
            interval_texts.push((index.0, index.1));
            self.mut_pixels.push(index.0);
        }
    }
}
