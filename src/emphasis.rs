use crate::{Element, Serialize, SerializeOptions, XmlWriter};

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EmphasisLevel {
	Reduced,
	None,
	#[default]
	Moderate,
	Strong
}

#[derive(Clone, Default, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Emphasis {
	level: EmphasisLevel,
	pub(crate) children: Vec<Element>
}

impl Emphasis {
	pub fn new<S: Into<Element>, I: IntoIterator<Item = S>>(level: EmphasisLevel, elements: I) -> Self {
		Self {
			level,
			children: elements.into_iter().map(|f| f.into()).collect()
		}
	}

	pub fn push(&mut self, element: impl Into<Element>) {
		self.children.push(element.into());
	}

	pub fn extend<S: Into<Element>, I: IntoIterator<Item = S>>(&mut self, elements: I) {
		self.children.extend(elements.into_iter().map(|f| f.into()));
	}

	pub fn level(&self) -> &EmphasisLevel {
		&self.level
	}

	pub fn children(&self) -> &[Element] {
		&self.children
	}

	pub fn children_mut(&mut self) -> &mut [Element] {
		&mut self.children
	}
}

impl Serialize for Emphasis {
	fn serialize_xml(&self, writer: &mut XmlWriter<'_>, _: &SerializeOptions) -> crate::Result<()> {
		writer.element("emphasis", |writer| {
			writer.attr(
				"level",
				match self.level {
					EmphasisLevel::Reduced => "reduced",
					EmphasisLevel::None => "none",
					EmphasisLevel::Moderate => "moderate",
					EmphasisLevel::Strong => "strong"
				}
			)
		})
	}
}

pub fn emphasis<S: Into<Element>, I: IntoIterator<Item = S>>(level: EmphasisLevel, elements: I) -> Emphasis {
	Emphasis::new(level, elements)
}
