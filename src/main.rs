extern crate strum;
#[macro_use] extern crate strum_macros;

pub mod config;
pub mod image;

use structopt::StructOpt;
use image::InputPNGImage;
use image::InputImage;

fn main() {
    let config = config::Config::from_args();
    let bit_numbering = if config.msb { config::BitNumbering::MSB } else { config::BitNumbering::LSB };
    let source_code_options = config::SourceCodeOptions { 
    	bit_numbering: bit_numbering,
    	invert_bits: config.invert_bits
    };
    println!("{:?}", config);
    println!("{:?}", source_code_options);
    let image: InputPNGImage = image::read_png_image(&config.input_file_path);
    println!("Image h:{} w:{}", image.height(), image.width());
}
