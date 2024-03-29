extern crate strum;
#[macro_use] extern crate strum_macros;

pub mod config;
pub mod image;
pub mod bytewriter;
pub mod convert;

use structopt::StructOpt;
use image::*;
use convert::FixedWidthFontConverter;
use std::io::LineWriter;

fn main() -> Result<(), ImageLoadingError> {
    let config = config::Config::from_args();

    let line_writer = LineWriter::new(config.output_stream());

    InputPNGImage::new(&config.input_file_path)
        .and_then( |image| -> Result<(), ImageLoadingError> {
            let converter = FixedWidthFontConverter{
                font_metrics: config.font_metrics(),
                output_format: config.format,
                source_code_options: config.source_code_options(),
            };
            converter.convert(image, line_writer)
        })
}
