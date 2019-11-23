extern crate strum;
#[macro_use] extern crate strum_macros;

pub mod config;
pub mod image;
pub mod bytewriter;
pub mod convert;

use structopt::StructOpt;
use image::*;
use convert::FixedWidthFontConverter;

fn main() {
    let config = config::Config::from_args();
    // println!("{:?}", config);
    // println!("{:?}", config.source_code_options());

    let source_code = InputPNGImage::new(config.input_file_path.clone())
        .and_then( |image| -> Result<String, ImageLoadingError> {
            let converter = FixedWidthFontConverter{
                font_metrics: config.font_metrics(),
                output_format: config.format,
                source_code_options: config.source_code_options()
            };
            Ok(converter.convert(image))
        });

    match source_code {
        Ok(code) => println!("{}", code),
        Err(error) => println!("Error processing image file: {}", error)
    }
    
}
