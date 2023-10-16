use std::fmt::Display;

use crate::{voice::Voice, Meta, SpeakableElement};

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

pub trait MicrosoftVoiceExt {
	/// For ACSS, configures a [`Voice`] section to send back viseme animations in the specified format.
	///
	/// ```
	/// # use ssml::{Flavor, mstts::{MicrosoftVoiceExt, MicrosoftViseme}, Serialize};
	/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
	/// let doc = ssml::Speak::new(
	/// 	Some("en-US"),
	/// 	[ssml::Voice::new(
	/// 		"en-US-JennyNeural",
	/// 		["Rainbow has seven colors: Red, orange, yellow, green, blue, indigo, and violet."]
	/// 	)
	/// 	.with_mstts_viseme(MicrosoftViseme::FacialExpression)]
	/// );
	///
	/// assert_eq!(
	/// 	doc.serialize_to_string(Flavor::MicrosoftAzureCognitiveSpeechServices)?,
	/// 	r#"<speak version="1.0" xmlns="http://www.w3.org/2001/10/synthesis" xml:lang="en-US" xmlns:mstts="http://www.w3.org/2001/mstts"><voice name="en-US-JennyNeural"><mstts:viseme type="FacialExpression" />Rainbow has seven colors: Red, orange, yellow, green, blue, indigo, and violet. </voice></speak>"#
	/// );
	/// # Ok(())
	/// # }
	/// ```
	fn with_mstts_viseme(self, config: MicrosoftViseme) -> Self;
}

impl MicrosoftVoiceExt for Voice {
	fn with_mstts_viseme(mut self, config: MicrosoftViseme) -> Self {
		self.elements
			.insert(0, SpeakableElement::Meta(Meta(format!("<mstts:viseme type=\"{config}\" />"))));
		self
	}
}
