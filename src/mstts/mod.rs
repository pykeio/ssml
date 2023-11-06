//! Elements exclusive to [`Flavor::MicrosoftAzureCognitiveSpeechServices`] (ACSS/MSTTS).

use std::fmt::Display;

use crate::{voice::Voice, Flavor, Meta};

pub mod express;
pub use self::express::{express, Express};

/// Viseme configuration for MSTTS.
///
/// See [`MicrosoftVoiceExt::with_mstts_viseme`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MicrosoftViseme {
	/// Receive visemes as an ID. (equivalent to `<mstts:viseme type="redlips_front" />`)
	ById,
	/// Receive visemes as blend shapes.
	FacialExpression
}

impl Display for MicrosoftViseme {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			MicrosoftViseme::ById => "redlips_front",
			MicrosoftViseme::FacialExpression => "FacialExpression"
		})
	}
}

/// A voice effect to apply to speech.
///
/// For some scenarios in production environments, the auditory experience might be degraded due to the playback
/// distortion on certain devices. For example, the synthesized speech from a car speaker might sound dull and
/// muffled due to environmental factors such as speaker response, room reverberation, and background noise. The
/// passenger might have to turn up the volume to hear more clearly. To avoid manual operations in such a scenario,
/// the audio effect processor can make the sound clearer by compensating the distortion of playback.
///
/// See [`MicrosoftVoiceExt::with_mstts_viseme`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MicrosoftVoiceEffect {
	/// Optimize the auditory experience when providing high-fidelity speech in cars, buses, and other enclosed
	/// automobiles.
	Automobile,
	/// Optimize the auditory experience for narrowband speech in telecom or telephone scenarios. You should use a
	/// sampling rate of 8 kHz. If the sample rate isn't 8 kHz, the auditory quality of the output speech isn't
	/// optimized.
	Telecom
}

impl Display for MicrosoftVoiceEffect {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			MicrosoftVoiceEffect::Automobile => "eq_car",
			MicrosoftVoiceEffect::Telecom => "eq_telecomhp8k"
		})
	}
}

/// Extensions for [`Voice`] specific to MSTTS, aka Azure Cognitive Speech Services or ACSS.
pub trait MicrosoftVoiceExt {
	/// For ACSS, configures a [`Voice`] section to send back viseme animations in the specified format.
	///
	/// ```
	/// # use ssml::{Flavor, mstts::{MicrosoftVoiceExt, MicrosoftViseme}, Serialize};
	/// # fn main() -> ssml::Result<()> {
	/// let doc = ssml::speak(
	/// 	Some("en-US"),
	/// 	[ssml::voice(
	/// 		"en-US-JennyNeural",
	/// 		["A rainbow has seven colors: Red, orange, yellow, green, blue, indigo, and violet."]
	/// 	)
	/// 	.with_mstts_viseme(MicrosoftViseme::FacialExpression)]
	/// );
	///
	/// assert_eq!(
	/// 	doc.serialize_to_string(&ssml::SerializeOptions::default().flavor(ssml::Flavor::MicrosoftAzureCognitiveSpeechServices).pretty())?,
	/// 	r#"<speak version="1.0" xmlns="http://www.w3.org/2001/10/synthesis" xml:lang="en-US" xmlns:mstts="http://www.w3.org/2001/mstts">
	/// 	<voice name="en-US-JennyNeural">
	/// 		<mstts:viseme type="FacialExpression" />
	/// 		A rainbow has seven colors: Red, orange, yellow, green, blue, indigo, and violet.
	/// 	</voice>
	/// </speak>"#
	/// );
	/// # Ok(())
	/// # }
	/// ```
	fn with_mstts_viseme(self, config: MicrosoftViseme) -> Self;

	/// For ACSS, configures a [`Voice`] section to have a certain effect applied to optimize the quality of synthesized
	/// speech output for specific scenarios on devices.
	///
	/// For some scenarios in production environments, the auditory experience might be degraded due to the playback
	/// distortion on certain devices. For example, the synthesized speech from a car speaker might sound dull and
	/// muffled due to environmental factors such as speaker response, room reverberation, and background noise. The
	/// passenger might have to turn up the volume to hear more clearly. To avoid manual operations in such a scenario,
	/// the audio effect processor can make the sound clearer by compensating the distortion of playback.
	///
	/// ```
	/// # use ssml::{Flavor, mstts::{MicrosoftVoiceExt, MicrosoftVoiceEffect}, Serialize};
	/// # fn main() -> ssml::Result<()> {
	/// let doc = ssml::speak(
	/// 	Some("en-US"),
	/// 	[ssml::voice(
	/// 		"en-US-JennyNeural",
	/// 		["Your call is being transferred to a service representative."]
	/// 	)
	/// 	.with_mstts_effect(MicrosoftVoiceEffect::Telecom)]
	/// );
	///
	/// assert_eq!(
	/// 	doc.serialize_to_string(&ssml::SerializeOptions::default().flavor(ssml::Flavor::MicrosoftAzureCognitiveSpeechServices).pretty())?,
	/// 	r#"<speak version="1.0" xmlns="http://www.w3.org/2001/10/synthesis" xml:lang="en-US" xmlns:mstts="http://www.w3.org/2001/mstts">
	/// 	<voice name="en-US-JennyNeural" effect="eq_telecomhp8k">
	/// 		Your call is being transferred to a service representative.
	/// 	</voice>
	/// </speak>"#
	/// );
	/// # Ok(())
	/// # }
	/// ```
	fn with_mstts_effect(self, effect: MicrosoftVoiceEffect) -> Self;
}

impl MicrosoftVoiceExt for Voice {
	fn with_mstts_viseme(mut self, config: MicrosoftViseme) -> Self {
		self.children.insert(
			0,
			Meta::new(format!("<mstts:viseme type=\"{config}\" />"))
				.with_name("MicrosoftViseme")
				.with_restrict_flavor([Flavor::MicrosoftAzureCognitiveSpeechServices])
				.into()
		);
		self
	}

	fn with_mstts_effect(mut self, effect: MicrosoftVoiceEffect) -> Self {
		self.attrs.push(("effect".to_string(), effect.to_string()));
		self
	}
}
