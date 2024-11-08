use alloc::{borrow::Cow, string::ToString};
use core::{fmt::Write, ops::Deref};

use crate::{Serialize, SerializeOptions, XmlWriter};

/// A non-marked-up string of text for use as a spoken element.
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Text<'s>(Cow<'s, str>);

impl<'s> Text<'s> {
	pub fn to_owned(&self) -> Text<'static> {
		self.clone().into_owned()
	}

	pub fn into_owned(self) -> Text<'static> {
		Text(match self.0 {
			Cow::Borrowed(b) => Cow::Owned(b.to_string()),
			Cow::Owned(b) => Cow::Owned(b)
		})
	}
}

impl Deref for Text<'_> {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		&*self.0
	}
}

impl<'s, T: Into<Cow<'s, str>>> From<T> for Text<'s> {
	fn from(value: T) -> Self {
		Self(value.into())
	}
}

impl Serialize for Text<'_> {
	fn serialize_xml<W: Write>(&self, writer: &mut XmlWriter<W>, _: &SerializeOptions) -> crate::Result<()> {
		writer.text(&self.0)
	}
}

/// Creates a spoken [`Text`] element from a string.
pub fn text<'s>(s: impl Into<Cow<'s, str>>) -> Text<'s> {
	Text(s.into())
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
