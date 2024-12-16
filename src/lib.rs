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

#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![allow(clippy::tabs_in_doc_comments)]

extern crate alloc;
extern crate core;

use alloc::{
	borrow::Cow,
	string::{String, ToString}
};
use core::fmt::{Debug, Write};

mod audio;
mod r#break;
mod element;
mod emphasis;
mod error;
mod group;
mod lang;
mod mark;
pub mod mstts;
mod prosody;
mod say_as;
mod speak;
mod text;
mod unit;
pub mod util;
pub mod visit;
pub mod visit_mut;
mod voice;
mod xml;

pub use self::{
	audio::{Audio, AudioRepeat, audio},
	r#break::{Break, BreakStrength, breaks},
	element::{CustomElement, Element, IntoElement},
	emphasis::{Emphasis, EmphasisLevel, emphasis},
	error::{Error, Result},
	group::{Group, group},
	lang::{Lang, lang},
	mark::{Mark, mark},
	prosody::{Prosody, ProsodyContour, ProsodyControl, ProsodyPitch, ProsodyRate, ProsodyVolume, prosody},
	say_as::{DateFormat, SayAs, SpeechFormat, say_as},
	speak::{Speak, speak},
	text::{Text, text},
	unit::{Decibels, DecibelsError, TimeDesignation, TimeDesignationError},
	voice::{Voice, VoiceConfig, VoiceGender, voice},
	xml::{EscapedDisplay, XmlWriter}
};

/// Vendor-specific flavor of SSML. Specifying this can be used to enable compatibility checks & add additional
/// metadata required by certain services.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
	pub pretty: bool
}

impl Default for SerializeOptions {
	fn default() -> Self {
		SerializeOptions {
			flavor: Flavor::Generic,
			pretty: false
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
	fn serialize_xml<W: Write>(&self, writer: &mut XmlWriter<W>, options: &SerializeOptions) -> crate::Result<()>;

	/// Serialize this SSML element into a string.
	fn serialize_to_string(&self, options: &SerializeOptions) -> crate::Result<String> {
		let mut out = String::new();
		self.serialize(&mut out, options)?;
		Ok(out)
	}
}

/// An [`Element`] that outputs a string of XML.
///
/// It differs from [`Text`] in that the contents of `Meta` are not escaped, meaning `Meta` can be used to write raw
/// XML into the document.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Meta<'s> {
	raw: Cow<'s, str>,
	name: Option<Cow<'s, str>>
}

impl<'s> Meta<'s> {
	pub fn new(xml: impl Into<Cow<'s, str>>) -> Self {
		Meta { raw: xml.into(), name: None }
	}

	pub fn with_name(mut self, name: impl Into<Cow<'s, str>>) -> Self {
		self.name = Some(name.into());
		self
	}

	pub fn to_owned(&self) -> Meta<'static> {
		self.clone().into_owned()
	}

	pub fn into_owned(self) -> Meta<'static> {
		Meta {
			raw: match self.raw {
				Cow::Borrowed(b) => Cow::Owned(b.to_string()),
				Cow::Owned(b) => Cow::Owned(b)
			},
			name: match self.name {
				Some(Cow::Borrowed(b)) => Some(Cow::Owned(b.to_string())),
				Some(Cow::Owned(b)) => Some(Cow::Owned(b)),
				None => None
			}
		}
	}
}

impl<'s> Serialize for Meta<'s> {
	fn serialize_xml<W: Write>(&self, writer: &mut XmlWriter<W>, _: &SerializeOptions) -> crate::Result<()> {
		writer.raw(&self.raw)
	}
}
