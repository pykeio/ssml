use alloc::{borrow::Cow, string::ToString};
use core::{
	fmt::Write,
	ops::{Add, AddAssign}
};

use crate::{Element, Serialize, SerializeOptions, XmlWriter, util};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LangFailure {
	ChangeVoice,
	IgnoreText,
	IgnoreLang,
	ProcessorChoice
}

impl LangFailure {
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::ChangeVoice => "changevoice",
			Self::IgnoreText => "ignoretext",
			Self::IgnoreLang => "ignorelang",
			Self::ProcessorChoice => "processorchoice"
		}
	}
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Lang<'s> {
	language: Cow<'s, str>,
	failure_behavior: Option<LangFailure>,
	pub(crate) children: Vec<Element<'s>>
}

impl<'s> Lang<'s> {
	pub fn new<S: Into<Element<'s>>, I: IntoIterator<Item = S>>(language: impl Into<Cow<'s, str>>, elements: I) -> Self {
		Self {
			language: language.into(),
			failure_behavior: None,
			children: elements.into_iter().map(|f| f.into()).collect()
		}
	}

	pub fn with_failure_behavior(mut self, behavior: LangFailure) -> Self {
		self.failure_behavior = Some(behavior);
		self
	}

	pub fn failure_behavior(&self) -> Option<&LangFailure> {
		self.failure_behavior.as_ref()
	}

	pub fn set_failure_behavior(&mut self, behavior: LangFailure) {
		self.failure_behavior = Some(behavior);
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

	pub fn to_owned(&self) -> Lang<'static> {
		self.clone().into_owned()
	}

	pub fn into_owned(self) -> Lang<'static> {
		Lang {
			language: match self.language {
				Cow::Borrowed(b) => Cow::Owned(b.to_string()),
				Cow::Owned(b) => Cow::Owned(b)
			},
			failure_behavior: self.failure_behavior,
			children: self.children.into_iter().map(Element::into_owned).collect()
		}
	}
}

impl<'s> Serialize for Lang<'s> {
	fn serialize_xml<W: Write>(&self, writer: &mut XmlWriter<W>, options: &SerializeOptions) -> crate::Result<()> {
		writer.element("lang", |writer| {
			writer.attr("xml:lang", &*self.language)?;
			writer.attr_opt("onlangfailure", self.failure_behavior.as_ref().map(LangFailure::as_str))?;
			util::serialize_elements(writer, &self.children, options)
		})
	}
}

impl<'s, 's2: 's, T: Into<Element<'s2>>> Add<T> for Lang<'s> {
	type Output = Lang<'s>;

	fn add(mut self, rhs: T) -> Self::Output {
		self.push(rhs.into());
		self
	}
}

impl<'s, 's2: 's, T: Into<Element<'s2>>> AddAssign<T> for Lang<'s> {
	fn add_assign(&mut self, rhs: T) {
		self.push(rhs.into());
	}
}

pub fn lang<'s, S: Into<Element<'s>>, I: IntoIterator<Item = S>>(lang: impl Into<Cow<'s, str>>, elements: I) -> Lang<'s> {
	Lang::new(lang, elements)
}
