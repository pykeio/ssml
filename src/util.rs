//! Utilities for serializing XML.

use std::io::{self, Write};

use crate::{Element, Flavor, Serialize};

pub fn escape<S: AsRef<str>>(str: S) -> String {
	let str = str.as_ref();
	str.replace('"', "&quot;")
		.replace('\'', "&apos;")
		.replace('<', "&lt;")
		.replace('>', "&gt;")
		.replace('&', "&amp;")
}

pub fn write_attr<W: Write>(writer: &mut W, key: impl AsRef<str>, val: impl AsRef<str>) -> io::Result<()> {
	write!(writer, " {}=\"{}\"", key.as_ref(), escape(val))?;
	Ok(())
}

/// Serialize a slice of elements, inserting spaces between adjacent text elements where needed.
pub fn serialize_elements<W: Write>(writer: &mut W, elements: impl AsRef<[Element]>, flavor: Flavor) -> anyhow::Result<()> {
	let elements = elements.as_ref();
	for i in 0..elements.len() {
		let el = &elements[i];
		el.serialize(writer, flavor)?;

		if matches!(el, Element::Text(_)) {
			if let Some(x) = elements.get(i + 1) {
				if matches!(x, Element::Text(_)) {
					writer.write_all(b" ")?;
				}
			}
		}
	}
	Ok(())
}
