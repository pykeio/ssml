use std::{error::Error, io::Write};

use crate::{Flavor, Serialize};

/// A non-marked-up string of text for use as a spoken element.
#[derive(Clone)]
pub struct Text(pub String);

impl<T: ToString> From<T> for Text {
	fn from(value: T) -> Self {
		Self(value.to_string())
	}
}

impl Serialize for Text {
	fn serialize<W: Write>(&self, writer: &mut W, _: Flavor) -> Result<(), Box<dyn Error>> {
		writer.write_all(self.0.as_bytes())?;
		writer.write_all(b" ")?;
		Ok(())
	}
}

/// Creates a spoken [`Text`] element from a string.
pub fn text(s: impl ToString) -> Text {
	Text(s.to_string())
}
