use std::io::Error;

#[derive(Debug)]
pub struct Coordinate {
    cartesian: (i32, i32),
    index: u32,
}

impl Coordinate {
    pub fn new() -> Coordinate {
        Coordinate {
            cartesian: (0, 0),
            index: 0,
        }
    }

    pub fn get_pos(&self) -> (i32, i32) {
        self.cartesian
    }

    pub fn get_index(&self) -> u32 {
        self.index
    }

    pub fn from_pos(realsize: (u32,u32), pos: (i32, i32)) -> Result<Coordinate,Error> {
        let posx = pos.0 ;
        let posy = pos.1 ;

        if realsize.0 == 0 || realsize.1 == 0 {
            return Ok(Coordinate::new());
        }
        if posx > realsize.0 as i32 / 2 || posx < -(realsize.0 as i32 / 2) {
            return Ok(Coordinate::new());
        }

        if posy > realsize.1 as i32 / 2 || posy < -(realsize.1 as i32 / 2) {
            return Ok(Coordinate::new());
        }

        let index = (realsize.0 / 2) as i32 + posx
            + (((realsize.0 * realsize.1) / 2) as i32)
            - (realsize.0 as i32 * posy);

//        if index >= (realsize.0 * realsize.1) as i32 {
//             index -= index - (realsize.0 * realsize.1) as i32 + 1;
//        }
        Ok(Coordinate {
            cartesian: pos,
            index: index as u32,
        })
    }

    pub fn from_index(realsize: (u32,u32), mut index: u32) -> Result<Coordinate, Error> {
        if index >= (realsize.0 * realsize.1) {
            index -= index - (realsize.0 * realsize.1) -1;
        }

        let y = (index / realsize.0) as i32 - (realsize.1/2) as i32;
        let x = (index % realsize.0) as i32 - realsize.0 as i32/2;

        Ok(Coordinate {
            cartesian: (x as i32, y as i32),
            index,
        })
    }

    pub fn substr(coordinate: &Coordinate, coordinate_2: &Coordinate) -> (i32,i32) {
        let cartesian = (
            (coordinate.cartesian.0 - coordinate_2.cartesian.0),
            (coordinate.cartesian.1 - coordinate_2.cartesian.1),
        );
        cartesian
    }
}
