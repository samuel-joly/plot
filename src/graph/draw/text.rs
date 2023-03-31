use crate::graph::{color::Color, coordinate::Coordinate};
use ab_glyph::{Font, FontRef, Glyph};
use std::collections::HashMap;
use std::io::Error;

pub struct TextCompiler<'a> {
    font: FontRef<'a>,
    fontsize: f32,
    symbols: HashMap<char, Vec<(u32, u32, u32)>>,
}

impl<'a> TextCompiler<'a> {
    pub fn from(font: FontRef<'a>, fontsize: f32) -> TextCompiler<'a> {
        let mut tc = TextCompiler {
            font,
            fontsize,
            symbols: HashMap::new(),
        };
        tc.compile_text();
        tc
    }

    pub fn compile_text(&mut self) {
        let str = String::from("1234567890abcdefghijklmnopqrstuvwxyz,-:.ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        for s in str.chars() {
            let mut ret: Vec<(u32, u32, u32)> = vec![];
            let q_glyph: Glyph = self.font.glyph_id(s).with_scale(self.fontsize);

            if let Some(q) = self.font.outline_glyph(q_glyph) {
                q.draw(|x, mut y, c| {
                    if s == ',' || s == '.' {
                        y += 10;
                    }
                    if s == '-' {
                        y += 5;
                    }
                    let red = (255.0 * c).floor() as u32;
                    let green = (255.0 * c).floor() as u32;
                    let blue = (255.0 * c).floor() as u32;
                    let color = Color::create_color(red, green, blue).unwrap();
                    ret.push((x, y, color));
                });
                self.symbols.insert(s, ret);
            }
        }
    }
}

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

    pub fn draw(&mut self, size: (u32, u32), text_compiler: &TextCompiler) -> Vec<(u32, u32)> {
        let mut ret: Vec<(u32, u32)> = vec![];
        let start_index = self.position.get_index();
        let mut glyph_count = 0;
        for s in self.string.chars() {
            let symbol = text_compiler.symbols.get(&s).unwrap();
            for pix in symbol {
                let pixel = ((start_index + pix.0) + (pix.1 * size.0), pix.2);
                let new_pos = pixel.0 + (glyph_count * 9);
                ret.push((new_pos, pix.2));
            }
            glyph_count += 1;
        }
        ret
    }
}
