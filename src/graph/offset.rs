#[derive(Debug)]
pub struct Offset {
    pub x: i32,
    pub y: i32,
    cumul_x: i32,
    cumul_y:i32,
    start_x: u32,
    start_y: u32,
}

impl Offset {
    pub fn new() -> Offset{
        Offset{
            x:0,
            y:0,
            cumul_x:0,
            cumul_y:0,
            start_x:0,
            start_y:0,
        }
    }

    fn set_start(&mut self ,x: u32, y:u32) {
        self.start_x = x;
        self.start_y = y;
    }

    fn set_cumul(&mut self) {
        self.cumul_x = self.x;
        self.cumul_y = self.y;
    }

    pub fn diff_drag_to_offset(&mut self ,x: i32, y:i32) {
        self.x = (self.start_x as i32 - x) + self.cumul_x ;
        self.y = (self.start_y as i32 - y) + self.cumul_y ;
    }

    pub fn _set_offset(&mut self, x:i32, y:i32) {
        self.x = x;
        self.y = y;
    }

    pub fn prepare_movement(&mut self, x:u32, y:u32) {
        self.set_start(x,y);
        self.set_cumul();
    }
}
