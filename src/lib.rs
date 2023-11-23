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
//! # fn main() -> ssml::Result<()> {
//! # let doc = ssml::speak(Some("en-US"), ["Hello, world!"]);
//! let str = doc.serialize_to_string(&ssml::SerializeOptions::default().pretty())?;
//! assert_eq!(
//! 	str,
//! 	r#"<speak version="1.0" xmlns="http://www.w3.org/2001/10/synthesis" xml:lang="en-US">
//! 	Hello, world!
//! </speak>"#
//! );
//! # Ok(())
//! # }
//! ```

#![allow(clippy::tabs_in_doc_comments)]

use std::{fmt::Debug, io::Write};

mod audio;
mod r#break;
mod element;
mod emphasis;
mod error;
mod mark;
pub mod mstts;
mod speak;
mod text;
mod unit;
pub mod util;
pub mod visit;
pub mod visit_mut;
mod voice;
mod xml;

pub(crate) use self::error::error;
pub use self::{
	audio::{audio, Audio, AudioRepeat},
	r#break::{breaks, Break, BreakStrength},
	element::{DynElement, Element},
	emphasis::{emphasis, Emphasis, EmphasisLevel},
	error::{Error, Result},
	mark::{mark, Mark},
	speak::{speak, Speak},
	text::{text, Text},
	unit::{Decibels, DecibelsError, TimeDesignation, TimeDesignationError},
	voice::{voice, Voice, VoiceConfig, VoiceGender},
	xml::XmlWriter
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

/// Configuration for elements that support [`Serialize`].
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct SerializeOptions {
	/// The flavor of SSML to output; see [`Flavor`]. When `perform_checks` is enabled (which it is by default), this
	/// can help catch compatibility issues with different speech synthesis providers.
	pub flavor: Flavor,
	/// Whether or not to format the outputted SSML in a human-readable format.
	///
	/// Generally, this should only be used for debugging. Some providers may charge per SSML character (not just spoken
	/// character), so enabling this option in production may significantly increase costs.
	pub pretty: bool,
	/// Whether or not to perform compatibility checks with the chosen flavor. This is enabled by default.
	pub perform_checks: bool
}

impl Default for SerializeOptions {
	fn default() -> Self {
		SerializeOptions {
			flavor: Flavor::Generic,
			pretty: false,
			perform_checks: true
		}
	}
}

impl SerializeOptions {
	pub fn min(mut self) -> Self {
		self.pretty = false;
		self
	}

	pub fn pretty(mut self) -> Self {
		self.pretty = true;
		self
	}

	pub fn flavor(mut self, flavor: Flavor) -> Self {
		self.flavor = flavor;
		self
	}

	pub fn perform_checks(mut self) -> Self {
		self.perform_checks = true;
		self
	}

	pub fn no_checks(mut self) -> Self {
		self.perform_checks = false;
		self
	}
}

/// Trait to support serializing SSML elements.
pub trait Serialize {
	/// Serialize this SSML element into an `std` [`Write`]r.
	fn serialize<W: Write>(&self, writer: &mut W, options: &SerializeOptions) -> crate::Result<()> {
		let mut writer = XmlWriter::new(writer, options.pretty);
		self.serialize_xml(&mut writer, options)?;
		Ok(())
	}

	/// Serialize this SSML element into an [`XmlWriter`].
	fn serialize_xml(&self, writer: &mut XmlWriter<'_>, options: &SerializeOptions) -> crate::Result<()>;

	/// Serialize this SSML element into a string.
	fn serialize_to_string(&self, options: &SerializeOptions) -> crate::Result<String> {
		let mut write = Vec::new();
		self.serialize(&mut write, options)?;
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
	fn serialize_xml(&self, writer: &mut XmlWriter<'_>, options: &SerializeOptions) -> crate::Result<()> {
		if options.perform_checks {
			if let Some(flavors) = self.restrict_flavor.as_ref() {
				if !flavors.iter().any(|f| f == &options.flavor) {
					return Err(crate::error!(
						"{} cannot be used with {:?}",
						if let Some(name) = &self.name { name } else { "this meta element" },
						options.flavor
					));
				}
			}
		}
		writer.raw(&self.raw)
	}
}
