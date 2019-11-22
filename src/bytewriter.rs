
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

pub struct CCodeGenerator {}
pub struct ArduinoCodeGenerator {}

impl ByteWriter for CCodeGenerator {
	fn begin(&self, timestamp: &String) -> String {
		format!("//\n// Font Data\n// Created: {}\n//\n", timestamp)
	}

	fn begin_array(&self, name: &str) -> String {
		format!("\n\nconst unsigned char {}[] = {{\n", name)
	}

	fn begin_array_row(&self) -> String {
		String::from("\t")
	}

	fn byte(&self, byte: u8) -> String {
		format!("0x{:02X},", byte)
	}

	fn comment(&self, comment: &String) -> String {
		format!(" // {}", comment)
	}

	fn line_break(&self) -> String {
		String::from("\n")
	}

	fn end_array(&self) -> String {
		String::from("};\n")
	}

	fn end(&self) -> String {
		String::from("\n\n")
	}
}

impl ByteWriter for ArduinoCodeGenerator {
	fn begin(&self, timestamp: &String) -> String {
		format!("//\n// Font Data\n// Created: {}\n//\n\n#include <Arduino.h>\n", timestamp)
	}

	fn begin_array(&self, name: &str) -> String {
		format!("\n\nconst uint8_t {}[] PROGMEM = {{\n", name)
	}

	fn begin_array_row(&self) -> String {
		String::from("\t")
	}

	fn byte(&self, byte: u8) -> String {
		format!("0x{:02X},", byte)
	}

	fn comment(&self, comment: &String) -> String {
		format!(" // {}", comment)
	}

	fn line_break(&self) -> String {
		String::from("\n")
	}

	fn end_array(&self) -> String {
		String::from("};\n")
	}

	fn end(&self) -> String {
		String::from("\n\n")
	}
}