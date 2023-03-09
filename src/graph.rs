pub mod color;
pub mod coordinate;
pub mod draw;
pub mod mouse_info;
pub mod scale;

use crate::graph::{draw::Drawable, mouse_info::Mouse, scale::Scale};

use ab_glyph::FontRef;

use self::color::Color;

pub struct Graph<'a> {
    pub buffer: Vec<u32>,
    pub mut_pixels: Vec<u32>,
    pub scale: Scale,
    pub shapes: Vec<Box<dyn Drawable>>,
    pub background: u32,
    pub foreground: u32,
    pub font: FontRef<'a>,
    pub mouse: Mouse,
}

impl<'a> Graph<'a> {
    pub fn new() -> Graph<'a> {
        Graph {
            buffer: Vec::new(),
            mut_pixels: vec![],
            scale: Scale::new(),
            shapes: vec![],
            background: 0x000000,
            foreground: 0xFFFFFF,
            font: FontRef::try_from_slice(include_bytes!(
                "/home/azefortwo/.local/share/fonts/LibreBaskerville-Italic.otf"
            ))
            .unwrap(),
            mouse: Mouse::new(),
        }
    }

    pub fn fill_buffer(&mut self, color: u32) {
        self.buffer = (0..((self.scale.width * self.scale.height) as usize))
            .map(|_| color)
            .collect::<Vec<u32>>();
        self.clear_mut_pixels();
    }

    pub fn clear_mut_pixels(&mut self) {
        self.mut_pixels = vec![];
    }

    pub fn _draw_shapes(&mut self) {
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

    pub fn draw_points(&mut self, points: &Vec<u32>) {
        for point in points {
            drop(std::mem::replace(
                &mut self.buffer[*point as usize],
                Color::create_color(0, 255, 0).unwrap(),
            ));
        }
    }

    pub fn draw_mouse_info(&mut self) {
        for (pix, _) in &self.mouse.position_pixels {
            drop(std::mem::replace(
                &mut self.buffer[*pix as usize],
                self.background,
            ));
        }
        for pix in &self.mouse.axis_pixels {
            drop(std::mem::replace(
                &mut self.buffer[*pix as usize],
                self.background,
            ));
        }

        self.mouse
            .draw_mouse_position(&self.scale, &self.font, 15.0);
        self.mouse.draw_mouse_axis(&self.scale);

        for (pix, color) in &self.mouse.position_pixels {
            drop(std::mem::replace(&mut self.buffer[*pix as usize], *color));
        }
        for pix in &self.mouse.axis_pixels {
            drop(std::mem::replace(
                &mut self.buffer[*pix as usize],
                self.foreground,
            ));
        }
    }

    pub fn draw_scale(&mut self) {
        for index in &self.scale.mut_pixels {
            drop(std::mem::replace(
                &mut self.buffer[*index as usize],
                self.background,
            ));
        }
        self.scale.mut_pixels = vec![];
        for index in self.scale.draw(self.foreground, &self.font) {
            self.mut_pixels.push(index.0);
            drop(std::mem::replace(
                &mut self.buffer[index.0 as usize],
                index.1,
            ));
        }
    }
}
