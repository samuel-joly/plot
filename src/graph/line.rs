/// Colored straight line
#[derive(Debug)]
pub struct Line {
    /// `from` and `to` are define as tupe(x,y)
    pub from: (i32,i32),
    pub to: (i32,i32),

    /// Color for the line, `softbuffer` color formatting is used
    pub color: u32,

    /// Is it not clear enough ?
    pub is_drawed: bool,
}

impl Line {
    pub fn from(start_pos: (i32, i32), end_pos: (i32, i32), color: u32) -> Line {
        Line {
            from: start_pos,
            to: end_pos,
            color,
            is_drawed: false,
        }
    }

    pub fn _new() -> Line {
        Line {
            from: (0,0),
            to: (0,0),
            color: 0xFFFFFF as u32,
            is_drawed: false,
        }
    }
}
