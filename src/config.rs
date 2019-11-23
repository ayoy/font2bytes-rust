use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, EnumString, EnumVariantNames, Debug, Clone, Copy)]
#[strum(serialize_all = "kebab-case")]
pub enum Format {
	C,
	Arduino,
	PythonList,
	PythonBytes
}

#[derive(PartialEq, Debug)]
pub enum BitNumbering {
	MSB,
	LSB
}

#[derive(Debug)]
pub struct SourceCodeOptions {
	pub bit_numbering: BitNumbering,
	pub invert_bits: bool
}

pub struct FontMetrics {
	pub height: u8,
	pub width: u8
}


#[derive(Debug, StructOpt)]
#[structopt(about = "Converts font bitmap to array of bytes for use in embedded systems.")]
pub struct Config {
	/// Font height in pixels
	#[structopt(short = "h", long = "height")]
	font_height: u8,

	/// Font width in pixels
	#[structopt(short = "w", long = "width")]
	font_width: u8,

	/// Output source code format
	///
	/// Available output formats: c, arduino, python-list, python-bytes
	#[structopt(short, long, default_value = "c")]
	pub format: Format,

	/// Path to the output file (stdout if not present)
	#[structopt(short = "o", long = "output")]
	pub output_file_path: Option<PathBuf>,

	/// Path to the input image file
	#[structopt(name = "path-to-image")]
	pub input_file_path: PathBuf,

	/// Store bytes in MSB mode (default is LSB)
	#[structopt(short, long)]
	pub msb: bool,

	/// Invert bits in output
	#[structopt(short, long)]
	pub invert_bits: bool
}

impl Config {
	pub fn font_metrics(&self) -> FontMetrics {
		FontMetrics { height: self.font_height, width: self.font_width }
	}

	pub fn source_code_options(&self) -> SourceCodeOptions {
		SourceCodeOptions { 
    		bit_numbering: self.bit_numbering(),
    		invert_bits: self.invert_bits
    	}
	}

	fn bit_numbering(&self) -> BitNumbering {
		return if self.msb { BitNumbering::MSB } else { BitNumbering::LSB };
	}
}
