use std::path::PathBuf;
use std::fs::File;
use std::io;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ImageLoadingError {
    IOError(io::Error),
    DecodingError(png::DecodingError)
}

impl Error for ImageLoadingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ImageLoadingError::IOError(e) => Some(e),
            ImageLoadingError::DecodingError(e) => Some(e)
        }
    }
}

impl fmt::Display for ImageLoadingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.source().unwrap().to_string())
    }
}

impl From<io::Error> for ImageLoadingError {
    fn from(e: io::Error) -> ImageLoadingError {
        ImageLoadingError::IOError(e)
    }
}

impl From<png::DecodingError> for ImageLoadingError {
    fn from(e: png::DecodingError) -> ImageLoadingError {
        ImageLoadingError::DecodingError(e)
    }    
}

pub trait InputImage {
	fn width(&self) -> u32;
	fn height(&self) -> u32;
	fn is_pixel_set(&self, x: u32, y: u32) -> bool;
}

pub struct InputPNGImage {
	pub width: u32,
	pub height: u32,
	image_data: Vec<u8>,
}

impl InputImage for InputPNGImage {
	fn width(&self) -> u32 { 
		return self.width; 
	}

	fn height(&self) -> u32 { 
		return self.height; 
	}

	fn is_pixel_set(&self, x: u32, y: u32) -> bool {
		if x >= self.width || y >= self.height {
			return false;
		}
		let width_offset = if self.width % 8 == 0 { self.width / 8 } else { self.width / 8 + 1 };
		let offset = y * width_offset + x/8;

		let mask = 1 << x%8;
        if self.image_data.len() <= offset as usize {
            println!("x: {}, y: {}, offset: {}, len: {}, width: {}, height: {}", x, y, offset, self.image_data.len(), self.width, self.height);
        }
		return self.image_data[offset as usize] & mask == mask;
	}
}

impl InputPNGImage {
    pub fn new<'a>(file_path: PathBuf) -> Result<InputPNGImage, ImageLoadingError> {
        let decoder = png::Decoder::new(File::open(file_path)?);
        let (info, mut reader) = decoder.read_info()?;

        let read_row = || -> Result<Option<Vec<u8>>, png::DecodingError> {
            reader.next_row().map( |optional_data| optional_data.map(|r| r.to_vec()) )
        };

        let image_data = read_png_image_data(info.color_type, read_row)?;

        Ok(InputPNGImage {
            width: info.width,
            height: info.height,
            image_data: image_data
        })
    }
}

fn read_png_image_data(color_type: png::ColorType, 
    mut read_next_row: impl FnMut() -> Result<Option<Vec<u8>>, png::DecodingError>) 
    -> Result<Vec<u8>, png::DecodingError>
{
    let mut data: Vec<u8> = Vec::new();

    let mut error: Option<png::DecodingError> = None;

    loop {
        let result = read_next_row();
        if result.is_err() {
            error = result.err();
            // break with a PNG decoding error
            break;
        }

        let optional_data = result.ok().unwrap();

        if optional_data == None {
            // break due to EOF
            break;
        }

        let row = optional_data.unwrap();

        let mut image_row = ImageRowIterator::new(&row, color_type).unwrap();

        let (mut mask, mut current_byte) = (1u8, 0u8);
        while let Some(pixel) = image_row.next() {

            if pixel.is_set() {
                current_byte |= mask;
            }

            if mask == 0b10000000 {
                data.push(current_byte);
                current_byte = 0;
                mask = 1;
            } else {
                mask <<= 1;
            }
        }
    }

    match error {
        Some(err) => Err(err),
        None => Ok(data)
    }
}

#[derive(Debug)]
enum Pixel {
    RGBA(u8, u8, u8, u8),
    Grayscale(u8, u8)
}

impl Pixel {
    const IS_SET_THRESHOLD: u8 = 0x32;

    fn from_vec(vec: &[u8]) -> Option<Pixel> {
        match vec.len() {
            1 => Some(Pixel::Grayscale(vec[0], 0xFF)),
            2 => Some(Pixel::Grayscale(vec[0], vec[1])),
            3 => Some(Pixel::RGBA(vec[0], vec[1], vec[2], 0xFF)),
            4 => Some(Pixel::RGBA(vec[0], vec[1], vec[2], vec[3])),
            _ => None
        }
    }

    fn is_set(&self) -> bool {
        match self {
            Pixel::Grayscale(gray, alpha) => *alpha == 0xFF && *gray < Pixel::IS_SET_THRESHOLD,
            Pixel::RGBA(r, g, b, alpha) => *alpha == 0xFF && 
                (
                    *r < Pixel::IS_SET_THRESHOLD || 
                    *g < Pixel::IS_SET_THRESHOLD || 
                    *b < Pixel::IS_SET_THRESHOLD
                )
        }
    }
}


#[derive(Debug)]
struct ImageRowIterator<'a> {
    row: &'a [u8], 
    step: usize,
    index: usize
}

impl<'a> ImageRowIterator<'a> {
    fn new(row: &'a [u8], color_type: png::ColorType) -> Option<ImageRowIterator> {
        let step = match color_type {
            png::ColorType::Grayscale => 1,
            png::ColorType::GrayscaleAlpha => 2,
            png::ColorType::RGB => 3,
            png::ColorType::RGBA => 4,
            _ => 0
        };

        if step == 0 {
            return None;
        }

        Some(ImageRowIterator {row: row, step: step, index: 0 })
    }
}

impl<'a> Iterator for ImageRowIterator<'a> {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        let end_index = std::cmp::min(self.index + self.step - 1, self.row.len() - 1);

        let values = &self.row[self.index..=end_index];
        self.index = end_index+1;

        if values.len() == self.step {
            Self::Item::from_vec(values)
        } else {
            None
        }        
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn is_pixel_set() {
    	use crate::image::InputImage;
    	use crate::image::InputPNGImage;
    	let image = InputPNGImage {
    		width: 12,
    		height: 4,
    		image_data: vec![
                0b01101101, 0b00000110,
                0b00111001, 0b00001111,
                0b01111101, 0b00001010,
                0b10101010, 0b00000101
            ] 
        };
        assert_eq!(image.width, 12);
        assert_eq!(image.height, 4);
        assert_eq!(image.is_pixel_set(0, 0), true);
        assert_eq!(image.is_pixel_set(1, 1), false);
        assert_eq!(image.is_pixel_set(2, 2), true);
        assert_eq!(image.is_pixel_set(1, 3), true);
        assert_eq!(image.is_pixel_set(5, 1), true);
        assert_eq!(image.is_pixel_set(6, 1), false);
        assert_eq!(image.is_pixel_set(7, 1), false);
        assert_eq!(image.is_pixel_set(8, 1), true);
        assert_eq!(image.is_pixel_set(10, 3), true);
        assert_eq!(image.is_pixel_set(100, 300), false);
    }

    #[test]
    fn read_png_image() {
        use crate::image::read_png_image_data;

        let mut num = 1;

        let read_row = || {
            num -= 1;
            if num < 0 {
                return Ok(None);
            }
            Ok(Some([
                191, 191, 191, 191, 
                191, 191, 191, 191,
                0, 0, 0, 255,
                0, 0, 0, 255, 
                191, 191, 191, 191, 
                191, 191, 191, 191, 
                0, 0, 0, 255, 
                0, 0, 0, 255,

                191, 191, 191, 191,
                0, 0, 0, 255,
                191, 191, 191, 191, 
                191, 191, 191, 191, 
                191, 191, 191, 191, 
                0, 0, 0, 255, 
                191, 191, 191, 191, 
                0, 0, 0, 255,

                191, 191, 191, 191, 
                0, 0, 0, 255,
                0, 0, 0, 255, 
                0, 0, 0, 255, 
                0, 0, 0, 255, 
                191, 191, 191, 191, 
                191, 191, 191, 191, 
                0, 0, 0, 255,

                0, 0, 0, 255,
                191, 191, 191, 191,
                0, 0, 0, 255,
                191, 191, 191, 191,
                191, 191, 191, 191,
                191, 191, 191, 191,
                191, 191, 191, 191,
                191, 191, 191, 191,

                191, 191, 191, 191,
                191, 191, 191, 191,
                191, 191, 191, 191,
                191, 191, 191, 191,
                191, 191, 191, 191,
                191, 191, 191, 191,
                191, 191, 191, 191,
                191, 191, 191, 191,

                0, 0, 0, 255,
                0, 0, 0, 255, 
                0, 0, 0, 255, 
                0, 0, 0, 255, 
                0, 0, 0, 255,
                0, 0, 0, 255, 
                0, 0, 0, 255, 
                0, 0, 0, 255, 

                191, 191, 191, 191,
                191, 191, 191, 191,
                191, 191, 191, 191,
                191, 191, 191, 191,
                0, 0, 0, 255,
                0, 0, 0, 255, 
                0, 0, 0, 255, 
                0, 0, 0, 255,

                191, 191, 191, 191,
                191, 191, 191, 191,
                191, 191, 191, 191,
                191, 191, 191, 191,
                0, 0, 0, 255,
                0, 0, 0, 255, 
                0, 0, 0, 255, 
                0, 0, 0, 255,

                191, 191, 191, 191,
                191, 191, 191, 191,
                191, 191, 191, 191,
                191, 191, 191, 191,
                0, 0, 0, 255,
                0, 0, 0, 255, 
                191, 191, 191, 191,
                0, 0, 0, 255,
            ].to_vec()))
        };
        
        let data = read_png_image_data(png::ColorType::RGBA, read_row).unwrap();

        assert_eq!(data, [
                0b11001100, 
                0b10100010,
                0b10011110,
                0b00000101, 
                0b00000000, 
                0b11111111, 
                0b11110000,
                0b11110000,
                0b10110000
            ].to_vec());
    }
}
