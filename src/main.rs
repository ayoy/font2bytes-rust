extern crate strum;
#[macro_use] extern crate strum_macros;

use structopt::StructOpt;
pub mod config;

fn main() {
    let config = config::Config::from_args();
    let bit_numbering = if config.msb { config::BitNumbering::MSB } else { config::BitNumbering::LSB };
    let source_code_options = config::SourceCodeOptions { 
    	bit_numbering: bit_numbering,
    	invert_bits: config.invert_bits
    };
    println!("{:?}", config);
    println!("{:?}", source_code_options);
    println!("{:?}", config.input_file_path);
}
