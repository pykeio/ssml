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
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let doc = ssml::speak(Some("en-US"), ["Hello, world!"]);
//! let str = doc.serialize_to_string(ssml::Flavor::AmazonPolly)?;
//! assert_eq!(str, r#"<speak xml:lang="en-US">Hello, world! </speak>"#);
//! # Ok(())
//! # }
//! ```

#![allow(clippy::tabs_in_doc_comments)]

use std::{error::Error, fmt::Debug, io::Write};

mod audio;
mod error;
pub mod mstts;
mod speak;
mod text;
mod unit;
mod util;
mod voice;

pub(crate) use self::error::{error, GenericError};
pub use self::{
	audio::{audio, Audio, AudioRepeat},
	speak::{speak, Speak, SpeakableElement},
	text::{text, Text},
	unit::{Decibels, DecibelsError, TimeDesignation, TimeDesignationError},
	voice::{voice, Voice, VoiceConfig, VoiceGender}
};

/// Vendor-specific flavor of SSML. Specifying this can be used to enable compatibility checks & add additional
/// metadata required by certain services.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Flavor {
	/// Generic SSML.
	///
	/// This skips any compatibility checks and assumes all elements are supported.
	#[default]
	Generic,
	/// Microsoft Azure Cognitive Speech Services (ACSS) flavored SSML.
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
	fn serialize<W: Write>(&self, writer: &mut W, flavor: Flavor) -> Result<(), Box<dyn Error>>;

	/// Serialize this SSML element into a string.
	fn serialize_to_string(&self, flavor: Flavor) -> Result<String, Box<dyn Error>> {
		let mut write = Vec::new();
		self.serialize(&mut write, flavor)?;
		Ok(std::str::from_utf8(&write)?.to_owned())
	}
}

/// A [`SpeakableElement`] that outputs a simple string.
///
/// It differs from [`Text`] in that the contents of `Meta` are not escaped, meaning `Meta` can be used to write raw
/// XML into the document.
#[derive(Debug, Clone)]
pub struct Meta(pub String);

impl Serialize for Meta {
	fn serialize<W: Write>(&self, writer: &mut W, _: Flavor) -> Result<(), Box<dyn Error>> {
		Ok(writer.write_all(self.0.as_bytes())?)
	}
}
