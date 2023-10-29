use std::{fmt::Debug, io::Write};

use crate::{custom::CustomElement, util, Audio, Flavor, Meta, Serialize, Text, Voice};

macro_rules! el {
	(
		$(#[$outer:meta])*
		pub enum $name:ident {
			$(
				$(#[$innermeta:meta])*
				$variant:ident($inner:ty)
			),*
		}
	) => {
		$(#[$outer])*
		pub enum $name {
			$(
				$(#[$innermeta])*
				$variant($inner)
			),*
		}

		$(impl From<$inner> for $name {
			fn from(val: $inner) -> $name {
				$name::$variant(val)
			}
		})*

		impl $crate::Serialize for $name {
			fn serialize<W: std::io::Write>(&self, writer: &mut W, flavor: $crate::Flavor) -> anyhow::Result<()> {
				match self {
					$($name::$variant(inner) => inner.serialize(writer, flavor),)*
				}
			}
		}
	};
}

el! {
	#[derive(Clone, Debug)]
	#[non_exhaustive]
	pub enum Element {
		Text(Text),
		Audio(Audio),
		Voice(Voice),
		Meta(Meta),
		Custom(Box<dyn CustomElement>)
		// Break(BreakElement),
		// Emphasis(EmphasisElement),
		// Lang(LangElement),
		// Mark(MarkElement),
		// Paragraph(ParagraphElement),
		// Phoneme(PhonemeElement),
		// Prosody(ProsodyElement),
		// SayAs(SayAsElement),
		// Sub(SubElement),
		// Sentence(SentenceElement),
		// Voice(VoiceElement),
		// Word(WordElement)
	}
}

impl<T: ToString> From<T> for Element {
	fn from(value: T) -> Self {
		Element::Text(Text(value.to_string()))
	}
}

/// The root element of an SSML document.
#[derive(Default, Debug)]
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
	/// # fn main() -> anyhow::Result<()> {
	/// let mut doc = ssml::speak(Some("en-US"), ["Hello, world!"]);
	/// doc.push("This is an SSML document.");
	///
	/// assert_eq!(
	/// 	doc.serialize_to_string(Flavor::AmazonPolly)?,
	/// 	r#"<speak xml:lang="en-US">Hello, world! This is an SSML document.</speak>"#
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
	/// # fn main() -> anyhow::Result<()> {
	/// let mut doc = ssml::speak(Some("en-US"), ["Hello, world!"]);
	/// doc.extend(["This is an SSML document."]);
	///
	/// assert_eq!(
	/// 	doc.serialize_to_string(Flavor::AmazonPolly)?,
	/// 	r#"<speak xml:lang="en-US">Hello, world! This is an SSML document.</speak>"#
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
	fn serialize<W: Write>(&self, writer: &mut W, flavor: Flavor) -> anyhow::Result<()> {
		writer.write_all(b"<speak")?;
		if flavor == Flavor::Generic || flavor == Flavor::MicrosoftAzureCognitiveSpeechServices {
			util::write_attr(writer, "version", "1.0")?;
			util::write_attr(writer, "xmlns", "http://www.w3.org/2001/10/synthesis")?;
		}

		if let Some(lang) = &self.lang {
			util::write_attr(writer, "xml:lang", lang)?;
		} else if flavor == Flavor::MicrosoftAzureCognitiveSpeechServices {
			return Err(crate::error!("{flavor:?} requires a language to be set"))?;
		}

		// Include `mstts` namespace for ACSS.
		if flavor == Flavor::MicrosoftAzureCognitiveSpeechServices {
			util::write_attr(writer, "xmlns:mstts", "http://www.w3.org/2001/mstts")?;
		}

		if let Some(start_mark) = &self.marks.0 {
			util::write_attr(writer, "startmark", start_mark)?;
		}
		if let Some(end_mark) = &self.marks.1 {
			util::write_attr(writer, "endmark", end_mark)?;
		}

		writer.write_all(b">")?;

		util::serialize_elements(writer, &self.children, flavor)?;

		writer.write_all(b"</speak>")?;
		Ok(())
	}
}

/// Creates a new SSML document with elements.
///
/// `lang` specifies the language of the spoken text contained within the document, e.g. `en-US`. It is required for
/// ACSS and will throw an error if not provided.
///
/// ```
/// # use ssml::Serialize;
/// # fn main() -> anyhow::Result<()> {
/// let doc = ssml::speak(Some("en-US"), ["Hello, world!"]);
///
/// let str = doc.serialize_to_string(ssml::Flavor::AmazonPolly)?;
/// assert_eq!(str, r#"<speak xml:lang="en-US">Hello, world!</speak>"#);
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
