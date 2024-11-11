use alloc::{borrow::Cow, string::ToString, vec::Vec};
use core::fmt::{self, Display, Write};

use crate::{
	Element, Serialize, SerializeOptions, XmlWriter,
	unit::{Decibels, TimeDesignation},
	util,
	xml::TrustedNoEscape
};

/// Specify repeating an [`Audio`] element's playback for a certain number of times, or for a determined duration.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AudioRepeat {
	/// Repeat the audio a certain number of times. A fractional value is allowed and describes a portion of the
	/// rendered media.
	///
	/// The value **cannot** be negative. Negative values will throw an error upon serialization.
	Times(f32),
	/// Repeat the audio for a certain duration.
	Duration(TimeDesignation)
}

/// [`Audio`] supports the insertion of recorded audio files and the insertion of other audio formats in conjunction
/// with synthesized speech output.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Audio<'s> {
	src: Cow<'s, str>,
	desc: Option<Cow<'s, str>>,
	alternate: Vec<Element<'s>>,
	clip: (Option<TimeDesignation>, Option<TimeDesignation>),
	repeat: Option<AudioRepeat>,
	sound_level: Option<Decibels>,
	speed: Option<f32>
}

impl<'s> Audio<'s> {
	/// Creates a new [`Audio`] element with an audio source URI.
	///
	/// ```
	/// ssml::audio("https://example.com/Congratulations_You_Won.wav");
	/// ```
	pub fn new(src: impl Into<Cow<'s, str>>) -> Self {
		Audio { src: src.into(), ..Audio::default() }
	}

	pub fn src(&self) -> &str {
		&self.src
	}

	pub fn set_src(&mut self, src: impl Into<Cow<'s, str>>) {
		self.src = src.into();
	}

	/// Appends alternate (fallback) elements. Alternate elements will be spoken or displayed if the audio document
	/// located at the specified URI is unavailable for whatever reason.
	///
	/// See also [`Audio::with_desc`] to provide an accessible description for this audio element.
	///
	/// ```
	/// ssml::audio("cat_purr.ogg").with_alternate(["PURR (sound didn't load)"]);
	/// ```
	pub fn with_alternate<S: Into<Element<'s>>, I: IntoIterator<Item = S>>(mut self, elements: I) -> Self {
		self.alternate.extend(elements.into_iter().map(|f| f.into()));
		self
	}

	/// Sets an accessible description for this audio element.
	///
	/// ```
	/// ssml::audio("cat_purr.ogg").with_desc("a purring cat");
	/// ```
	pub fn with_desc(mut self, desc: impl Into<Cow<'s, str>>) -> Self {
		self.desc = Some(desc.into());
		self
	}

	pub fn desc(&self) -> Option<&str> {
		self.desc.as_deref()
	}

	pub fn set_desc(&mut self, desc: impl Into<Cow<'s, str>>) {
		self.desc = Some(desc.into());
	}

	pub fn take_desc(&mut self) -> Option<Cow<'s, str>> {
		self.desc.take()
	}

	/// Specify an offset from the beginning and to the end of which to clip this audio's duration to.
	///
	/// ```
	/// // Play the sound starting from 0.25s, and stop at 0.75s, for a total duration of 0.5s.
	/// ssml::audio("cat_purr.ogg").with_clip("0.25s", "750ms");
	/// ```
	pub fn with_clip(mut self, begin: impl Into<TimeDesignation>, end: impl Into<TimeDesignation>) -> Self {
		self.clip = (Some(begin.into()), Some(end.into()));
		self
	}

	/// Specify an offset from the beginning of the audio to start playback.
	///
	/// ```
	/// // maybe skip some silence at the beginning
	/// ssml::audio("cat_purr.ogg").with_clip_begin("0.15s");
	/// ```
	pub fn with_clip_begin(mut self, begin: impl Into<TimeDesignation>) -> Self {
		self.clip.0 = Some(begin.into());
		self
	}

	pub fn clip_begin(&self) -> Option<&TimeDesignation> {
		self.clip.0.as_ref()
	}

	pub fn set_clip_begin(&mut self, begin: impl Into<TimeDesignation>) {
		self.clip.0 = Some(begin.into());
	}

	pub fn take_clip_begin(&mut self) -> Option<TimeDesignation> {
		self.clip.0.take()
	}

	/// Specify an offset from the beginning of the audio to end playback.
	///
	/// ```
	/// // maybe skip some silence at the end
	/// ssml::audio("cat_purr.ogg").with_clip_begin("0.75s");
	/// ```
	pub fn with_clip_end(mut self, end: impl Into<TimeDesignation>) -> Self {
		self.clip.1 = Some(end.into());
		self
	}

	pub fn clip_end(&self) -> Option<&TimeDesignation> {
		self.clip.1.as_ref()
	}

	pub fn set_clip_end(&mut self, end: impl Into<TimeDesignation>) {
		self.clip.1 = Some(end.into());
	}

	pub fn take_clip_end(&mut self) -> Option<TimeDesignation> {
		self.clip.1.take()
	}

	/// Repeat this audio source for a set amount of times, or for a set duration. See [`AudioRepeat`].
	///
	/// ```
	/// // Play the beep sound effect 3 times
	/// ssml::audio("beep.ogg").with_repeat(ssml::AudioRepeat::Times(3.0));
	/// // Happy kitty!
	/// ssml::audio("cat_purr.ogg").with_repeat(ssml::AudioRepeat::Duration("30s".into()));
	/// ```
	pub fn with_repeat(mut self, repeat: AudioRepeat) -> Self {
		self.repeat = Some(repeat);
		self
	}

	pub fn repeat(&self) -> Option<&AudioRepeat> {
		self.repeat.as_ref()
	}

	pub fn set_repeat(&mut self, repeat: AudioRepeat) {
		self.repeat = Some(repeat);
	}

	pub fn take_repeat(&mut self) -> Option<AudioRepeat> {
		self.repeat.take()
	}

	/// Specify the relative volume of the referenced audio, in decibels. Setting to a large negative value like
	/// `-100dB` will effectively silence the audio clip. A value of `-6.0dB` will play the audio at approximately half
	/// the volume, and likewise `+6.0dB` will play the audio at twice the volume.
	///
	/// ```
	/// ssml::audio("cat_meow.ogg").with_sound_level("+6.0dB");
	/// ```
	pub fn with_sound_level(mut self, db: impl Into<Decibels>) -> Self {
		self.sound_level = Some(db.into());
		self
	}

	pub fn sound_level(&self) -> Option<&Decibels> {
		self.sound_level.as_ref()
	}

	pub fn set_sound_level(&mut self, db: impl Into<Decibels>) {
		self.sound_level = Some(db.into());
	}

	pub fn take_sound_level(&mut self) -> Option<Decibels> {
		self.sound_level.take()
	}

	/// Specify the speed at which to play the audio clip (where `1.0` is normal speed).
	///
	/// ```
	/// // panic beeping at 2x speed
	/// ssml::audio("beep.ogg").with_repeat(ssml::AudioRepeat::Times(12.0)).with_speed(2.0);
	/// ```
	pub fn with_speed(mut self, speed: f32) -> Self {
		self.speed = Some(speed);
		self
	}

	pub fn speed(&self) -> Option<f32> {
		self.speed
	}

	pub fn set_speed(&mut self, speed: f32) {
		self.speed = Some(speed.into());
	}

	pub fn take_speed(&mut self) -> Option<f32> {
		self.speed.take()
	}

	/// Returns a reference to the elements contained in this `audio` element's alternate/fallback section.
	pub fn alternate(&self) -> &[Element<'s>] {
		&self.alternate
	}

	/// Returns a reference to the elements contained in this `audio` element's alternate/fallback section.
	pub fn alternate_mut(&mut self) -> &mut Vec<Element<'s>> {
		&mut self.alternate
	}

	pub fn to_owned(&self) -> Audio<'static> {
		self.clone().into_owned()
	}

	pub fn into_owned(self) -> Audio<'static> {
		Audio {
			src: match self.src {
				Cow::Borrowed(b) => Cow::Owned(b.to_string()),
				Cow::Owned(b) => Cow::Owned(b)
			},
			desc: match self.desc {
				Some(Cow::Borrowed(b)) => Some(Cow::Owned(b.to_string())),
				Some(Cow::Owned(b)) => Some(Cow::Owned(b)),
				None => None
			},
			alternate: self.alternate.into_iter().map(Element::into_owned).collect(),
			clip: self.clip,
			repeat: self.repeat,
			sound_level: self.sound_level,
			speed: self.speed
		}
	}
}

impl<'s> Serialize for Audio<'s> {
	fn serialize_xml<W: Write>(&self, writer: &mut XmlWriter<W>, options: &SerializeOptions) -> crate::Result<()> {
		writer.element("audio", |writer| {
			writer.attr("src", &*self.src)?;

			writer.attr_opt("clipBegin", self.clip.0.as_ref())?;
			writer.attr_opt("clipEnd", self.clip.1.as_ref())?;

			if let Some(repeat) = &self.repeat {
				match repeat {
					AudioRepeat::Duration(dur) => writer.attr("repeatDur", dur)?,
					AudioRepeat::Times(times) => writer.attr("times", times)?
				}
			}

			writer.attr_opt("soundLevel", self.sound_level.as_ref().map(|t| t))?;
			writer.attr_opt("speed", self.speed.map(|s| SpeedFormatter(s)))?;

			if let Some(desc) = &self.desc {
				writer.element("desc", |writer| writer.text(desc))?;
			}

			util::serialize_elements(writer, &self.alternate, options)?;

			Ok(())
		})?;
		Ok(())
	}
}

/// Creates a new [`Audio`] element with an audio source URI.
///
/// ```
/// ssml::audio("https://example.com/Congratulations_You_Won.wav");
/// ```
pub fn audio<'s>(src: impl Into<Cow<'s, str>>) -> Audio<'s> {
	Audio::new(src)
}

struct SpeedFormatter(f32);
impl Display for SpeedFormatter {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_fmt(format_args!("{}%", self.0 * 100.))
	}
}
impl TrustedNoEscape for SpeedFormatter {}
