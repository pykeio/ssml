use std::{error::Error, io::Write};

use crate::{util, Audio, Flavor, Serialize, Text};

macro_rules! el {
	(
		$(#[$outer:meta])*
		pub enum $name:ident {
			$(
				$(#[$innermeta:meta])*
				$variant:ident($inner:ident)
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
			fn serialize<W: std::io::Write>(&self, writer: &mut W, flavor: $crate::Flavor) -> std::result::Result<(), std::boxed::Box<(dyn std::error::Error + 'static)>> {
				match self {
					$($name::$variant(inner) => inner.serialize(writer, flavor),)*
				}
			}
		}
	};
}

el! {
	#[derive(Clone)]
	pub enum SpeakableElement {
		Text(Text),
		Audio(Audio)
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

impl<T: ToString> From<T> for SpeakableElement {
	fn from(value: T) -> Self {
		SpeakableElement::Text(Text(value.to_string()))
	}
}

/// The root element of an SSML document.
#[derive(Default)]
pub struct Speak {
	elements: Vec<SpeakableElement>,
	marks: (Option<String>, Option<String>),
	lang: Option<String>
}

impl Speak {
	/// Creates a new SSML document with spoken elements.
	///
	/// `lang` specifies the language of the spoken text contained within the document, e.g. `en-US`. It is required for
	/// ACSS and will throw an error if not provided.
	///
	/// ```
	/// ssml::Speak::new(Some("en-US"), ["Hello, world!"]);
	/// ```
	pub fn new<S: Into<SpeakableElement>, I: IntoIterator<Item = S>>(lang: Option<&str>, elements: I) -> Self {
		Self {
			elements: elements.into_iter().map(|f| f.into()).collect(),
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

	/// Extend this SSML document with additional spoken elements.
	///
	/// ```
	/// # use ssml::{Flavor, Serialize};
	/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
	/// let mut doc = ssml::speak(Some("en-US"), ["Hello, world!"]);
	/// doc = doc.with_elements(["This is an SSML document."]);
	///
	/// assert_eq!(
	/// 	doc.serialize_to_string(Flavor::AmazonPolly)?,
	/// 	r#"<speak xml:lang="en-US">Hello, world! This is an SSML document. </speak>"#
	/// );
	/// # Ok(())
	/// # }
	/// ```
	pub fn with_elements<S: Into<SpeakableElement>, I: IntoIterator<Item = S>>(mut self, elements: I) -> Self {
		self.elements.extend(elements.into_iter().map(|f| f.into()));
		self
	}
}

impl Serialize for Speak {
	fn serialize<W: Write>(&self, writer: &mut W, flavor: Flavor) -> Result<(), Box<dyn Error>> {
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
			util::write_attr(writer, "xmlns:mstts", "https://www.w3.org/2001/mstts")?;
		}

		if let Some(start_mark) = &self.marks.0 {
			util::write_attr(writer, "startmark", start_mark)?;
		}
		if let Some(end_mark) = &self.marks.1 {
			util::write_attr(writer, "endmark", end_mark)?;
		}

		writer.write_all(b">")?;

		for el in &self.elements {
			el.serialize(writer, flavor)?;
		}

		writer.write_all(b"</speak>")?;
		Ok(())
	}
}

/// Creates a new SSML document with spoken elements.
///
/// `lang` specifies the language of the spoken text contained within the document, e.g. `en-US`. It is required for
/// ACSS and will throw an error if not provided.
///
/// ```
/// # use ssml::Serialize;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let doc = ssml::speak(Some("en-US"), ["Hello, world!"]);
///
/// let str = doc.serialize_to_string(ssml::Flavor::AmazonPolly)?;
/// assert_eq!(str, r#"<speak xml:lang="en-US">Hello, world! </speak>"#);
/// # Ok(())
/// # }
/// ```
pub fn speak<S: Into<SpeakableElement>, I: IntoIterator<Item = S>>(lang: Option<&str>, elements: I) -> Speak {
	Speak {
		elements: elements.into_iter().map(|f| f.into()).collect(),
		lang: lang.map(|f| f.to_owned()),
		..Speak::default()
	}
}
