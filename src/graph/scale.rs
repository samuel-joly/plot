use winit::dpi::PhysicalSize;

#[derive(Debug)]
pub enum Orientation {
    _FromZero,
    _ToZero,
    Centered,
}

#[derive(Debug)]
pub struct Scale {
    pub factor_x: f32,
    pub factor_y: f32,
    pub width: u32,
    pub height: u32,
    pub og_interval_x: f32,
    pub og_interval_y: f32,
    pub interval_x: f32,
    pub interval_y: f32,
    pub orientation: Orientation
}

impl Scale {
    pub fn new () -> Scale {
        Scale {
            factor_x: 0.00,
            factor_y: 0.00,
            width: 0,
            height: 0,
            og_interval_x: 0.00,
            og_interval_y: 0.00,
            interval_x: 0.00,
            interval_y: 0.00,
            orientation: Orientation::Centered,
        }
    }

    pub fn _from(x: f32, y: f32, width: u32, height: u32, orientation: Orientation) -> Scale {
        let sc = Scale {
            factor_x: 0.0,
            factor_y: 0.0,
            width,
            height,
            og_interval_x: x,
            og_interval_y: y,
            interval_x: x,
            interval_y: y,
            orientation,
        };
        sc
    }

    pub fn set_size(&mut self, size:PhysicalSize<u32>) {
        self.width = size.width;
        self.height = size.height;
        self.set_scale_factor();
    }

    fn set_scale_factor(&mut self) {
        self.factor_x = self.width as f32 / self.interval_x as f32;
        self.factor_y = self.height as f32 / self.interval_y as f32;
    }

    pub fn set_scale(&mut self, x_interval: f32, y_interval: f32) {
        self.interval_x = x_interval;
        self.interval_y = y_interval;
        if self.og_interval_x == 0.0 && self.og_interval_y == 0.0 {
            self.og_interval_x = x_interval;
            self.og_interval_y = y_interval;
        }

        self.set_scale_factor()
    }
}
