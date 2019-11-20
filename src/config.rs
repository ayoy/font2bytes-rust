use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, EnumString, EnumVariantNames, Debug)]
#[strum(serialize_all = "kebab-case")]
pub enum Generator {
	C,
	Arduino,
	PythonList,
	PythonBytes
}

#[derive(Debug)]
pub enum BitNumbering {
	MSB,
	LSB
}

#[derive(Debug)]
pub struct SourceCodeOptions {
	pub bit_numbering: BitNumbering,
	pub invert_bits: bool
}


#[derive(Debug, StructOpt)]
#[structopt(about = "Converts font bitmap to array of bytes for use in embedded systems.")]
pub struct Config {
	/// Font height in pixels
	#[structopt(short = "h", long = "height")]
	pub font_height: u8,

	/// Font width in pixels
	#[structopt(short = "w", long = "width")]
	pub font_width: u8,

	/// Output source code format
	///
	/// Available output formats: c, arduino, python-list, python-bytes
	#[structopt(short, long, default_value = "c")]
	pub format: Generator,

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
