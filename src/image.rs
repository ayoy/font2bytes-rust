use std::path::PathBuf;
use std::fs::File;

trait InputImage {
	fn width(&self) -> u32;
	fn height(&self) -> u32;
	fn is_pixel_set(&self, x: u32, y: u32) -> bool;
}

pub struct InputPNGImage {
	pub file_path: PathBuf,
	pub width: u32,
	pub height: u32,
	image_data: Vec<bool>,
}

impl InputImage for InputPNGImage {
	fn width(&self) -> u32 { 
		return self.width; 
	}

	fn height(&self) -> u32 { 
		return self.height; 
	}

	fn is_pixel_set(&self, x: u32, y: u32) -> bool {
		return self.image_data[(x * self.width + y) as usize]; 
	}
}

pub fn read_png_image(file_path: &PathBuf) -> InputPNGImage {
	let decoder = png::Decoder::new(File::open(file_path).unwrap());
	let (info, mut reader) = decoder.read_info().unwrap();
	let mut data: Vec<bool> = Vec::with_capacity(reader.output_buffer_size()/4);

	let mut line = 0;

	while line < info.height {
		let row = reader.next_row().unwrap().unwrap();

		let mut byte = 0;
		while byte < row.len() {
			let (r, g, b, a) = (row[byte], row[byte+1], row[byte+2], row[byte+3]);
			data.push((a == 0xFF) && (r < 0x32 || g < 0x32 || b < 0x32));
			byte += 4;
		}

		
		println!("reading line {} {} {}", line, row.len(), data.len());
		line += 1;
	}


	return InputPNGImage { 
		file_path: file_path.clone(), 
		width: info.width,
		height: info.height,
		image_data: data 
	}
}