#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageError {
    OutOfBounds { x: u32, y: u32 },
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

    fn index(&self, x: u32, y: u32) -> Result<usize, ImageError> {
        if x >= self.width || y >= self.height {
            return Err(ImageError::OutOfBounds { x, y });
        }
        Ok((y * self.width + x) as usize)
        // y →
        // x ↓
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Result<Rgb, ImageError> {
        let index = self.index(x, y)?;
        Ok(self.pixels[index])
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, rgb: Rgb) -> Result<(), ImageError> {
        let index = self.index(x, y)?;
        self.pixels[index] = rgb;
        Ok(())
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

        assert_eq!(image.get_pixel(0, 0), Ok(Rgb { r: 0, g: 0, b: 0 }));
    }

    #[test]
    fn set_pixel_updates_pixels() {
        let mut image = Image::new(2, 1);
        let red = Rgb { r: 255, g: 0, b: 0 };

        assert_eq!(image.set_pixel(0, 0, red), Ok(()));
        assert_eq!(image.get_pixel(0, 0), Ok(Rgb { r: 255, g: 0, b: 0 }));
    }

    #[test]
    fn get_pixel_returns_none_when_out_of_bound() {
        let image = Image::new(2, 1);

        assert_eq!(
            image.get_pixel(2, 0),
            Err(ImageError::OutOfBounds { x: 2, y: 0 })
        );
        assert_eq!(
            image.get_pixel(0, 1),
            Err(ImageError::OutOfBounds { x: 0, y: 1 })
        );
    }

    #[test]
    fn set_pixel_returns_false_when_out_of_bounds() {
        let mut image = Image::new(2, 1);
        let red = Rgb { r: 255, g: 0, b: 0 };

        assert_eq!(
            image.set_pixel(2, 0, red),
            Err(ImageError::OutOfBounds { x: 2, y: 0 })
        );
        assert_eq!(
            image.set_pixel(0, 1, red),
            Err(ImageError::OutOfBounds { x: 0, y: 1 })
        );
    }
}
