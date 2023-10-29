use std::io::Write;

use crate::{util, Flavor, Serialize};

/// A non-marked-up string of text for use as a spoken element.
#[derive(Default, Debug, Clone)]
pub struct Text(pub String);

impl<T: ToString> From<T> for Text {
	fn from(value: T) -> Self {
		Self(value.to_string())
	}
}

impl Serialize for Text {
	fn serialize<W: Write>(&self, writer: &mut W, _: Flavor) -> anyhow::Result<()> {
		writer.write_all(util::escape(&self.0).as_bytes())?;
		Ok(())
	}
}

/// Creates a spoken [`Text`] element from a string.
pub fn text(s: impl ToString) -> Text {
	Text(s.to_string())
}

#[cfg(test)]
mod tests {
	use super::text;
	use crate::{Flavor, Serialize};

	#[test]
	fn text_escapes() -> anyhow::Result<()> {
		assert_eq!(text("One & two").serialize_to_string(Flavor::Generic)?, "One &amp; two");
		Ok(())
	}
}
