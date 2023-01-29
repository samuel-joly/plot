pub mod coordinate;
pub mod line;
mod offset;
pub mod text;
use line::Line;

use offset::Offset;
use winit::dpi::{PhysicalSize, PhysicalPosition};

pub use text::Text;

use self::coordinate::Coordinate;

pub enum Drawable {
    Line(Line),
    Text(Text),
}

impl Drawable {
    fn draw_shape(&self, graph: &mut Graph) {
        match self {
            Drawable::Line(line) => line.draw(graph),
            Drawable::Text(text) => text.draw(graph),
        }
    }
}

#[derive(Debug)]
pub struct Graph {
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<u32>,
    pub offset: Offset,
    pub mut_pixels: Vec<u32>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            width: 0,
            height: 0,
            buffer: Vec::new(),
            offset: Offset::new(),
            mut_pixels: vec![],
        }
    }

    pub fn init_buffer(&mut self, color: u32, width: u32, height: u32) {
        self.buffer = (0..((width * height) as usize))
            .map(|_| color)
            .collect::<Vec<_>>();
    }

    pub fn draw(&mut self, shapes: &Vec<Drawable>) {
        for shape in shapes {
            shape.draw_shape(self);
        }
    }

    pub fn draw_axis(&mut self) {
        let half_size = (self.width * self.height) / 2;
        for i_column in 1..self.width {
            let next_index = ((half_size + i_column as u32) as i32) as usize;
            drop(std::mem::replace(
                &mut self.buffer[next_index],
                0xFFFFFF as u32,
            ));
        }

        let half_width = self.width / 2;
        for i_row in 0..self.height {
            let next_index = ((self.width * i_row) + half_width) as usize;
            drop(std::mem::replace(
                &mut self.buffer[next_index],
                0xFFFFFF as u32,
            ));
        }
    }

    pub fn set_size(&mut self, size: PhysicalSize<u32>) {
        self.width = size.width;
        self.height = size.height;
    }

    pub fn get_mouse_axis(&self, position: (i32, i32)) -> (Line, Line) {
        let x_cursor = Line::from(
            (position.0, 0),
            (position.0, position.1),
            0x0000CC as u32,
            true,
        );

        let y_cursor = Line::from(
            (0, position.1),
            (position.0, position.1),
            0x0000CC as u32,
            true,
        );
        (x_cursor, y_cursor)
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

    pub fn mouse_coordinates(&self, mouse_position: PhysicalPosition<f64>) -> Vec<Drawable> {
        let x = mouse_position.x.floor() as i32 - (self.width / 2) as i32;
        let y = (self.height / 2) as i32 - mouse_position.y.floor() as i32;
        let (mouse_axis_x, mouse_axis_y) = self.get_mouse_axis((x, y+3));

        let mouse_txt = Text::from(
            format!("x:{} y:{}", x, y).to_string(),
            0x00CC00 as u32,
            Coordinate::from_pos(&self, (x + 3, y + 10)).unwrap(),
        )
        .unwrap();

        vec![
            Drawable::Text(mouse_txt),
            Drawable::Line(mouse_axis_x),
            Drawable::Line(mouse_axis_y),
        ]
    }
}
