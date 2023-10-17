use std::{fmt::Display, io::Write};

use crate::{util, Flavor, Serialize, SpeakableElement};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum VoiceGender {
	#[default]
	Unspecified,
	Neutral,
	Female,
	Male
}

impl Display for VoiceGender {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			VoiceGender::Unspecified => "",
			VoiceGender::Neutral => "neutral",
			VoiceGender::Female => "female",
			VoiceGender::Male => "male"
		})
	}
}

/// Configuration for the [`Voice`] element.
#[derive(Default, Debug, Clone)]
pub struct VoiceConfig {
	pub gender: Option<VoiceGender>,
	pub age: Option<u8>,
	pub name: Option<String>,
	pub variant: Option<String>
}

impl VoiceConfig {
	/// Creates a new [`VoiceConfig`] with the specified voice name and no other attributes.
	///
	/// ```
	/// let doc = ssml::VoiceConfig::named("en-US-JennyNeural");
	/// ```
	pub fn named(name: impl ToString) -> Self {
		Self {
			name: Some(name.to_string()),
			..VoiceConfig::default()
		}
	}
}

impl<S: ToString> From<S> for VoiceConfig {
	fn from(value: S) -> Self {
		VoiceConfig::named(value)
	}
}

impl Serialize for VoiceConfig {
	fn serialize<W: Write>(&self, writer: &mut W, _: Flavor) -> anyhow::Result<()> {
		if let Some(gender) = &self.gender {
			util::write_attr(writer, "gender", gender.to_string())?;
		}
		if let Some(age) = &self.age {
			util::write_attr(writer, "age", age.to_string())?;
		}
		if let Some(name) = &self.name {
			util::write_attr(writer, "name", name)?;
		}
		if let Some(variant) = &self.variant {
			util::write_attr(writer, "variant", variant)?;
		}
		Ok(())
	}
}

/// The [`Voice`] element allows you to specify a voice or use multiple different voices in one document.
#[derive(Default, Debug, Clone)]
pub struct Voice {
	pub(crate) elements: Vec<SpeakableElement>,
	config: VoiceConfig
}

impl Voice {
	/// Creates a new `voice` element to change the voice of a section spoken elements.
	///
	/// ```
	/// # use ssml::{Flavor, Serialize};
	/// # fn main() -> anyhow::Result<()> {
	/// let doc = ssml::Speak::new(None, [ssml::Voice::new("en-US-Neural2-F", ["Hello, world!"])]);
	///
	/// assert_eq!(
	/// 	doc.serialize_to_string(Flavor::GoogleCloudTextToSpeech)?,
	/// 	r#"<speak><voice name="en-US-Neural2-F">Hello, world! </voice></speak>"#
	/// );
	/// # Ok(())
	/// # }
	/// ```
	pub fn new<S: Into<SpeakableElement>, I: IntoIterator<Item = S>>(config: impl Into<VoiceConfig>, elements: I) -> Self {
		Self {
			elements: elements.into_iter().map(|f| f.into()).collect(),
			config: config.into()
		}
	}

	/// Extend this `voice` section with additional spoken elements.
	///
	/// ```
	/// # use ssml::{Flavor, Serialize};
	/// # fn main() -> anyhow::Result<()> {
	/// let mut voice = ssml::voice("en-US-Neural2-F", ["Hello, world!"]);
	/// voice = voice.with_elements(["This is an SSML document."]);
	/// let doc = ssml::Speak::new(None, [voice]);
	///
	/// assert_eq!(
	/// 	doc.serialize_to_string(Flavor::GoogleCloudTextToSpeech)?,
	/// 	r#"<speak><voice name="en-US-Neural2-F">Hello, world! This is an SSML document. </voice></speak>"#
	/// );
	/// # Ok(())
	/// # }
	/// ```
	pub fn with_elements<S: Into<SpeakableElement>, I: IntoIterator<Item = S>>(mut self, elements: I) -> Self {
		self.elements.extend(elements.into_iter().map(|f| f.into()));
		self
	}

	/// Modifies the voice configuration of this `voice` section.
	///
	/// ```
	/// let mut voice = ssml::Voice::default();
	/// voice = voice.with_voice(ssml::VoiceConfig { age: Some(42), ..Default::default() });
	/// ```
	pub fn with_voice(mut self, config: impl Into<VoiceConfig>) -> Self {
		self.config = config.into();
		self
	}
}

impl Serialize for Voice {
	fn serialize<W: Write>(&self, writer: &mut W, flavor: Flavor) -> anyhow::Result<()> {
		writer.write_all(b"<voice")?;
		self.config.serialize(writer, flavor)?;
		writer.write_all(b">")?;
		for el in &self.elements {
			el.serialize(writer, flavor)?;
		}
		writer.write_all(b"</voice>")?;
		Ok(())
	}
}

/// Creates a new `voice` element to change the voice of a section spoken elements.
///
/// ```
/// # use ssml::{Flavor, Serialize};
/// # fn main() -> anyhow::Result<()> {
/// let doc = ssml::speak(None, [ssml::voice("en-US-Neural2-F", ["Hello, world!"])]);
///
/// assert_eq!(
/// 	doc.serialize_to_string(Flavor::GoogleCloudTextToSpeech)?,
/// 	r#"<speak><voice name="en-US-Neural2-F">Hello, world! </voice></speak>"#
/// );
/// # Ok(())
/// # }
/// ```
pub fn voice<S: Into<SpeakableElement>, I: IntoIterator<Item = S>>(config: impl Into<VoiceConfig>, elements: I) -> Voice {
	Voice::new(config, elements)
}
