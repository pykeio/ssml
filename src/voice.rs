use alloc::{borrow::Cow, string::ToString, vec, vec::Vec};
use core::fmt::{self, Display, Write};

use crate::{Element, Serialize, SerializeOptions, XmlWriter, util, xml::TrustedNoEscape};

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
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(match self {
			VoiceGender::Unspecified => "",
			VoiceGender::Neutral => "neutral",
			VoiceGender::Female => "female",
			VoiceGender::Male => "male"
		})
	}
}
impl TrustedNoEscape for VoiceGender {}

/// Configuration for the [`Voice`] element.
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VoiceConfig<'s> {
	pub gender: Option<VoiceGender>,
	pub age: Option<u8>,
	pub names: Option<Vec<Cow<'s, str>>>,
	pub variant: Option<Cow<'s, str>>,
	pub languages: Option<Vec<Cow<'s, str>>>
}

impl<'s> VoiceConfig<'s> {
	/// Creates a new [`VoiceConfig`] with the specified voice name and no other attributes.
	///
	/// ```
	/// let doc = ssml::VoiceConfig::named("en-US-JennyNeural");
	/// ```
	pub fn named(name: impl Into<Cow<'s, str>>) -> Self {
		Self {
			names: Some(vec![name.into()]),
			..VoiceConfig::default()
		}
	}

	pub fn to_owned(&self) -> VoiceConfig<'static> {
		self.clone().into_owned()
	}

	pub fn into_owned(self) -> VoiceConfig<'static> {
		VoiceConfig {
			gender: self.gender.clone(),
			age: self.age.clone(),
			names: self.names.map(|n| {
				n.into_iter()
					.map(|s| match s {
						Cow::Borrowed(b) => Cow::Owned(b.to_string()),
						Cow::Owned(b) => Cow::Owned(b)
					})
					.collect()
			}),
			variant: match self.variant {
				Some(Cow::Borrowed(b)) => Some(Cow::Owned(b.to_string())),
				Some(Cow::Owned(b)) => Some(Cow::Owned(b)),
				None => None
			},
			languages: self.languages.map(|n| {
				n.into_iter()
					.map(|s| match s {
						Cow::Borrowed(b) => Cow::Owned(b.to_string()),
						Cow::Owned(b) => Cow::Owned(b)
					})
					.collect()
			})
		}
	}
}

impl<'s, S: Into<Cow<'s, str>>> From<S> for VoiceConfig<'s> {
	fn from(value: S) -> Self {
		VoiceConfig::named(value)
	}
}

impl<'s> Serialize for VoiceConfig<'s> {
	fn serialize_xml<W: Write>(&self, writer: &mut XmlWriter<W>, _: &SerializeOptions) -> crate::Result<()> {
		writer.attr_opt("gender", self.gender.as_ref())?;
		writer.attr_opt("age", self.age.as_ref())?;
		writer.attr_opt("name", self.names.as_ref().map(|c| c.join(" ")))?;
		writer.attr_opt("variant", self.variant.as_deref())?;
		writer.attr_opt("language", self.languages.as_ref().map(|c| c.join(" ")))
	}
}

/// The [`Voice`] element allows you to specify a voice or use multiple different voices in one document.
#[derive(Clone, Default, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Voice<'s> {
	pub(crate) children: Vec<Element<'s>>,
	pub(crate) attrs: Vec<(Cow<'s, str>, Cow<'s, str>)>,
	config: VoiceConfig<'s>
}

impl<'s> Voice<'s> {
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
	pub fn new<S: Into<Element<'s>>, I: IntoIterator<Item = S>>(config: impl Into<VoiceConfig<'s>>, elements: I) -> Self {
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
	/// voice = voice.with_config(ssml::VoiceConfig { age: Some(42), ..Default::default() });
	/// ```
	pub fn with_config(mut self, config: impl Into<VoiceConfig<'s>>) -> Self {
		self.config = config.into();
		self
	}

	/// Returns the voice configuration used by this element.
	pub fn config(&self) -> &VoiceConfig {
		&self.config
	}

	pub fn set_config(&mut self, config: impl Into<VoiceConfig<'s>>) {
		self.config = config.into();
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
	pub fn push(&mut self, element: impl Into<Element<'s>>) {
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
	pub fn extend<S: Into<Element<'s>>, I: IntoIterator<Item = S>>(&mut self, elements: I) {
		self.children.extend(elements.into_iter().map(|f| f.into()));
	}

	/// Returns a reference to the elements contained within this `voice` section.
	pub fn children(&self) -> &[Element<'s>] {
		&self.children
	}

	/// Returns a mutable reference to the elements contained within this `voice` section.
	pub fn children_mut(&mut self) -> &mut Vec<Element<'s>> {
		&mut self.children
	}

	pub fn to_owned(&self) -> Voice<'static> {
		self.clone().into_owned()
	}

	pub fn into_owned(self) -> Voice<'static> {
		Voice {
			children: self.children.into_iter().map(Element::into_owned).collect(),
			attrs: self
				.attrs
				.into_iter()
				.map(|(k, v)| {
					(
						match k {
							Cow::Borrowed(b) => Cow::Owned(b.to_string()),
							Cow::Owned(b) => Cow::Owned(b)
						},
						match v {
							Cow::Borrowed(b) => Cow::Owned(b.to_string()),
							Cow::Owned(b) => Cow::Owned(b)
						}
					)
				})
				.collect(),
			config: self.config.into_owned()
		}
	}
}

impl<'s> Serialize for Voice<'s> {
	fn serialize_xml<W: Write>(&self, writer: &mut XmlWriter<W>, options: &SerializeOptions) -> crate::Result<()> {
		writer.element("voice", |writer| {
			self.config.serialize_xml(writer, options)?;
			for attr in &self.attrs {
				writer.attr(&attr.0, &*attr.1)?;
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
pub fn voice<'s, S: Into<Element<'s>>, I: IntoIterator<Item = S>>(config: impl Into<VoiceConfig<'s>>, elements: I) -> Voice<'s> {
	Voice::new(config, elements)
}
