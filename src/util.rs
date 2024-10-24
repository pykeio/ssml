//! Utilities for serializing XML.

use alloc::string::String;
use core::fmt::{self, Write};

use crate::{Element, Serialize, SerializeOptions, XmlWriter};

/// Serialize a slice of elements, inserting spaces between adjacent text elements where needed.
pub fn serialize_elements<W: Write>(writer: &mut XmlWriter<W>, elements: impl AsRef<[Element]>, options: &SerializeOptions) -> crate::Result<()> {
	let elements = elements.as_ref();
	for i in 0..elements.len() {
		let el = &elements[i];
		el.serialize_xml(writer, options)?;

		if !writer.pretty && matches!(el, Element::Text(_)) {
			if let Some(x) = elements.get(i + 1) {
				if matches!(x, Element::Text(_)) {
					writer.write.write_char(' ')?;
				}
			}
		}
	}
	Ok(())
}

/// Escape the given text for use in XML.
pub fn escape<W: Write>(writer: &mut W, text: impl AsRef<str>) -> fmt::Result {
	let text = text.as_ref();
	for char in text.chars() {
		match char {
			'"' => writer.write_str("&quot;")?,
			'\'' => writer.write_str("&apos;")?,
			'<' => writer.write_str("&lt;")?,
			'>' => writer.write_str("&gt;")?,
			'&' => writer.write_str("&amp;")?,
			_ => writer.write_char(char)?
		}
	}
	Ok(())
}

pub fn escape_to_string(text: impl AsRef<str>) -> Result<String, fmt::Error> {
	let text = text.as_ref();
	let mut out = String::with_capacity(text.len());
	escape(&mut out, text)?;
	Ok(out)
}
