use std::io::Error;

pub struct Color {}

impl Color {
    pub fn create_color(red: u32, green: u32, blue: u32) -> Result<u32, Error> {
        if red > 256 || green > 256 || blue > 256 {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("red green and blue should be lower or equal than 255, {} {} {} given", red, green, blue),
            ));
        }

        let color = blue | (green << 8) | (red << 16);
        Ok(color)
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn create_color_red() {
        let red = Color::create_color(255, 0, 0).unwrap();
        assert!(
            0xFF0000 as u32 == red,
            "\nred is equal to {:#x}({0})\nshould be 0xFF0000({1})\n",
            red as u32,
            0xFF0000 as u32,
        );
    }

    #[test]
    fn create_color_green() {
        let green = Color::create_color(0, 255, 0).unwrap();
        assert!(
            0x00FF00 as u32 == green,
            "\ngreen is equal to {:#x}({0})\nshould be 0x00FF00({1})\n",
            green as u32,
            0x00FF00 as u32,
        );
    }

    #[test]
    fn create_color_blue() {
        let blue = Color::create_color(0, 0, 255).unwrap();
        assert!(
            0x0000FF as u32 == blue,
            "\nblue is equal to {:#x}({0})\nshould be 0x0000FF({1})\n",
            blue as u32,
            0x0000FF as u32,
        );
    }

    #[test]
    fn create_color_white() {
        let white = Color::create_color(255, 255, 255).unwrap();
        assert!(
            0xFFFFFF as u32 == white,
            "\nblue is equal to {:#x}({0})\nshould be 0x0000FF({1})\n",
            white as u32,
            0x0000FF as u32,
        );
    }
}
