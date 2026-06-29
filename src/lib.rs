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

    fn file_header(width: u32, height: u32) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(b"BM"); // file type
        bytes.extend_from_slice(&Image::file_size(width, height).to_le_bytes()); // file size
        bytes.extend_from_slice(&0u16.to_le_bytes()); // reserve
        bytes.extend_from_slice(&0u16.to_le_bytes()); // reserve
        bytes.extend_from_slice(&PIXEL_ARRAY_OFFSET.to_le_bytes()); // image data offset

        bytes
    }

    fn dib_header(width: u32, height: u32) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&DIB_HEADER_SIZE.to_le_bytes());
        bytes.extend_from_slice(&(width as i32).to_le_bytes());
        bytes.extend_from_slice(&(height as i32).to_le_bytes());
        bytes.extend_from_slice(&1u16.to_le_bytes());
        bytes.extend_from_slice(&24u16.to_le_bytes());
        bytes.extend_from_slice(&0u32.to_le_bytes());
        bytes.extend_from_slice(&Image::pixel_data_size(width, height).to_le_bytes());
        bytes.extend_from_slice(&0i32.to_le_bytes());
        bytes.extend_from_slice(&0i32.to_le_bytes());
        bytes.extend_from_slice(&0u32.to_le_bytes());
        bytes.extend_from_slice(&0u32.to_le_bytes());

        bytes
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&Image::file_header(self.width, self.height));
        bytes.extend_from_slice(&Image::dib_header(self.width, self.height));
        bytes.resize(Image::file_size(self.width, self.height) as usize, 0);

        bytes
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

    #[test]
    fn file_header_starts_with_bm() {
        let header = Image::file_header(1, 1);

        assert_eq!(&header[0..2], b"BM");
    }

    #[test]
    fn file_header_has_expected_size() {
        let header = Image::file_header(1, 1);

        assert_eq!(header.len(), BMP_FILE_HEADER_SIZE as usize);
    }

    #[test]
    fn file_header_writes_file_size_and_pixel_offset() {
        let header = Image::file_header(1, 1);

        assert_eq!(&header[2..6], &58u32.to_le_bytes());
        assert_eq!(&header[10..14], &PIXEL_ARRAY_OFFSET.to_le_bytes());
    }

    #[test]
    fn dib_header_has_expected_size() {
        let header = Image::dib_header(1, 1);

        assert_eq!(header.len(), DIB_HEADER_SIZE as usize);
    }

    #[test]
    fn dib_header_writes_size_width_and_height() {
        let header = Image::dib_header(2, 3);

        assert_eq!(&header[0..4], &DIB_HEADER_SIZE.to_le_bytes());
        assert_eq!(&header[4..8], &2i32.to_le_bytes());
        assert_eq!(&header[8..12], &3i32.to_le_bytes());
    }

    #[test]
    fn dib_header_writes_planes_and_bit_count() {
        let header = Image::dib_header(1, 1);

        assert_eq!(&header[12..14], &1u16.to_le_bytes());
        assert_eq!(&header[14..16], &24u16.to_le_bytes());
    }

    #[test]
    fn dib_header_writes_pixel_data_size() {
        let header = Image::dib_header(3, 2);

        assert_eq!(&header[20..24], &24u32.to_le_bytes());
    }

    #[test]
    fn to_bytes_has_expected_file_size() {
        let image = Image::new(1, 1);
        let bytes = image.to_bytes();

        assert_eq!(bytes.len(), 58);
    }

    #[test]
    fn to_bytes_writes_headers_before_pixel_data() {
        let image = Image::new(1, 1);
        let bytes = image.to_bytes();

        assert_eq!(&bytes[0..2], b"BM");
        assert_eq!(&bytes[10..14], &PIXEL_ARRAY_OFFSET.to_le_bytes());
        assert_eq!(&bytes[14..18], &DIB_HEADER_SIZE.to_le_bytes());
    }
}
