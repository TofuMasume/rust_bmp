const BMP_FILE_HEADER_SIZE: u32 = 14;
const DIB_HEADER_SIZE: u32 = 40;
const PIXEL_ARRAY_OFFSET: u32 = BMP_FILE_HEADER_SIZE + DIB_HEADER_SIZE;

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
    fn index(&self, x: u32, y: u32) -> Result<usize, ImageError> {
        if x >= self.width || y >= self.height {
            return Err(ImageError::OutOfBounds { x, y });
        }
        Ok((y * self.width + x) as usize)
        // y →
        // x ↓
    }

    fn row_padding(width: u32) -> u32 {
        let raw_row_bytes = width * 3;
        (4 - raw_row_bytes % 4) % 4
    }

    fn row_size(width: u32) -> u32 {
        width * 3 + Image::row_padding(width)
    }

    fn pixel_data_size(width: u32, height: u32) -> u32 {
        Image::row_size(width) * height
    }

    fn file_size(width: u32, height: u32) -> u32 {
        PIXEL_ARRAY_OFFSET + Image::pixel_data_size(width, height)
    }

    pub fn new(width: u32, height: u32) -> Self {
        let black = Rgb { r: 0, g: 0, b: 0 };

        Self {
            width,
            height,
            pixels: vec![black; (width * height) as usize],
        }
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

    #[test]
    fn row_padding_aligns_rows_to_four_bytes() {
        assert_eq!(Image::row_padding(1), 1);
        assert_eq!(Image::row_padding(2), 2);
        assert_eq!(Image::row_padding(3), 3);
        assert_eq!(Image::row_padding(4), 0);
    }

    #[test]
    fn row_size_includes_padding() {
        assert_eq!(Image::row_size(1), 4);
        assert_eq!(Image::row_size(2), 8);
        assert_eq!(Image::row_size(3), 12);
        assert_eq!(Image::row_size(4), 12);
    }

    #[test]
    fn pixel_data_size_is_row_size_times_height() {
        assert_eq!(Image::pixel_data_size(1, 1), 4);
        assert_eq!(Image::pixel_data_size(2, 1), 8);
        assert_eq!(Image::pixel_data_size(3, 2), 24);
        assert_eq!(Image::pixel_data_size(4, 2), 24);
    }

    #[test]
    fn file_size_includes_headers_and_pixel_data() {
        assert_eq!(Image::file_size(1, 1), 58);
        assert_eq!(Image::file_size(2, 1), 62);
        assert_eq!(Image::file_size(3, 1), 66);
        assert_eq!(Image::file_size(4, 1), 66);
    }
}
