use crate::config::Format;

pub trait ByteWriter {
	fn begin(&self, timestamp: &String) -> String;
	fn begin_array(&self, name: &str) -> String;
	fn begin_array_row(&self) -> String;
	fn byte(&self, byte: u8) -> String;
	fn comment(&self, comment: &String) -> String;
	fn line_break(&self) -> String;
	fn end_array(&self) -> String;
	fn end(&self) -> String;
}

impl ByteWriter for Format {
	fn begin(&self, timestamp: &String) -> String {
		match self {
			Format::C => format!("//\n// Font Data\n// Created: {}\n//\n", timestamp),
			Format::Arduino => format!("//\n// Font Data\n// Created: {}\n//\n\n#include <Arduino.h>\n", timestamp),
			Format::PythonList | Format::PythonBytes => format!("#\n# Font Data\n# Created: {}\n#\n", timestamp)
		}
	}

	fn begin_array(&self, name: &str) -> String {
		match self {
			Format::C => format!("\n\nconst unsigned char {}[] = {{\n", name),
			Format::Arduino => format!("\n\nconst uint8_t {}[] PROGMEM = {{\n", name),
			Format::PythonList => format!("\n\n{} = [\n", name),
			Format::PythonBytes => format!("\n\n{} = b'' \\\n", name)
		}		
	}

	fn begin_array_row(&self) -> String {
		match self {
			Format::PythonBytes => String::from("\t'"),
			_ => String::from("\t")
		}
	}

	fn byte(&self, byte: u8) -> String {
		match self {
			Format::C | Format::Arduino => format!("0x{:02X},", byte),
			Format::PythonList => format!("0x{:02x},", byte),
			Format::PythonBytes => format!("\\x{:02x}", byte)
		}
	}

	fn comment(&self, comment: &String) -> String {
		match self {
			Format::C | Format::Arduino => format!(" // {}", comment),
			Format::PythonList => format!(" # {}", comment),
			Format::PythonBytes => String::from("' \\")
		}
	}

	fn line_break(&self) -> String {
		String::from("\n")
	}

	fn end_array(&self) -> String {
		match self {
			Format::C | Format::Arduino => String::from("};\n"),
			Format::PythonList => String::from("]\n"),
			Format::PythonBytes => String::from("")
		}
	}

	fn end(&self) -> String {
		String::from("\n\n")
	}
}
