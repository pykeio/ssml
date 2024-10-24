use std::fmt::Write;

use crate::{Element, Flavor, Serialize, SerializeOptions, XmlWriter, util};

/// A generic expression for use in [`Express`]. Contains the name of the expression and the intensity/degree (default
/// `1.0`).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Expression(String, f32);

macro_rules! define_expressions {
	($($(#[$outer:meta])* $x:ident => $e:expr),*) => {
		$(
			$(#[$outer])*
			///
			/// Some voices may not support this expression. See [the Azure docs][ms] for more information.
			///
			/// [ms]: https://learn.microsoft.com/en-us/azure/ai-services/speech-service/language-support?tabs=tts#voice-styles-and-roles
			pub struct $x;

			impl From<$x> for Expression {
				fn from(_: $x) -> Expression {
					Expression(String::from($e), 1.0)
				}
			}

			impl $x {
				/// Configure the intensity of this expression. You can specify a stronger or softer style to make the
				/// speech more expressive or subdued. The range of accepted values are: `0.01` to `2` inclusive. The
				/// default value is `1`, which means the predefined style intensity. The minimum unit is `0.01`, which
				/// results in a slight tendency for the target style. A value of `2` results in a doubling of the
				/// default style intensity.
				pub fn with_degree(&self, degree: f32) -> Expression {
					Expression(String::from($e), degree.clamp(0.01, 2.0))
				}
			}
		)*
	};
}

define_expressions! {
	/// Expresses an excited and high-energy tone for promoting a product or service.
	Advertisement => "advertisement_upbeat",
	/// Expresses a warm and affectionate tone, with higher pitch and vocal energy. The speaker is in a state of
	/// attracting the attention of the listener. The personality of the speaker is often endearing in nature.
	Affectionate => "affectionate",
	/// Expresses an angry and annoyed tone.
	Angry => "angry",
	/// Expresses a warm and relaxed tone for digital assistants.
	Assistant => "assistant",
	/// Expresses a cool, collected, and composed attitude when speaking. Tone, pitch, and prosody are more uniform
	/// compared to other types of speech.
	Calm => "calm",
	/// Expresses a casual and relaxed tone.
	Chat => "chat",
	/// Expresses a positive and happy tone.
	Cheerful => "cheerful",
	/// Expresses a friendly and helpful tone for customer support.
	CustomerService => "customerservice",
	/// Expresses a melancholic and despondent tone with lower pitch and energy.
	Depressed => "depressed",
	/// Expresses a disdainful and complaining tone. Speech of this emotion displays displeasure and contempt.
	Disgruntled => "disgruntled",
	/// Narrates documentaries in a relaxed, interested, and informative style suitable for dubbing documentaries,
	/// expert commentary, and similar content.
	NarrationDocumentary => "documentary-narration",
	/// Expresses an uncertain and hesitant tone when the speaker is feeling uncomfortable.
	Embarrassed => "embarrassed",
	/// Expresses a sense of caring and understanding.
	Empathetic => "empathetic",
	/// Expresses a tone of admiration when you desire something that someone else has.
	Envious => "envious",
	/// Expresses an upbeat and hopeful tone. It sounds like something great is happening and the speaker is happy about
	/// it.
	Excited => "excited",
	/// Expresses a scared and nervous tone, with higher pitch, higher vocal energy, and faster rate. The speaker is in
	/// a state of tension and unease.
	Fearful => "fearful",
	/// Expresses a pleasant, inviting, and warm tone. It sounds sincere and caring.
	Friendly => "friendly",
	/// Expresses a mild, polite, and pleasant tone, with lower pitch and vocal energy.
	Gentle => "gentle",
	/// Expresses a warm and yearning tone. It sounds like something good will happen to the speaker.
	Hopeful => "hopeful",
	/// Expresses emotions in a melodic and sentimental way.
	Lyrical => "lyrical",
	/// Expresses a professional, objective tone for content reading.
	NarrationProfessional => "narration-professional",
	/// Expresses a soothing and melodious tone for content reading.
	NarrationRelaxed => "narration-relaxed",
	/// Expresses a formal and professional tone for narrating news.
	Newscast => "newscast",
	/// Expresses a versatile and casual tone for general news delivery.
	NewscastCasual => "newscast-casual",
	/// Expresses a formal, confident, and authoritative tone for news delivery.
	NewscastFormal => "newscast-formal",
	/// Expresses an emotional and rhythmic tone while reading a poem.
	PoetryReading => "poetry-reading",
	/// Expresses a sorrowful tone.
	Sad => "sad",
	/// Expresses a strict and commanding tone. Speaker often sounds stiffer and much less relaxed with firm cadence.
	Serious => "serious",
	/// Expresses a tone that sounds as if the voice is distant or in another location and making an effort to be
	/// clearly heard.
	Shouting => "shouting",
	/// Expresses a relaxed and interested tone for broadcasting a sports event.
	SportsCommentary => "sports_commentary",
	/// Expresses an intensive and energetic tone for broadcasting exciting moments in a sports event.
	SportsCommentaryExcited => "sports_commentary_excited",
	/// Expresses a soft tone that's trying to make a quiet and gentle sound.
	Whispering => "whispering",
	/// Expresses a scared tone, with a faster pace and a shakier voice. It sounds like the speaker is in an unsteady
	/// and frantic status.
	Terrified => "terrified",
	/// Expresses a cold and indifferent tone.
	Unfriendly => "unfriendly"
}

/// Change the speaking style for part of an SSML document, in ACSS/MSTTS.
///
/// Not all neural voices support all expressions. See [the Azure docs][ms] for more information on which voices support
/// which expressions.
///
/// [ms]: https://learn.microsoft.com/en-us/azure/ai-services/speech-service/language-support?tabs=tts#voice-styles-and-roles
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Express {
	expression: Expression,
	children: Vec<Element>
}

impl Express {
	/// Creates a new [`Express`] section to modify the speaking style of a section of elements.
	///
	/// ```
	/// # use ssml::Serialize;
	/// use ssml::mstts;
	///
	/// # fn main() -> ssml::Result<()> {
	/// let doc = ssml::speak(
	/// 	Some("en-US"),
	/// 	[ssml::voice(
	/// 		"en-US-JaneNeural",
	/// 		[mstts::express(mstts::express::Cheerful.with_degree(0.5), ["Good morning!"])]
	/// 	)]
	/// );
	///
	/// assert_eq!(
	/// 	doc.serialize_to_string(&ssml::SerializeOptions::default().flavor(ssml::Flavor::MicrosoftAzureCognitiveSpeechServices).pretty())?,
	/// 	r#"<speak version="1.0" xmlns="http://www.w3.org/2001/10/synthesis" xml:lang="en-US" xmlns:mstts="http://www.w3.org/2001/mstts">
	/// 	<voice name="en-US-JaneNeural">
	/// 		<mstts:express-as style="cheerful" styledegree="0.5">
	/// 			Good morning!
	/// 		</mstts:express-as>
	/// 	</voice>
	/// </speak>"#
	/// );
	/// # Ok(())
	/// # }
	/// ```
	pub fn new<S: Into<Element>, I: IntoIterator<Item = S>>(expression: impl Into<Expression>, elements: I) -> Self {
		Self {
			expression: expression.into(),
			children: elements.into_iter().map(|f| f.into()).collect()
		}
	}

	/// Extend this `express-as` section with an additional element.
	pub fn push(&mut self, element: impl Into<Element>) {
		self.children.push(element.into());
	}

	/// Extend this `express-as` section with additional elements.
	pub fn extend<S: Into<Element>, I: IntoIterator<Item = S>>(&mut self, elements: I) {
		self.children.extend(elements.into_iter().map(|f| f.into()));
	}

	/// Returns a reference to the elements contained within this `voice` section.
	pub fn children(&self) -> &[Element] {
		&self.children
	}

	/// Returns a mutable reference to the elements contained within this `voice` section.
	pub fn children_mut(&mut self) -> &mut [Element] {
		&mut self.children
	}

	/// Converts this element into an [`Element`].
	pub fn into_el(self) -> Element {
		Element::FlavorMSTTS(super::Element::Express(self))
	}
}

impl From<Express> for crate::Element {
	fn from(value: Express) -> Self {
		value.into_el()
	}
}

impl Serialize for Express {
	fn serialize_xml<W: Write>(&self, writer: &mut XmlWriter<W>, options: &SerializeOptions) -> crate::Result<()> {
		if options.perform_checks && options.flavor != Flavor::MicrosoftAzureCognitiveSpeechServices {
			return Err(crate::error!("`mstts::Express` is only supported in ACSS/MSTTS"));
		}

		writer.element("mstts:express-as", |writer| {
			writer.attr("style", &self.expression.0)?;
			writer.attr("styledegree", self.expression.1)?;
			util::serialize_elements(writer, &self.children, options)
		})
	}
}

/// Creates a new [`Express`] section to modify the speaking style of a section of elements.
///
/// ```
/// # use ssml::{Flavor, Serialize};
/// use ssml::mstts;
///
/// # fn main() -> ssml::Result<()> {
/// let doc = ssml::speak(
/// 	Some("en-US"),
/// 	[ssml::voice(
/// 		"en-US-JaneNeural",
/// 		[mstts::express(mstts::express::Cheerful.with_degree(0.5), ["Good morning!"])]
/// 	)]
/// );
///
/// assert_eq!(
/// 	doc.serialize_to_string(&ssml::SerializeOptions::default().flavor(ssml::Flavor::MicrosoftAzureCognitiveSpeechServices).pretty())?,
/// 	r#"<speak version="1.0" xmlns="http://www.w3.org/2001/10/synthesis" xml:lang="en-US" xmlns:mstts="http://www.w3.org/2001/mstts">
/// 	<voice name="en-US-JaneNeural">
/// 		<mstts:express-as style="cheerful" styledegree="0.5">
/// 			Good morning!
/// 		</mstts:express-as>
/// 	</voice>
/// </speak>"#
/// );
/// # Ok(())
/// # }
/// ```
pub fn express<S: Into<Element>, I: IntoIterator<Item = S>>(expression: impl Into<Expression>, elements: I) -> Express {
	Express::new(expression, elements)
}
