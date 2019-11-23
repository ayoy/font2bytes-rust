extern crate strum;
#[macro_use] extern crate strum_macros;

pub mod config;
pub mod image;
pub mod bytewriter;
pub mod convert;

use structopt::StructOpt;
use image::*;

fn main() {
    let config = config::Config::from_args();
    let bit_numbering = if config.msb { config::BitNumbering::MSB } else { config::BitNumbering::LSB };
    let source_code_options = config::SourceCodeOptions { 
    	bit_numbering: bit_numbering,
    	invert_bits: config.invert_bits
    };
    println!("{:?}", config);
    println!("{:?}", source_code_options);

    let source_code = InputPNGImage::new(config.input_file_path.clone())
        .and_then( |image| -> Result<String, ImageLoadingError> {
            let converter = convert::FixedWidthFontConverter::new(config.font_metrics(), config.format);
            Ok(converter.convert(image))
        });

    match source_code {
        Ok(code) => println!("{}", code),
        Err(error) => println!("Error processing image file: {}", error)
    }
    
}
