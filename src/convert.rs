use crate::config::FontMetrics;
use crate::config::Format;
use crate::image::InputImage;
use chrono::prelude::Local;
use crate::bytewriter::*;
use std::cmp::min;

pub struct FixedWidthFontConverter {
	font_metrics: FontMetrics,
	format: Format
}

impl FixedWidthFontConverter {
	pub fn new(font_metrics: FontMetrics, format: Format) -> FixedWidthFontConverter {
		FixedWidthFontConverter { font_metrics: font_metrics, format: format }
	}

	pub fn convert<T: InputImage>(&self, image: T) -> String {

	    let generator = CCodeGenerator {};
	    let timestamp = Local::now().format("%d/%m/%Y at %H:%M:%S");

	    let mut source_code = String::new();

	    source_code.push_str(&generator.begin(&timestamp.to_string()));
	    source_code.push_str(&generator.begin_array("font"));

	    let mut character_count = 0;
	    for y in 0..image.height()/(self.font_metrics.height as u32) {

		    for x in 0..image.width()/(self.font_metrics.width as u32) {
		    	source_code.push_str(&generator.begin_array_row());

		    	for row in 0..self.font_metrics.height {
		    		let mut remaining_bits: u8 = self.font_metrics.width;

		    		// for width > 8, each line will be represented by more than one byte;
                	// let's track bytes count per line here
                	let mut byte_index: u8 = 0;

                	while remaining_bits > 0 {
                		let mut byte: u8 = 0;
                		let bit_count: u8 = min(remaining_bits, 8);
                		let mut mask: u8 = 0b10000000;

                		for bit in 0..bit_count {
                			let x_coord: u32 = x * (self.font_metrics.width as u32) + (bit as u32) + 8 * (byte_index as u32);
                			let y_coord: u32 = y * (self.font_metrics.height as u32) + (row as u32);
                			if image.is_pixel_set(x_coord, y_coord) {
                                byte |= mask;
                            }
    	                    mask >>= 1;
	                        remaining_bits -= 1;
                		}
	                    source_code.push_str(&generator.byte(byte));
    	                byte_index += 1;
                	}
		    	}
		    	source_code.push_str(&generator.comment(&format!("Character 0x{0:02X} ({0})", character_count)));
		    	source_code.push_str(&generator.line_break());
		    	character_count += 1;
		    }
	    }
	    source_code.push_str(&generator.end_array());
	    source_code.push_str(&generator.end());
	    return source_code;
	}
}