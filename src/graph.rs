use softbuffer::GraphicsContext;
use winit::dpi::PhysicalSize;

#[derive(Debug)]
pub struct Graph {
    width: u32,
    height: u32,
    buffer: Vec<u32>,
    offset_x: i32,
    offset_y: i32,
    offset_start_x: u32,
    offset_start_y: u32,
    old_offset_x: i32,
    old_offset_y:i32,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            width: 0,
            height: 0,
            buffer: Vec::new(),
            offset_x: 0,
            offset_y: 0,
            offset_start_x: 0,
            offset_start_y: 0,
            old_offset_x:0,
            old_offset_y:0,
        }
    }

    pub fn draw_plane(&mut self, gc: &mut GraphicsContext) {
        let buffer = (0..((self.width * self.height) as usize))
            .map(|index| {
                let (y, x) = self.get_pos(index);

                let width = (self.width as i32- self.offset_x) as usize;
                let height = (self.height as i32 - self.offset_y) as usize;

                let black = 0x00;
                let white = 0xFFFFFF;

                // Draw center lines
                if x > (width/ 2) - 2 && x < (width/ 2) + 2 {
                    black
                } else if y > (height/ 2) - 2 && y < (height/ 2) + 2 {
                    black
                } else {
                    white
                }
            })
            .collect::<Vec<_>>();

        gc.set_buffer(&buffer, self.width as u16, self.height as u16);
        self.buffer = buffer;
    }

    fn get_pos(&self, index: usize) -> (usize, usize) {
        (index / (self.width as usize), index % (self.width as usize))
    }

    pub fn set_size(&mut self, size: PhysicalSize<u32>) {
        self.width = size.width;
        self.height = size.height;
    }

    pub fn prepare_movement(&mut self, x:u32, y:u32) {
        self.set_offset_start(x,y);
        self.set_old_offsets();
    }

    fn set_offset_start(&mut self ,x: u32, y:u32) {
        self.offset_start_x = x;
        self.offset_start_y = y;
    }

    fn set_old_offsets(&mut self) {
        self.old_offset_x = self.offset_x;
        self.old_offset_y = self.offset_y;
    }

    pub fn move_graph_to(&mut self ,x: i32, y:i32) {
        self.offset_x = (self.offset_start_x as i32 - x) + self.old_offset_x ;
        self.offset_y = (self.offset_start_y as i32 - y) + self.old_offset_y ;
    }

}
