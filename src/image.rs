use std::path::PathBuf;
use std::fs::File;

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
		if x > self.width || y > self.height {
			return false;
		}
		let width_offset = if self.width % 8 == 0 { self.width / 8 } else { self.width / 8 + 1 };
		let offset = y * width_offset + x/8;

		let mask = 1 << x%8;
		return self.image_data[offset as usize] & mask == mask;
	}
}

pub fn read_png_image(file_path: &PathBuf) -> InputPNGImage {
	let decoder = png::Decoder::new(File::open(file_path).unwrap());
	let (info, mut reader) = decoder.read_info().unwrap();

	let read_row = || reader.next_row().unwrap().unwrap().to_vec();

	InputPNGImage { 
		width: info.width,
		height: info.height,
		image_data: read_png_image_data(info.height, read_row)
	}
}

fn read_png_image_data(num_rows: u32, mut read_row: impl FnMut() -> Vec<u8>) -> Vec<u8> {
	let mut data: Vec<u8> = Vec::new();

	let mut line = 0;

	while line < num_rows {
		let row = read_row();

		let (mut index, mut mask, mut current_byte) = (0, 1u8, 0u8);
		while index < row.len() {
			let (r, g, b, a) = (row[index], row[index+1], row[index+2], row[index+3]);
			let is_set = (a == 0xFF) && (r < 0x32 || g < 0x32 || b < 0x32);

			if is_set {
				current_byte |= mask;
			}
			index += 4;

			if mask == 0b10000000 {
				data.push(current_byte);
				current_byte = 0;
				mask = 1;
			} else {
				mask <<= 1;
			}
		}

		line += 1;
	}

	data
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

        let read_row = || {
            [
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
            ].to_vec()
        };
        
        let data = read_png_image_data(1, read_row);

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
