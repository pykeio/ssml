use alloc::string::{String, ToString};
use core::fmt::Write;

use crate::{Serialize, SerializeOptions, XmlWriter};

/// A non-marked-up string of text for use as a spoken element.
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Text(pub String);

impl<T: ToString> From<T> for Text {
	fn from(value: T) -> Self {
		Self(value.to_string())
	}
}

impl Serialize for Text {
	fn serialize_xml<W: Write>(&self, writer: &mut XmlWriter<W>, _: &SerializeOptions) -> crate::Result<()> {
		writer.text(&self.0)
	}
}

/// Creates a spoken [`Text`] element from a string.
pub fn text(s: impl ToString) -> Text {
	Text(s.to_string())
}

#[cfg(test)]
mod tests {
	use super::text;
	use crate::{Serialize, SerializeOptions};

	#[test]
	fn text_escapes() -> crate::Result<()> {
		assert_eq!(text("One & two").serialize_to_string(&SerializeOptions::default())?, "One &amp; two");
		Ok(())
	}
}
