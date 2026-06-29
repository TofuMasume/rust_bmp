#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Rgb>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        let black = Rgb { r: 0, g: 0, b: 0 };

        Self {
            width,
            height,
            pixels: vec![black; (width * height) as usize],
        }
    }

    fn index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
        // y →
        // x ↓
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<Rgb> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(self.pixels[self.index(x, y)])
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, rgb: Rgb) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }

        let index = self.index(x, y);
        self.pixels[index] = rgb;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_image_has_given_size() {
        let image = Image::new(2, 1);

        assert_eq!(image.width(), 2);
        assert_eq!(image.height(), 1);
    }

    #[test]
    fn new_image_is_filled_with_black() {
        let image = Image::new(1, 1);

        assert_eq!(image.get_pixel(0, 0), Some(Rgb { r: 0, g: 0, b: 0 }));
    }

    #[test]
    fn set_pixel_updates_pixels() {
        let mut image = Image::new(2, 1);
        let red = Rgb { r: 255, g: 0, b: 0 };

        assert!(image.set_pixel(0, 0, red));
        assert_eq!(image.get_pixel(0, 0), Some(Rgb { r: 255, g: 0, b: 0 }));
    }
}
