use crate::config::FontMetrics;
use crate::config::{Format, SourceCodeOptions, BitNumbering};
use crate::image::{InputImage, ImageLoadingError};
use chrono::prelude::Local;
use crate::bytewriter::*;
use std::cmp::min;
use std::io::LineWriter;
use std::io::Write;

pub struct FixedWidthFontConverter {
    pub font_metrics: FontMetrics,
    pub output_format: Format,
    pub source_code_options: SourceCodeOptions,
}

impl FixedWidthFontConverter {
    pub fn convert<T: InputImage>(&self, image: T, mut buffer: LineWriter<Box<dyn Write>>) -> Result<(), ImageLoadingError> {

        let timestamp = Local::now().format("%d/%m/%Y at %H:%M:%S");

        buffer.write_all(&self.output_format.begin(&timestamp.to_string()).as_bytes())?;
        buffer.write_all(&self.output_format.begin_array("font").as_bytes())?;

        let mut character_count = 0;
        for y in 0..image.height()/(self.font_metrics.height as u32) {

            for x in 0..image.width()/(self.font_metrics.width as u32) {
                buffer.write_all(&self.output_format.begin_array_row().as_bytes())?;

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
                        buffer.write_all(&self.output_format.byte(self.format_byte(byte)).as_bytes())?;
                        byte_index += 1;
                    }
                }
                buffer.write_all(&self.output_format.comment(&format!("Character 0x{0:02X} ({0})", character_count)).as_bytes())?;
                buffer.write_all(&self.output_format.line_break().as_bytes())?;
                character_count += 1;
            }
        }
        buffer.write_all(&self.output_format.end_array().as_bytes())?;
        buffer.write_all(&self.output_format.end().as_bytes())?;

        Ok(())
    }

    fn format_byte(&self, byte: u8) -> u8 {
        let mut b = byte;
        if self.source_code_options.bit_numbering == BitNumbering::MSB {
            b = ((b & 0b11110000) >> 4) | ((b & 0b00001111) << 4);
            b = ((b & 0b11001100) >> 2) | ((b & 0b00110011) << 2);
            b = ((b & 0b10101010) >> 1) | ((b & 0b01010101) << 1);
        }
        if self.source_code_options.invert_bits {
            b = !b;
        }
        b
    }
}