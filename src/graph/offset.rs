#[derive(Debug)]
pub struct Offset {
    pub x: i32,
    pub y: i32,
    _cumul_x: i32,
    _cumul_y:i32,
    _start_x: u32,
    _start_y: u32,
}

impl Offset {
    pub fn _new() -> Offset{
        Offset{
            x:0,
            y:0,
            _cumul_x:0,
            _cumul_y:0,
            _start_x:0,
            _start_y:0,
        }
    }

    fn _set_start(&mut self ,x: u32, y:u32) {
        self._start_x = x;
        self._start_y = y;
    }

    fn _set_cumul(&mut self) {
        self._cumul_x = self.x;
        self._cumul_y = self.y;
    }

    pub fn _diff_drag_to_offset(&mut self ,x: i32, y:i32) {
        self.x = (self._start_x as i32 - x) + self._cumul_x ;
        self.y = (self._start_y as i32 - y) + self._cumul_y ;
    }

    pub fn _set_offset(&mut self, x:i32, y:i32) {
        self.x = x;
        self.y = y;
    }

    pub fn _prepare_movement(&mut self, x:u32, y:u32) {
        self._set_start(x,y);
        self._set_cumul();
    }

    pub fn _reset_cumul(&mut self) {
        self._cumul_x = 0;
        self._cumul_y = 0;
    }
}
