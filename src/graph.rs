use coordinate::Coordinate;
use line::Line;
use scale::Scale;
use text::Text;
use winit::dpi::{PhysicalPosition, PhysicalSize};

pub mod coordinate;
pub mod line;
mod offset;
pub mod scale;
pub mod text;

pub enum Drawable {
    Line(Line),
    Text(Text),
}

#[derive(Debug)]
pub struct Graph {
    pub buffer: Vec<u32>,
    pub mut_pixels: Vec<u32>,
    pub scale: Scale,
}

impl Drawable {
    fn draw_shape(&mut self, graph: &mut Graph) {
        match self {
            Drawable::Line(line) => {
                if line.scaled == false {
                    line.from = (
                        (line.og_from.0 as f32 * graph.scale.factor_x).floor() as i32,
                        (line.og_from.1 as f32 * graph.scale.factor_y).floor() as i32,
                    );
                    line.to = (
                        (line.og_to.0 as f32 * graph.scale.factor_x).floor() as i32,
                        (line.og_to.1 as f32 * graph.scale.factor_y).floor() as i32,
                    );
                    line.scaled = true;
                }
                line.draw(graph)
            }
            Drawable::Text(text) => text.draw(graph),
        }
    }
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

        for i in 1..10 {
            let mut coord_y =
                i * ((self.scale.height as f32 / 10.0) * self.scale.width as f32).floor() as u32;
            let coord_x = (i as f32 * (self.scale.width as f32 / 10.0)).floor() as u32;

            coord_y -= coord_y % self.scale.width;
            coord_y += self.scale.width;

            let line_s_x =
                Coordinate::from_index((self.scale.width, self.scale.height), coord_x).unwrap();
            let line_s_y =
                Coordinate::from_index((self.scale.width, self.scale.height), coord_y).unwrap();
            let line_start_x = line_s_x.get_pos();
            let line_start_y = line_s_y.get_pos();

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
                index_x += self.scale.width;
            }

            if i % 2 == 0 {
                let mut x_val = line_start_x.0 as f32 / self.scale.factor_x;
                x_val = f32::trunc(x_val * 100.0) / 100.0;
                let mut y_val = -line_start_y.1 as f32 / self.scale.factor_y;
                y_val = f32::trunc(y_val * 100.0) / 100.0;
                let text_x = x_val.to_string();
                let text_y = y_val.to_string();

                let coord_text_x = Coordinate::from_index(
                    (self.scale.width, self.scale.height),
                    index_x - (6 * text_x.len() as u32) + (5 * self.scale.width),
                )
                .unwrap();
                let coord_text_y = Coordinate::from_index(
                    (self.scale.width, self.scale.height),
                    index_y - (3 * self.scale.width),
                )
                .unwrap();

                let scale_text_x =
                    Text::from(text_x, 0xFFFFFF as u32, coord_text_x, true).unwrap();
                let scale_text_y =
                    Text::from(text_y, 0xFFFFFF as u32, coord_text_y, true).unwrap();
                scale_shape.push(Drawable::Text(scale_text_x));
                scale_shape.push(Drawable::Text(scale_text_y));
            }
        }
        self.draw(&mut scale_shape);
    }

    pub fn set_size(&mut self, size: PhysicalSize<u32>) {
        self.mut_pixels = vec![];
        self.scale.set_size(size);
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
        }

        for i in 0..20 {
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

        let mouse_txt_x = Text::from(
            format!(
                "x:{}",
                f32::trunc((x as f32 / self.scale.factor_x) * 100.0) / 100.0
            )
            .to_string(),
            0x00CC00 as u32,
            Coordinate::from_pos(
                (self.scale.width, self.scale.height),
                ((x.floor() as i32 + 3), (y.floor() as i32) + 26),
            )
            .unwrap(),
            true,
        )
        .unwrap();
        let mouse_txt_y = Text::from(
            format!(
                "y:{}",
                f32::trunc((y as f32 / self.scale.factor_y) * 100.0) / 100.0
            )
            .to_string(),
            0x00CC00 as u32,
            Coordinate::from_pos(
                (self.scale.width, self.scale.height),
                ((x.floor() as i32) + 3, (y.floor() as i32) + 10),
            )
            .unwrap(),
            true,
        )
        .unwrap();

        vec![Drawable::Text(mouse_txt_x), Drawable::Text(mouse_txt_y)]
    }
}
