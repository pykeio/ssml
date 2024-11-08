use alloc::{borrow::Cow, string::ToString};
use core::fmt::Write;

use crate::{Serialize, SerializeOptions, XmlWriter};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mark<'s> {
	name: Cow<'s, str>
}

impl<'s> Mark<'s> {
	pub fn new(name: impl Into<Cow<'s, str>>) -> Self {
		Self { name: name.into() }
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn to_owned(&self) -> Mark<'static> {
		self.clone().into_owned()
	}

	pub fn into_owned(self) -> Mark<'static> {
		Mark {
			name: match self.name {
				Cow::Borrowed(b) => Cow::Owned(b.to_string()),
				Cow::Owned(b) => Cow::Owned(b)
			}
		}
	}
}

impl<'s> Serialize for Mark<'s> {
	fn serialize_xml<W: Write>(&self, writer: &mut XmlWriter<W>, _: &SerializeOptions) -> crate::Result<()> {
		writer.element("mark", |writer| writer.attr("name", &*self.name))
	}
}

pub fn mark<'s>(name: impl Into<Cow<'s, str>>) -> Mark<'s> {
	Mark::new(name)
}
