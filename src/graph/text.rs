use super::{coordinate::Coordinate, Graph};
use std::io::Error;

#[derive(Debug)]
pub struct Text {
    pub buffer: Vec<Vec<u32>>,
    pub color: u32,
    pub string: String,
    pub position: Coordinate,
    pub is_mut: bool,
}

impl Text {
    pub fn _new() -> Text {
        Text {
            buffer: vec![vec![]],
            color: 0xFFFFFF as u32,
            string: "".to_string(),
            position: Coordinate::new(),
            is_mut: false,
        }
    }

    pub fn from(
        string: String,
        color: u32,
        position: Coordinate,
        is_mut: bool,
    ) -> Result<Text, Error> {
        let mut t = Text {
            buffer: vec![vec![]],
            color,
            string,
            position,
            is_mut,
        };

        t.init_buffer()?;

        Ok(t)
    }

    pub fn draw(&self, graph: &mut Graph) {
        let mut symbol_count = 0;
        let start_index = self.position.get_index();
        for symbol in &self.buffer {
            let mut index = start_index + (symbol_count * 9);
            let mut count = 0;
            for pixel in symbol {
                drop(std::mem::replace(&mut graph.buffer[index as usize], *pixel));
                if self.is_mut {
                    graph.mut_pixels.push(index);
                }
                index += 1;
                count += 1;
                if count % 6 == 0 {
                    index -= 6;
                    index += graph.scale.width;
                }
            }
            symbol_count += 1;
        }
    }

    fn init_buffer(&mut self) -> Result<(), Error> {
        let b = 0x000000 as u32;
        let t = 0xFFFFFF as u32;
        let c = self.color;
        for s in self.string.split("") {
            match s {
                "0" => {
                    self.buffer.push(vec![
                        b, c, c, c, c, b, b, c, c, c, c, b, b, c, b, b, c, b, b, c, b, b, c, b, b,
                        c, b, b, c, b, b, c, b, b, c, b, b, c, c, c, c, b, b, c, c, c, c, b,
                    ]);
                }
                "1" => {
                    self.buffer.push(vec![
                        b, b, c, c, b, b, b, c, c, c, b, b, b, c, c, c, b, b, b, b, c, c, b, b, b,
                        b, c, c, b, b, b, b, c, c, b, b, b, b, c, c, b, b, b, c, c, c, c, b,
                    ]);
                }
                "2" => {
                    self.buffer.push(vec![
                        b, c, c, c, c, b, b, c, c, c, c, b, b, b, b, c, c, b, b, b, b, c, c, b, b,
                        c, c, c, c, b, b, c, c, b, b, b, b, c, c, b, b, b, b, c, c, c, c, b,
                    ]);
                }
                "3" => {
                    self.buffer.push(vec![
                        b, c, c, c, c, b, b, c, c, c, c, b, b, b, b, b, c, b, b, b, c, c, c, b, b,
                        b, c, c, c, b, b, b, b, b, c, b, b, c, c, c, c, b, b, c, c, c, c, b,
                    ]);
                }
                "4" => {
                    self.buffer.push(vec![
                        b, b, b, c, c, b, b, b, c, c, c, b, b, c, b, c, c, b, c, b, b, c, c, b, c,
                        c, c, c, c, c, b, b, b, c, c, b, b, b, b, c, c, b, b, b, b, c, c, b,
                    ]);
                }
                "5" => {
                    self.buffer.push(vec![
                        b, c, c, c, c, b, b, c, c, c, c, b, b, c, c, b, b, b, b, c, c, c, c, b, b,
                        c, c, c, c, b, b, b, b, c, c, b, b, c, c, c, c, b, b, c, c, c, c, b,
                    ]);
                }

                "6" => {
                    self.buffer.push(vec![
                        b, c, c, c, c, b, b, c, b, b, b, b, b, c, b, b, b, b, b, c, c, c, c, b, b,
                        c, b, b, c, b, b, c, b, b, c, b, b, c, c, c, c, b, b, c, c, c, c, b,
                    ]);
                }
                "7" => {
                    self.buffer.push(vec![
                        b, c, c, c, c, c, b, c, c, c, c, c, b, b, b, c, c, b, b, b, b, c, c, b, b,
                        b, c, c, b, b, b, b, c, c, b, b, b, c, c, b, b, b, b, c, c, b, b, b,
                    ]);
                }
                "8" => {
                    self.buffer.push(vec![
                        b, c, c, c, c, b, b, c, b, b, c, b, b, c, b, b, c, b, b, c, c, c, c, b, b,
                        c, c, c, c, b, b, c, b, b, c, b, b, c, b, b, c, b, b, c, c, c, c, b,
                    ]);
                }
                "9" => {
                    self.buffer.push(vec![
                        b, c, c, c, c, b, b, c, b, b, c, b, b, c, b, b, c, b, b, c, c, c, c, b, b,
                        b, b, b, c, b, b, b, b, b, c, b, b, c, c, c, c, b, b, c, c, c, c, b,
                    ]);
                }
                "y" => {
                    self.buffer.push(vec![
                        b, b, b, b, b, b, b, t, b, b, t, b, b, t, b, b, t, b, b, t, b, b, t, b, b,
                        b, t, t, t, b, b, b, b, t, b, b, b, b, t, b, b, b, b, t, b, b, b, b,
                    ]);
                }
                "x" => {
                    self.buffer.push(vec![
                        b, b, b, b, b, b, b, t, b, b, t, b, b, t, b, b, t, b, b, b, t, t, b, b, b,
                        b, t, t, b, b, b, b, t, t, b, b, b, t, b, b, t, b, b, t, b, b, t, b,
                    ]);
                }
                ":" => {
                    self.buffer.push(vec![
                        b, b, b, b, b, b, b, b, t, t, b, b, b, b, t, t, b, b, b, b, b, b, b, b, b,
                        b, b, b, b, b, b, b, t, t, b, b, b, b, t, t, b, b, b, b, b, b, b, b,
                    ]);
                }
                " " => {
                    self.buffer.push(vec![
                        b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b,
                        b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b,
                    ]);
                }
                "." => {
                    self.buffer.push(vec![
                        b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b,
                        b, t, t, b, b, b, b, t, t, b, b, b, b, b, t, b, b, b, b, t, b, b, b,
                    ]);
                }
                ";" => {
                    self.buffer.push(vec![
                        b, b, b, b, b, b, b, b, t, t, b, b, b, b, t, t, b, b, b, b, b, b, b, b, b,
                        b, t, t, b, b, b, b, t, t, b, b, b, b, b, t, b, b, b, b, t, b, b, b,
                    ]);
                }
                "-" => {
                    self.buffer.push(vec![
                        b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, c, c, c, c, b, b,
                        c, c, c, c, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b,
                    ]);
                }
                _ => (),
            }
        }
        Ok(())
    }
}

/*
    b,b,b,b,b,b,
    b,b,b,b,b,b,
    b,b,b,b,b,b,
    b,b,b,b,b,b,
    b,b,b,b,b,b,
    b,b,b,b,b,b,
    b,b,b,b,b,b,
    b,b,b,b,b,b,
*/
