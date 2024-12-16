use alloc::{borrow::Cow, string::ToString, vec::Vec};
use core::{
	fmt::{Debug, Write},
	ops::{Add, AddAssign}
};

use crate::{Element, Flavor, Serialize, SerializeOptions, XmlWriter, util};

/// The root element of an SSML document.
#[derive(Clone, Default, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Speak<'s> {
	children: Vec<Element<'s>>,
	marks: (Option<Cow<'s, str>>, Option<Cow<'s, str>>),
	lang: Option<Cow<'s, str>>
}

impl<'s> Speak<'s> {
	/// Creates a new SSML document with elements.
	///
	/// `lang` specifies the language of the spoken text contained within the document, e.g. `en-US`. It is required for
	/// ACSS and will throw an error if not provided.
	///
	/// ```
	/// let doc = ssml::speak(Some("en-US"), ["Hello, world!"]);
	/// ```
	pub fn new<S: Into<Element<'s>>, I: IntoIterator<Item = S>>(lang: Option<&'s str>, elements: I) -> Self {
		Self {
			children: elements.into_iter().map(|f| f.into()).collect(),
			lang: lang.map(|f| Cow::Borrowed(f)),
			..Speak::default()
		}
	}

	pub fn with_start_mark(mut self, mark: impl Into<Cow<'s, str>>) -> Self {
		self.marks.0 = Some(mark.into());
		self
	}

	pub fn start_mark(&self) -> Option<&str> {
		self.marks.0.as_deref()
	}

	pub fn set_start_mark(&mut self, mark: impl Into<Cow<'s, str>>) {
		self.marks.0 = Some(mark.into());
	}

	pub fn take_start_mark(&mut self) -> Option<Cow<'s, str>> {
		self.marks.0.take()
	}

	pub fn with_end_mark(mut self, mark: impl Into<Cow<'s, str>>) -> Self {
		self.marks.1 = Some(mark.into());
		self
	}

	pub fn end_mark(&self) -> Option<&str> {
		self.marks.1.as_deref()
	}

	pub fn set_end_mark(&mut self, mark: impl Into<Cow<'s, str>>) {
		self.marks.1 = Some(mark.into());
	}

	pub fn take_end_mark(&mut self) -> Option<Cow<'s, str>> {
		self.marks.1.take()
	}

	/// Extend this SSML document with an additional element.
	///
	/// ```
	/// # use ssml::{Flavor, Serialize};
	/// # fn main() -> ssml::Result<()> {
	/// let mut doc = ssml::speak(Some("en-US"), ["Hello, world!"]);
	/// doc.push("This is an SSML document.");
	///
	/// assert_eq!(
	/// 	doc.serialize_to_string(&ssml::SerializeOptions::default().pretty())?,
	/// 	r#"<speak version="1.0" xmlns="http://www.w3.org/2001/10/synthesis" xml:lang="en-US">
	/// 	Hello, world!
	/// 	This is an SSML document.
	/// </speak>"#
	/// );
	/// # Ok(())
	/// # }
	/// ```
	pub fn push(&mut self, element: impl Into<Element<'s>>) {
		self.children.push(element.into());
	}

	/// Extend this SSML document with additional elements.
	///
	/// ```
	/// # use ssml::{Flavor, Serialize};
	/// # fn main() -> ssml::Result<()> {
	/// let mut doc = ssml::speak(Some("en-US"), ["Hello, world!"]);
	/// doc.extend(["This is an SSML document."]);
	///
	/// assert_eq!(
	/// 	doc.serialize_to_string(&ssml::SerializeOptions::default().pretty())?,
	/// 	r#"<speak version="1.0" xmlns="http://www.w3.org/2001/10/synthesis" xml:lang="en-US">
	/// 	Hello, world!
	/// 	This is an SSML document.
	/// </speak>"#
	/// );
	/// # Ok(())
	/// # }
	/// ```
	pub fn extend<S: Into<Element<'s>>, I: IntoIterator<Item = S>>(&mut self, elements: I) {
		self.children.extend(elements.into_iter().map(|f| f.into()));
	}

	/// Returns a reference to the document's direct children.
	pub fn children(&self) -> &[Element<'s>] {
		&self.children
	}

	/// Returns a mutable reference to the document's direct children.
	pub fn children_mut(&mut self) -> &mut Vec<Element<'s>> {
		&mut self.children
	}

	pub fn to_owned(&self) -> Speak<'static> {
		self.clone().into_owned()
	}

	pub fn into_owned(self) -> Speak<'static> {
		Speak {
			children: self.children.into_iter().map(Element::into_owned).collect(),
			marks: (
				match self.marks.0 {
					Some(Cow::Borrowed(b)) => Some(Cow::Owned(b.to_string())),
					Some(Cow::Owned(b)) => Some(Cow::Owned(b)),
					None => None
				},
				match self.marks.1 {
					Some(Cow::Borrowed(b)) => Some(Cow::Owned(b.to_string())),
					Some(Cow::Owned(b)) => Some(Cow::Owned(b)),
					None => None
				}
			),
			lang: match self.lang {
				Some(Cow::Borrowed(b)) => Some(Cow::Owned(b.to_string())),
				Some(Cow::Owned(b)) => Some(Cow::Owned(b)),
				None => None
			}
		}
	}
}

impl<'s> Serialize for Speak<'s> {
	fn serialize_xml<W: Write>(&self, writer: &mut XmlWriter<W>, options: &SerializeOptions) -> crate::Result<()> {
		writer.element("speak", |writer| {
			if matches!(options.flavor, Flavor::Generic | Flavor::MicrosoftAzureCognitiveSpeechServices) {
				writer.attr("version", "1.0")?;
				writer.attr("xmlns", "http://www.w3.org/2001/10/synthesis")?;
			}

			writer.attr_opt("xml:lang", self.lang.as_deref())?;
			// Include `mstts` namespace for ACSS.
			if options.flavor == Flavor::MicrosoftAzureCognitiveSpeechServices {
				writer.attr("xmlns:mstts", "http://www.w3.org/2001/mstts")?;
			}

			writer.attr_opt("startmark", self.marks.0.as_deref())?;
			writer.attr_opt("endmark", self.marks.1.as_deref())?;

			util::serialize_elements(writer, &self.children, options)
		})
	}
}

impl<'s, 's2: 's, T: Into<Element<'s2>>> Add<T> for Speak<'s> {
	type Output = Speak<'s>;

	fn add(mut self, rhs: T) -> Self::Output {
		self.push(rhs.into());
		self
	}
}

impl<'s, 's2: 's, T: Into<Element<'s2>>> AddAssign<T> for Speak<'s> {
	fn add_assign(&mut self, rhs: T) {
		self.push(rhs.into());
	}
}

/// Creates a new SSML document with elements.
///
/// `lang` specifies the language of the spoken text contained within the document, e.g. `en-US`. It is required for
/// ACSS and will throw an error if not provided.
///
/// ```
/// # use ssml::Serialize;
/// # fn main() -> ssml::Result<()> {
/// let doc = ssml::speak(Some("en-US"), ["Hello, world!"]);
///
/// let str = doc.serialize_to_string(&ssml::SerializeOptions::default().pretty())?;
/// assert_eq!(
/// 	str,
/// 	r#"<speak version="1.0" xmlns="http://www.w3.org/2001/10/synthesis" xml:lang="en-US">
/// 	Hello, world!
/// </speak>"#
/// );
/// # Ok(())
/// # }
/// ```
pub fn speak<'s, S: Into<Element<'s>>, I: IntoIterator<Item = S>>(lang: Option<&'s str>, elements: I) -> Speak<'s> {
	Speak {
		children: elements.into_iter().map(|f| f.into()).collect(),
		lang: lang.map(|f| Cow::Borrowed(f)),
		..Speak::default()
	}
}
