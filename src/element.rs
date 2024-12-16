use alloc::{borrow::Cow, string::ToString, vec::Vec};
use core::fmt::{Debug, Write};

use crate::{Audio, Break, Emphasis, Mark, Meta, SayAs, Serialize, SerializeOptions, Text, Voice, XmlWriter, util};

macro_rules! el {
	(
		$(#[$outer:meta])*
		pub enum $name:ident<'s> {
			$(
				$(#[$innermeta:meta])*
				$variant:ident($inner:ty)
			),*
		}
	) => {
		$(#[$outer])*
		pub enum $name<'s> {
			$(
				$(#[$innermeta])*
				$variant($inner)
			),*
		}

		$(impl<'s> From<$inner> for $name<'s> {
			fn from(val: $inner) -> $name<'s> {
				$name::$variant(val)
			}
		})*

		impl<'s> $crate::Serialize for $name<'s> {
			fn serialize_xml<W: ::core::fmt::Write>(&self, writer: &mut $crate::XmlWriter<W>, options: &$crate::SerializeOptions) -> $crate::Result<()> {
				match self {
					$($name::$variant(inner) => inner.serialize_xml(writer, options),)*
				}
			}
		}
	};
}
pub(crate) use el;

el! {
	/// Represents all SSML elements.
	#[derive(Clone, Debug)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
	#[non_exhaustive]
	pub enum Element<'s> {
		Text(Text<'s>),
		Audio(Audio<'s>),
		Voice(Voice<'s>),
		Meta(Meta<'s>),
		Break(Break),
		Emphasis(Emphasis<'s>),
		Mark(Mark<'s>),
		SayAs(SayAs<'s>),
		FlavorMSTTS(crate::mstts::Element<'s>),
		Custom(CustomElement<'s>)
		// Lang(LangElement),
		// Paragraph(ParagraphElement),
		// Phoneme(PhonemeElement),
		// Prosody(ProsodyElement),
		// Sub(SubElement),
		// Sentence(SentenceElement),
		// Word(WordElement)
	}
}

impl<'s> Element<'s> {
	pub fn to_owned(&self) -> Element<'static> {
		self.clone().into_owned()
	}

	pub fn into_owned(self) -> Element<'static> {
		match self {
			Self::Text(el) => Element::Text(el.into_owned()),
			Self::Audio(el) => Element::Audio(el.into_owned()),
			Self::Voice(el) => Element::Voice(el.into_owned()),
			Self::Meta(el) => Element::Meta(el.into_owned()),
			Self::Break(el) => Element::Break(el),
			Self::Emphasis(el) => Element::Emphasis(el.into_owned()),
			Self::Mark(el) => Element::Mark(el.into_owned()),
			Self::Custom(el) => Element::Custom(el.into_owned()),
			_ => panic!()
		}
	}
}

impl<'s, T: Into<Cow<'s, str>>> From<T> for Element<'s> {
	fn from(value: T) -> Self {
		Element::Text(Text::from(value))
	}
}

pub trait IntoElement<'s> {
	fn into_element(self) -> Element<'s>;
}

impl<'s, T: Into<Element<'s>>> IntoElement<'s> for T {
	fn into_element(self) -> Element<'s> {
		self.into()
	}
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CustomElement<'s> {
	tag: Cow<'s, str>,
	attrs: Vec<(Cow<'s, str>, Cow<'s, str>)>,
	children: Vec<Element<'s>>
}

impl<'s> CustomElement<'s> {
	pub fn new(tag: impl Into<Cow<'s, str>>) -> Self {
		Self {
			tag: tag.into(),
			attrs: Vec::new(),
			children: Vec::new()
		}
	}

	pub fn with_attr(mut self, name: impl Into<Cow<'s, str>>, value: impl Into<Cow<'s, str>>) -> Self {
		self.attrs.push((name.into(), value.into()));
		self
	}

	pub fn with_child(mut self, element: impl Into<Element<'s>>) -> Self {
		self.children.push(element.into());
		self
	}

	pub fn with_children<S: Into<Element<'s>>, I: IntoIterator<Item = S>>(mut self, elements: I) -> Self {
		self.children.extend(elements.into_iter().map(|f| f.into()));
		self
	}

	pub fn to_owned(&self) -> CustomElement<'static> {
		self.clone().into_owned()
	}

	pub fn into_owned(self) -> CustomElement<'static> {
		CustomElement {
			tag: match self.tag {
				Cow::Borrowed(b) => Cow::Owned(b.to_string()),
				Cow::Owned(b) => Cow::Owned(b)
			},
			attrs: self
				.attrs
				.into_iter()
				.map(|(k, v)| {
					(
						match k {
							Cow::Borrowed(b) => Cow::Owned(b.to_string()),
							Cow::Owned(b) => Cow::Owned(b)
						},
						match v {
							Cow::Borrowed(b) => Cow::Owned(b.to_string()),
							Cow::Owned(b) => Cow::Owned(b)
						}
					)
				})
				.collect(),
			children: self.children.into_iter().map(Element::into_owned).collect()
		}
	}
}

impl<'s> Serialize for CustomElement<'s> {
	fn serialize_xml<W: Write>(&self, writer: &mut XmlWriter<W>, options: &SerializeOptions) -> crate::Result<()> {
		writer.element(&self.tag, |writer| {
			for (name, value) in &self.attrs {
				writer.attr(name, value.as_ref())?;
			}
			util::serialize_elements(writer, &self.children, options)
		})
	}
}
