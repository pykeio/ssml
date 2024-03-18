use crate::{Serialize, SerializeOptions, XmlWriter};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mark {
	name: String
}

impl Mark {
	pub fn new(name: impl ToString) -> Self {
		Self { name: name.to_string() }
	}

	pub fn name(&self) -> &str {
		&self.name
	}
}

impl Serialize for Mark {
	fn serialize_xml(&self, writer: &mut XmlWriter<'_>, _: &SerializeOptions) -> crate::Result<()> {
		writer.element("mark", |writer| writer.attr("name", &self.name))
	}
}

pub fn mark(name: impl ToString) -> Mark {
	Mark::new(name)
}
