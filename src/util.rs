//! Utilities for serializing XML.

use crate::{Element, Serialize, SerializeOptions, XmlWriter};

/// Serialize a slice of elements, inserting spaces between adjacent text elements where needed.
pub fn serialize_elements(writer: &mut XmlWriter<'_>, elements: impl AsRef<[Element]>, options: &SerializeOptions) -> crate::Result<()> {
	let elements = elements.as_ref();
	for i in 0..elements.len() {
		let el = &elements[i];
		el.serialize_xml(writer, options)?;

		if !writer.pretty && matches!(el, Element::Text(_)) {
			if let Some(x) = elements.get(i + 1) {
				if matches!(x, Element::Text(_)) {
					writer.write.write_all(b" ")?;
				}
			}
		}
	}
	Ok(())
}
