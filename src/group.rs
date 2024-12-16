use alloc::vec::Vec;
use core::{
	fmt::Write,
	ops::{Add, AddAssign}
};

use crate::{Element, Serialize, SerializeOptions, XmlWriter};

#[derive(Clone, Default, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Group<'s> {
	pub(crate) children: Vec<Element<'s>>
}

impl<'s> Group<'s> {
	pub fn new<S: Into<Element<'s>>, I: IntoIterator<Item = S>>(elements: I) -> Self {
		Self {
			children: elements.into_iter().map(|f| f.into()).collect()
		}
	}

	pub fn children(&self) -> &[Element<'s>] {
		&self.children
	}

	pub fn children_mut(&mut self) -> &mut Vec<Element<'s>> {
		&mut self.children
	}

	pub fn push(&mut self, element: impl Into<Element<'s>>) {
		self.children.push(element.into());
	}

	pub fn extend<S: Into<Element<'s>>, I: IntoIterator<Item = S>>(&mut self, elements: I) {
		self.children.extend(elements.into_iter().map(|f| f.into()));
	}

	pub fn to_owned(&self) -> Group<'static> {
		self.clone().into_owned()
	}

	pub fn into_owned(self) -> Group<'static> {
		Group {
			children: self.children.into_iter().map(Element::into_owned).collect()
		}
	}
}

impl<'s> Serialize for Group<'s> {
	fn serialize_xml<W: Write>(&self, writer: &mut XmlWriter<W>, options: &SerializeOptions) -> crate::Result<()> {
		for child in &self.children {
			child.serialize_xml(writer, options)?;
		}
		Ok(())
	}
}

impl<'s, 's2: 's, T: Into<Element<'s2>>> Add<T> for Group<'s> {
	type Output = Group<'s>;

	fn add(mut self, rhs: T) -> Self::Output {
		self.push(rhs.into());
		self
	}
}

impl<'s, 's2: 's, T: Into<Element<'s2>>> AddAssign<T> for Group<'s> {
	fn add_assign(&mut self, rhs: T) {
		self.push(rhs.into());
	}
}

pub fn group<'s, S: Into<Element<'s>>, I: IntoIterator<Item = S>>(elements: I) -> Group<'s> {
	Group::new(elements)
}
