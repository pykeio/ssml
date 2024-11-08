use alloc::vec::Vec;
use core::fmt::Write;

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
pub struct Emphasis<'s> {
	level: EmphasisLevel,
	pub(crate) children: Vec<Element<'s>>
}

impl<'s> Emphasis<'s> {
	pub fn new<S: Into<Element<'s>>, I: IntoIterator<Item = S>>(level: EmphasisLevel, elements: I) -> Self {
		Self {
			level,
			children: elements.into_iter().map(|f| f.into()).collect()
		}
	}

	pub fn push(&mut self, element: impl Into<Element<'s>>) {
		self.children.push(element.into());
	}

	pub fn extend<S: Into<Element<'s>>, I: IntoIterator<Item = S>>(&mut self, elements: I) {
		self.children.extend(elements.into_iter().map(|f| f.into()));
	}

	pub fn level(&self) -> &EmphasisLevel {
		&self.level
	}

	pub fn children(&self) -> &[Element<'s>] {
		&self.children
	}

	pub fn children_mut(&mut self) -> &mut [Element<'s>] {
		&mut self.children
	}

	pub fn to_owned(&self) -> Emphasis<'static> {
		self.clone().into_owned()
	}

	pub fn into_owned(self) -> Emphasis<'static> {
		Emphasis {
			level: self.level,
			children: self.children.into_iter().map(Element::into_owned).collect()
		}
	}
}

impl<'s> Serialize for Emphasis<'s> {
	fn serialize_xml<W: Write>(&self, writer: &mut XmlWriter<W>, _: &SerializeOptions) -> crate::Result<()> {
		writer.element("emphasis", |writer| {
			writer.attr("level", match self.level {
				EmphasisLevel::Reduced => "reduced",
				EmphasisLevel::None => "none",
				EmphasisLevel::Moderate => "moderate",
				EmphasisLevel::Strong => "strong"
			})
		})
	}
}

pub fn emphasis<'s, S: Into<Element<'s>>, I: IntoIterator<Item = S>>(level: EmphasisLevel, elements: I) -> Emphasis<'s> {
	Emphasis::new(level, elements)
}
