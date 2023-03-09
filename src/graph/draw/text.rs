use crate::graph::{color::Color, coordinate::Coordinate};
use ab_glyph::{Font, FontRef, Glyph};
use std::io::Error;

#[derive(Debug)]
pub struct Text {
    pub string: String,
    pub position: Coordinate,
    pub is_mut: bool,
}

impl Text {
    pub fn _new() -> Text {
        Text {
            string: "".to_string(),
            position: Coordinate::new(),
            is_mut: false,
        }
    }

    pub fn from(string: String, position: Coordinate, is_mut: Option<bool>) -> Result<Text, Error> {
        let t = Text {
            string,
            position,
            is_mut: is_mut.unwrap_or(false),
        };

        Ok(t)
    }

    pub fn draw(&mut self, size: (u32, u32), fontsize: f32, font: &FontRef) -> Vec<(u32, u32)> {
        let mut ret: Vec<(u32, u32)> = vec![];
        let start_index = self.position.get_index();
        let mut glyph_count = 0;
        let vals = self.string.clone();
        for s in vals.chars() {
            let q_glyph: Glyph = font.glyph_id(s).with_scale(fontsize);

            if let Some(q) = font.outline_glyph(q_glyph) {
                q.draw(|mut x, mut y, c| {
                    if s == '.' || s == ',' || s == '_' {
                        y += 10;
                        if s == '.' {
                            x += 2;
                        }
                    }
                    if s == '-' {
                        y += 5;
                    }
                    let pixel = ((start_index + x) + (y * size.0), c);
                    let new_pos = pixel.0 + (glyph_count * 9);
                    let red = (255.0 * pixel.1).floor() as u32;
                    let green = (255.0 * pixel.1).floor() as u32;
                    let blue = (255.0 * pixel.1).floor() as u32;
                    let color = Color::create_color(red, green, blue).unwrap();
                    ret.push((new_pos, color));
                });
                glyph_count += 1;
            }
        }
        ret
    }
}
