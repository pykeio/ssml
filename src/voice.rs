use std::fmt::Display;

use crate::{util, Element, Serialize, SerializeOptions, XmlWriter};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
	fn serialize_xml(&self, writer: &mut XmlWriter<'_>, _: &SerializeOptions) -> crate::Result<()> {
		writer.attr_opt("gender", self.gender.as_ref().map(|c| c.to_string()))?;
		writer.attr_opt("age", self.age.as_ref().map(|c| c.to_string()))?;
		writer.attr_opt("name", self.names.as_ref().map(|c| c.join(" ")))?;
		writer.attr_opt("variant", self.variant.as_ref())?;
		writer.attr_opt("language", self.languages.as_ref().map(|c| c.join(" ")))
	}
}

/// The [`Voice`] element allows you to specify a voice or use multiple different voices in one document.
#[derive(Clone, Default, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
	/// # fn main() -> ssml::Result<()> {
	/// let doc = ssml::speak(None, [ssml::voice("en-US-Neural2-F", ["Hello, world!"])]);
	///
	/// assert_eq!(
	/// 	doc.serialize_to_string(
	/// 		&ssml::SerializeOptions::default().flavor(ssml::Flavor::GoogleCloudTextToSpeech).pretty()
	/// 	)?,
	/// 	r#"<speak>
	/// 	<voice name="en-US-Neural2-F">
	/// 		Hello, world!
	/// 	</voice>
	/// </speak>"#
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
	/// # fn main() -> ssml::Result<()> {
	/// let mut voice = ssml::voice("en-US-Neural2-F", ["Hello, world!"]);
	/// voice.push("This is an SSML document.");
	/// let doc = ssml::speak(None, [voice]);
	///
	/// assert_eq!(
	/// 	doc.serialize_to_string(
	/// 		&ssml::SerializeOptions::default().flavor(ssml::Flavor::GoogleCloudTextToSpeech).pretty()
	/// 	)?,
	/// 	r#"<speak>
	/// 	<voice name="en-US-Neural2-F">
	/// 		Hello, world!
	/// 		This is an SSML document.
	/// 	</voice>
	/// </speak>"#
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
	/// # fn main() -> ssml::Result<()> {
	/// let mut voice = ssml::voice("en-US-Neural2-F", ["Hello, world!"]);
	/// voice.extend(["This is an SSML document."]);
	/// let doc = ssml::speak(None, [voice]);
	///
	/// assert_eq!(
	/// 	doc.serialize_to_string(
	/// 		&ssml::SerializeOptions::default().flavor(ssml::Flavor::GoogleCloudTextToSpeech).pretty()
	/// 	)?,
	/// 	r#"<speak>
	/// 	<voice name="en-US-Neural2-F">
	/// 		Hello, world!
	/// 		This is an SSML document.
	/// 	</voice>
	/// </speak>"#
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
	fn serialize_xml(&self, writer: &mut XmlWriter<'_>, options: &SerializeOptions) -> crate::Result<()> {
		writer.element("voice", |writer| {
			self.config.serialize_xml(writer, options)?;
			for attr in &self.attrs {
				writer.attr(&attr.0, &attr.1)?;
			}
			util::serialize_elements(writer, &self.children, options)
		})
	}
}

/// Creates a new `voice` element to change the voice of a section of spoken elements.
///
/// ```
/// # use ssml::{Flavor, Serialize};
/// # fn main() -> ssml::Result<()> {
/// let doc = ssml::speak(None, [ssml::voice("en-US-Neural2-F", ["Hello, world!"])]);
///
/// assert_eq!(
/// 	doc.serialize_to_string(
/// 		&ssml::SerializeOptions::default().flavor(ssml::Flavor::GoogleCloudTextToSpeech).pretty()
/// 	)?,
/// 	r#"<speak>
/// 	<voice name="en-US-Neural2-F">
/// 		Hello, world!
/// 	</voice>
/// </speak>"#
/// );
/// # Ok(())
/// # }
/// ```
pub fn voice<S: Into<Element>, I: IntoIterator<Item = S>>(config: impl Into<VoiceConfig>, elements: I) -> Voice {
	Voice::new(config, elements)
}
