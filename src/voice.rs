use std::{fmt::Display, io::Write};

use crate::{util, Element, Flavor, Serialize};

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
	pub names: Option<Vec<String>>,
	pub variant: Option<String>,
	pub languages: Option<Vec<String>>
}

impl VoiceConfig {
	/// Creates a new [`VoiceConfig`] with the specified voice name and no other attributes.
	///
	/// ```
	/// let doc = ssml::VoiceConfig::named("en-US-JennyNeural");
	/// ```
	pub fn named(name: impl ToString) -> Self {
		Self {
			names: Some(vec![name.to_string()]),
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
		if let Some(names) = &self.names {
			util::write_attr(writer, "name", names.join(" "))?;
		}
		if let Some(variant) = &self.variant {
			util::write_attr(writer, "variant", variant)?;
		}
		if let Some(languages) = &self.languages {
			util::write_attr(writer, "language", languages.join(" "))?;
		}
		Ok(())
	}
}

/// The [`Voice`] element allows you to specify a voice or use multiple different voices in one document.
#[derive(Clone, Default, Debug)]
pub struct Voice {
	pub(crate) children: Vec<Element>,
	pub(crate) attrs: Vec<(String, String)>,
	config: VoiceConfig
}

impl Voice {
	/// Creates a new `voice` element to change the voice of a section of spoken elements.
	///
	/// ```
	/// # use ssml::{Flavor, Serialize};
	/// # fn main() -> anyhow::Result<()> {
	/// let doc = ssml::speak(None, [ssml::voice("en-US-Neural2-F", ["Hello, world!"])]);
	///
	/// assert_eq!(
	/// 	doc.serialize_to_string(Flavor::GoogleCloudTextToSpeech)?,
	/// 	r#"<speak><voice name="en-US-Neural2-F">Hello, world!</voice></speak>"#
	/// );
	/// # Ok(())
	/// # }
	/// ```
	pub fn new<S: Into<Element>, I: IntoIterator<Item = S>>(config: impl Into<VoiceConfig>, elements: I) -> Self {
		Self {
			children: elements.into_iter().map(|f| f.into()).collect(),
			attrs: vec![],
			config: config.into()
		}
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

	/// Extend this `voice` section with an additional element.
	///
	/// ```
	/// # use ssml::{Flavor, Serialize};
	/// # fn main() -> anyhow::Result<()> {
	/// let mut voice = ssml::voice("en-US-Neural2-F", ["Hello, world!"]);
	/// voice.push("This is an SSML document.");
	/// let doc = ssml::speak(None, [voice]);
	///
	/// assert_eq!(
	/// 	doc.serialize_to_string(Flavor::GoogleCloudTextToSpeech)?,
	/// 	r#"<speak><voice name="en-US-Neural2-F">Hello, world! This is an SSML document.</voice></speak>"#
	/// );
	/// # Ok(())
	/// # }
	/// ```
	pub fn push(&mut self, element: impl Into<Element>) {
		self.children.push(element.into());
	}

	/// Extend this `voice` section with additional elements.
	///
	/// ```
	/// # use ssml::{Flavor, Serialize};
	/// # fn main() -> anyhow::Result<()> {
	/// let mut voice = ssml::voice("en-US-Neural2-F", ["Hello, world!"]);
	/// voice.extend(["This is an SSML document."]);
	/// let doc = ssml::speak(None, [voice]);
	///
	/// assert_eq!(
	/// 	doc.serialize_to_string(Flavor::GoogleCloudTextToSpeech)?,
	/// 	r#"<speak><voice name="en-US-Neural2-F">Hello, world! This is an SSML document.</voice></speak>"#
	/// );
	/// # Ok(())
	/// # }
	/// ```
	pub fn extend<S: Into<Element>, I: IntoIterator<Item = S>>(&mut self, elements: I) {
		self.children.extend(elements.into_iter().map(|f| f.into()));
	}

	/// Returns the voice configuration used by this element.
	pub fn config(&self) -> &VoiceConfig {
		&self.config
	}

	/// Returns a reference to the elements contained within this `voice` section.
	pub fn children(&self) -> &[Element] {
		&self.children
	}

	/// Returns a mutable reference to the elements contained within this `voice` section.
	pub fn children_mut(&mut self) -> &mut [Element] {
		&mut self.children
	}
}

impl Serialize for Voice {
	fn serialize<W: Write>(&self, writer: &mut W, flavor: Flavor) -> anyhow::Result<()> {
		writer.write_all(b"<voice")?;
		self.config.serialize(writer, flavor)?;
		for attr in &self.attrs {
			util::write_attr(writer, &attr.0, &attr.1)?;
		}
		writer.write_all(b">")?;
		util::serialize_elements(writer, &self.children, flavor)?;
		writer.write_all(b"</voice>")?;
		Ok(())
	}
}

/// Creates a new `voice` element to change the voice of a section of spoken elements.
///
/// ```
/// # use ssml::{Flavor, Serialize};
/// # fn main() -> anyhow::Result<()> {
/// let doc = ssml::speak(None, [ssml::voice("en-US-Neural2-F", ["Hello, world!"])]);
///
/// assert_eq!(
/// 	doc.serialize_to_string(Flavor::GoogleCloudTextToSpeech)?,
/// 	r#"<speak><voice name="en-US-Neural2-F">Hello, world!</voice></speak>"#
/// );
/// # Ok(())
/// # }
/// ```
pub fn voice<S: Into<Element>, I: IntoIterator<Item = S>>(config: impl Into<VoiceConfig>, elements: I) -> Voice {
	Voice::new(config, elements)
}
