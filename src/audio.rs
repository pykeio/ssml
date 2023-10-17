use std::io::Write;

use crate::{
	speak::SpeakableElement,
	unit::{Decibels, TimeDesignation},
	util, Flavor, Serialize
};

/// Specify repeating an [`Audio`] element's playback for a certain number of times, or for a determined duration.
#[derive(Debug, Clone, PartialEq)]
pub enum AudioRepeat {
	/// Repeat the audio a certain number of times. A fractional value is allowed and describes a portion of the
	/// rendered media.
	///
	/// The value **cannot** be negative. Negative values will throw an error upon serialization.
	Times(f32),
	/// Repeat the audio for a certain duration.
	Duration(TimeDesignation)
}

/// An SSML `<audio />` element. [`Audio`] supports the insertion of recorded audio files and the insertion of other
/// audio formats in conjunction with synthesized speech output.
#[derive(Debug, Default, Clone)]
pub struct Audio {
	src: String,
	desc: Option<String>,
	alternate: Vec<SpeakableElement>,
	clip: (Option<TimeDesignation>, Option<TimeDesignation>),
	repeat: Option<AudioRepeat>,
	sound_level: Option<Decibels>,
	speed: Option<f32>
}

impl Audio {
	/// Creates a new [`Audio`] element with an audio source URI.
	///
	/// ```
	/// ssml::Audio::new("https://example.com/Congratulations_You_Won.wav");
	/// ```
	pub fn new(src: impl ToString) -> Self {
		Audio {
			src: src.to_string(),
			..Audio::default()
		}
	}

	/// Appends alternate (fallback) elements. Alternate elements will be spoken or displayed if the audio document
	/// located at the specified URI is unavailable for whatever reason.
	///
	/// See also [`Audio::with_desc`] to provide an accessible description for this audio element.
	///
	/// ```
	/// # use ssml::Audio;
	/// Audio::new("cat_purr.ogg").with_alternate(["PURR (sound didn't load)"]);
	/// ```
	pub fn with_alternate<S: Into<SpeakableElement>, I: IntoIterator<Item = S>>(mut self, elements: I) -> Self {
		self.alternate.extend(elements.into_iter().map(|f| f.into()));
		self
	}

	/// Sets an accessible description for this audio element.
	///
	/// ```
	/// # use ssml::Audio;
	/// Audio::new("cat_purr.ogg").with_desc("a purring cat");
	/// ```
	pub fn with_desc(mut self, desc: impl ToString) -> Self {
		self.desc = Some(desc.to_string());
		self
	}

	/// Specify an offset from the beginning and to the end of which to clip this audio's duration to.
	///
	/// ```
	/// # use ssml::Audio;
	/// // Play the sound starting from 0.25s, and stop at 0.75s, for a total duration of 0.5s.
	/// Audio::new("cat_purr.ogg").with_clip("0.25s", "750ms");
	/// ```
	pub fn with_clip(mut self, begin: impl Into<TimeDesignation>, end: impl Into<TimeDesignation>) -> Self {
		self.clip = (Some(begin.into()), Some(end.into()));
		self
	}

	/// Specify an offset from the beginning of the audio to start playback.
	///
	/// ```
	/// # use ssml::Audio;
	/// // maybe skip some silence at the beginning
	/// Audio::new("cat_purr.ogg").with_clip_begin("0.15s");
	/// ```
	pub fn with_clip_begin(mut self, begin: impl Into<TimeDesignation>) -> Self {
		self.clip.0 = Some(begin.into());
		self
	}

	/// Specify an offset from the beginning of the audio to end playback.
	///
	/// ```
	/// # use ssml::Audio;
	/// // maybe skip some silence at the end
	/// Audio::new("cat_purr.ogg").with_clip_begin("0.75s");
	/// ```
	pub fn with_clip_end(mut self, end: impl Into<TimeDesignation>) -> Self {
		self.clip.1 = Some(end.into());
		self
	}

	/// Repeat this audio source for a set amount of times, or for a set duration. See [`AudioRepeat`].
	///
	/// ```
	/// # use ssml::{Audio, AudioRepeat};
	/// // Play the beep sound effect 3 times
	/// Audio::new("beep.ogg").with_repeat(AudioRepeat::Times(3.0));
	/// // Happy kitty!
	/// Audio::new("cat_purr.ogg").with_repeat(AudioRepeat::Duration("30s".into()));
	/// ```
	pub fn with_repeat(mut self, repeat: AudioRepeat) -> Self {
		self.repeat = Some(repeat);
		self
	}

	/// Specify the relative volume of the referenced audio, in decibels. Setting to a large negative value like
	/// `-100dB` will effectively silence the audio clip. A value of `-6.0dB` will play the audio at approximately half
	/// the volume, and likewise `+6.0dB` will play the audio at twice the volume.
	///
	/// ```
	/// # use ssml::Audio;
	/// Audio::new("cat_meow.ogg").with_sound_level("+6.0dB");
	/// ```
	pub fn with_sound_level(mut self, db: impl Into<Decibels>) -> Self {
		self.sound_level = Some(db.into());
		self
	}

	/// Specify the speed at which to play the audio clip (where `1.0` is normal speed).
	///
	/// ```
	/// # use ssml::{Audio, AudioRepeat};
	/// // panic beeping at 2x speed
	/// Audio::new("beep.ogg").with_repeat(AudioRepeat::Times(12.0)).with_speed(2.0);
	/// ```
	pub fn with_speed(mut self, speed: f32) -> Self {
		self.speed = Some(speed);
		self
	}
}

impl Serialize for Audio {
	fn serialize<W: Write>(&self, writer: &mut W, flavor: Flavor) -> anyhow::Result<()> {
		writer.write_all(b"<audio")?;
		if !self.src.is_empty() {
			util::write_attr(writer, "src", &self.src)?;
		} else if flavor == Flavor::GoogleCloudTextToSpeech {
			// https://cloud.google.com/text-to-speech/docs/ssml#attributes_1
			return Err(crate::error!("GCTTS requires <audio> elements to have a valid `src`."))?;
		}

		if let Some(clip_begin) = &self.clip.0 {
			util::write_attr(writer, "clipBegin", clip_begin.to_string())?;
		}
		if let Some(clip_end) = &self.clip.1 {
			util::write_attr(writer, "clipEnd", clip_end.to_string())?;
		}

		if let Some(repeat) = &self.repeat {
			match repeat {
				AudioRepeat::Duration(dur) => util::write_attr(writer, "repeatDur", dur.to_string())?,
				AudioRepeat::Times(times) => {
					if times.is_sign_negative() {
						return Err(crate::error!("`times` cannot be negative"))?;
					}
					util::write_attr(writer, "times", times.to_string())?;
				}
			}
		}

		if let Some(sound_level) = &self.sound_level {
			util::write_attr(writer, "soundLevel", sound_level.to_string())?;
		}

		if let Some(speed) = &self.speed {
			if speed.is_sign_negative() {
				return Err(crate::error!("`speed` cannot be negative"))?;
			}
			util::write_attr(writer, "speed", format!("{}%", speed * 100.))?;
		}

		writer.write_all(b">")?;
		if let Some(desc) = &self.desc {
			writer.write_fmt(format_args!("<desc>{}</desc>", util::escape_xml(desc)))?;
		}
		for el in &self.alternate {
			el.serialize(writer, flavor)?;
		}
		writer.write_all(b"</audio>")?;

		Ok(())
	}
}

/// Creates a new [`Audio`] element with an audio source URI.
///
/// ```
/// ssml::audio("https://example.com/Congratulations_You_Won.wav");
/// ```
pub fn audio(src: impl ToString) -> Audio {
	Audio::new(src)
}

#[cfg(test)]
mod tests {
	use super::{Audio, AudioRepeat};
	use crate::{Flavor, Serialize};

	#[test]
	fn non_negative_speed() {
		assert!(Audio::default().with_speed(-1.0).serialize_to_string(Flavor::Generic).is_err());
	}

	#[test]
	fn non_negative_repeat_times() {
		assert!(
			Audio::default()
				.with_repeat(AudioRepeat::Times(-1.0))
				.serialize_to_string(Flavor::Generic)
				.is_err()
		);
	}
}
