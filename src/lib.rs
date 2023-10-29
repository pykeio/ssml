//! Utilities for writing SSML documents.
//!
//! The root document in SSML is [`Speak`]. Use [`speak()`] to quickly create a document.
//!
//! ```
//! let doc = ssml::speak(Some("en-US"), ["Hello, world!"]);
//! ```
//!
//! Use [`Serialize`] to convert SSML elements to their string XML representation, which can then be sent to your speech
//! synthesis service of chocie.
//!
//! ```
//! use ssml::Serialize;
//! # fn main() -> anyhow::Result<()> {
//! # let doc = ssml::speak(Some("en-US"), ["Hello, world!"]);
//! let str = doc.serialize_to_string(ssml::Flavor::AmazonPolly)?;
//! assert_eq!(str, r#"<speak xml:lang="en-US">Hello, world!</speak>"#);
//! # Ok(())
//! # }
//! ```

#![allow(clippy::tabs_in_doc_comments)]

use std::{fmt::Debug, io::Write};

mod audio;
mod custom;
mod error;
pub mod mstts;
mod speak;
mod text;
mod unit;
pub mod util;
pub mod visit;
pub mod visit_mut;
mod voice;

pub(crate) use self::error::{error, GenericError};
pub use self::{
	audio::{audio, Audio, AudioRepeat},
	speak::{speak, Element, Speak},
	text::{text, Text},
	unit::{Decibels, DecibelsError, TimeDesignation, TimeDesignationError},
	voice::{voice, Voice, VoiceConfig, VoiceGender}
};

/// Vendor-specific flavor of SSML. Specifying this can be used to enable compatibility checks & add additional
/// metadata required by certain services.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Flavor {
	/// Generic SSML.
	///
	/// This skips any compatibility checks and assumes all elements are supported.
	#[default]
	Generic,
	/// Microsoft Azure Cognitive Speech Services (ACSS / MSTTS) flavored SSML.
	///
	/// Selecting this flavor will namely add the proper `xmlns` to the XML document, which is required by ACSS.
	MicrosoftAzureCognitiveSpeechServices,
	/// Google Cloud Text-to-Speech (GCTTS) flavored SSML.
	GoogleCloudTextToSpeech,
	/// Amazon Polly flavored SSML.
	///
	/// This will use compatibility checks for Standard voices only. Some SSML elements are not supported by Neural
	/// voices. See the [Amazon Polly documentation](https://docs.aws.amazon.com/polly/latest/dg/supportedtags.html)
	/// for more information on what tags Neural voices do not support.
	AmazonPolly,
	/// pyke Songbird flavored SSML.
	PykeSongbird
}

/// Trait to support serializing SSML elements.
pub trait Serialize {
	/// Serialize this SSML element into a [`Write`]r.
	fn serialize<W: Write>(&self, writer: &mut W, flavor: Flavor) -> anyhow::Result<()>;

	/// Serialize this SSML element into a string.
	fn serialize_to_string(&self, flavor: Flavor) -> anyhow::Result<String> {
		let mut write = Vec::new();
		self.serialize(&mut write, flavor)?;
		Ok(std::str::from_utf8(&write)?.to_owned())
	}
}

/// An [`Element`] that outputs a string of XML.
///
/// It differs from [`Text`] in that the contents of `Meta` are not escaped, meaning `Meta` can be used to write raw
/// XML into the document.
#[derive(Debug, Clone)]
pub struct Meta {
	raw: String,
	name: Option<String>,
	restrict_flavor: Option<Vec<Flavor>>
}

impl Meta {
	pub fn new(xml: impl ToString) -> Self {
		Meta {
			raw: xml.to_string(),
			name: None,
			restrict_flavor: None
		}
	}

	pub fn with_name(mut self, name: impl ToString) -> Self {
		self.name = Some(name.to_string());
		self
	}

	pub fn with_restrict_flavor(mut self, flavors: impl IntoIterator<Item = Flavor>) -> Self {
		self.restrict_flavor = Some(flavors.into_iter().collect());
		self
	}
}

impl Serialize for Meta {
	fn serialize<W: Write>(&self, writer: &mut W, flavor: Flavor) -> anyhow::Result<()> {
		if let Some(flavors) = self.restrict_flavor.as_ref() {
			if !flavors.iter().any(|f| f == &flavor) {
				anyhow::bail!("{} cannot be used with {flavor:?}", if let Some(name) = &self.name { name } else { "this meta element" });
			}
		}
		Ok(writer.write_all(self.raw.as_bytes())?)
	}
}
