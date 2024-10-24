use std::fmt::{Debug, Write};

use crate::{Element, Flavor, Serialize, SerializeOptions, XmlWriter, util};

/// The root element of an SSML document.
#[derive(Default, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Speak {
	children: Vec<Element>,
	marks: (Option<String>, Option<String>),
	lang: Option<String>
}

impl Speak {
	/// Creates a new SSML document with elements.
	///
	/// `lang` specifies the language of the spoken text contained within the document, e.g. `en-US`. It is required for
	/// ACSS and will throw an error if not provided.
	///
	/// ```
	/// let doc = ssml::speak(Some("en-US"), ["Hello, world!"]);
	/// ```
	pub fn new<S: Into<Element>, I: IntoIterator<Item = S>>(lang: Option<&str>, elements: I) -> Self {
		Self {
			children: elements.into_iter().map(|f| f.into()).collect(),
			lang: lang.map(|f| f.to_owned()),
			..Speak::default()
		}
	}

	pub fn with_start_mark(mut self, mark: impl ToString) -> Self {
		self.marks.0 = Some(mark.to_string());
		self
	}

	pub fn with_end_mark(mut self, mark: impl ToString) -> Self {
		self.marks.1 = Some(mark.to_string());
		self
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
	pub fn push(&mut self, element: impl Into<Element>) {
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
	pub fn extend<S: Into<Element>, I: IntoIterator<Item = S>>(&mut self, elements: I) {
		self.children.extend(elements.into_iter().map(|f| f.into()));
	}

	/// Returns a reference to the document's direct children.
	pub fn children(&self) -> &[Element] {
		&self.children
	}

	/// Returns a mutable reference to the document's direct children.
	pub fn children_mut(&mut self) -> &mut [Element] {
		&mut self.children
	}
}

impl Serialize for Speak {
	fn serialize_xml<W: Write>(&self, writer: &mut XmlWriter<W>, options: &SerializeOptions) -> crate::Result<()> {
		if options.perform_checks && self.lang.is_none() && options.flavor == Flavor::MicrosoftAzureCognitiveSpeechServices {
			return Err(crate::error!("{:?} requires a language to be set", options.flavor))?;
		}

		writer.element("speak", |writer| {
			if matches!(options.flavor, Flavor::Generic | Flavor::MicrosoftAzureCognitiveSpeechServices) {
				writer.attr("version", "1.0")?;
				writer.attr("xmlns", "http://www.w3.org/2001/10/synthesis")?;
			}

			writer.attr_opt("xml:lang", self.lang.as_ref())?;
			// Include `mstts` namespace for ACSS.
			if options.flavor == Flavor::MicrosoftAzureCognitiveSpeechServices {
				writer.attr("xmlns:mstts", "http://www.w3.org/2001/mstts")?;
			}

			writer.attr_opt("startmark", self.marks.0.as_ref())?;
			writer.attr_opt("endmark", self.marks.1.as_ref())?;

			util::serialize_elements(writer, &self.children, options)
		})
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
pub fn speak<S: Into<Element>, I: IntoIterator<Item = S>>(lang: Option<&str>, elements: I) -> Speak {
	Speak {
		children: elements.into_iter().map(|f| f.into()).collect(),
		lang: lang.map(|f| f.to_owned()),
		..Speak::default()
	}
}
