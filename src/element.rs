use std::fmt::Debug;

use dyn_clone::DynClone;

use crate::{Audio, Meta, Serialize, SerializeOptions, Text, Voice, XmlWriter};

macro_rules! el {
	(
		$(#[$outer:meta])*
		pub enum $name:ident {
			$(
				$(#[$innermeta:meta])*
				$variant:ident($inner:ty)
			),*
		}
	) => {
		$(#[$outer])*
		pub enum $name {
			$(
				$(#[$innermeta])*
				$variant($inner)
			),*
		}

		$(impl From<$inner> for $name {
			fn from(val: $inner) -> $name {
				$name::$variant(val)
			}
		})*

		impl $crate::Serialize for $name {
			fn serialize_xml(&self, writer: &mut $crate::XmlWriter<'_>, options: &$crate::SerializeOptions) -> crate::Result<()> {
				match self {
					$($name::$variant(inner) => inner.serialize_xml(writer, options),)*
				}
			}
		}
	};
}

el! {
	/// Represents all SSML elements.
	#[derive(Clone, Debug)]
	#[non_exhaustive]
	pub enum Element {
		Text(Text),
		Audio(Audio),
		Voice(Voice),
		Meta(Meta),
		/// A dyn element can be used to implement your own custom elements outside of the `ssml` crate. See
		/// [`DynElement`] for more information and examples.
		Dyn(Box<dyn DynElement>)
		// Break(BreakElement),
		// Emphasis(EmphasisElement),
		// Lang(LangElement),
		// Mark(MarkElement),
		// Paragraph(ParagraphElement),
		// Phoneme(PhonemeElement),
		// Prosody(ProsodyElement),
		// SayAs(SayAsElement),
		// Sub(SubElement),
		// Sentence(SentenceElement),
		// Voice(VoiceElement),
		// Word(WordElement)
	}
}

impl<T: ToString> From<T> for Element {
	fn from(value: T) -> Self {
		Element::Text(Text(value.to_string()))
	}
}

/// A dynamic element which can be used to implement non-standard SSML elements outside of the `ssml` crate.
///
/// ```
/// use ssml::{DynElement, Element, Serialize, SerializeOptions, XmlWriter};
///
/// #[derive(Debug, Clone)]
/// pub struct TomfooleryElement {
/// 	value: f32,
/// 	children: Vec<Element>
/// }
///
/// impl TomfooleryElement {
/// 	// Increase the tomfoolery level of a section of elements.
/// 	// ...
/// 	pub fn new<S: Into<Element>, I: IntoIterator<Item = S>>(value: f32, elements: I) -> Self {
/// 		Self {
/// 			value,
/// 			children: elements.into_iter().map(|f| f.into()).collect()
/// 		}
/// 	}
///
/// 	// not required, but makes your code much cleaner!
/// 	pub fn into_dyn(self) -> Element {
/// 		Element::Dyn(Box::new(self))
/// 	}
/// }
///
/// impl DynElement for TomfooleryElement {
/// 	fn serialize_xml(&self, writer: &mut XmlWriter<'_>, options: &SerializeOptions) -> ssml::Result<()> {
/// 		writer.element("tomfoolery", |writer| {
/// 			writer.attr("influence", self.value.to_string())?;
/// 			ssml::util::serialize_elements(writer, &self.children, options)
/// 		})
/// 	}
/// }
///
/// # fn main() -> ssml::Result<()> {
/// let doc = ssml::speak(
/// 	Some("en-US"),
/// 	[TomfooleryElement::new(2.0, ["Approaching dangerous levels of tomfoolery!"]).into_dyn()]
/// );
/// let str = doc.serialize_to_string(&ssml::SerializeOptions::default().pretty())?;
/// assert_eq!(
/// 	str,
/// 	r#"<speak version="1.0" xmlns="http://www.w3.org/2001/10/synthesis" xml:lang="en-US">
/// 	<tomfoolery influence="2">
/// 		Approaching dangerous levels of tomfoolery!
/// 	</tomfoolery>
/// </speak>"#
/// );
/// # Ok(())
/// # }
/// ```
pub trait DynElement: Debug + DynClone + Send {
	/// Serialize this dynamic element into an [`XmlWriter`].
	///
	/// See [`Serialize::serialize_xml`] for more information.
	fn serialize_xml(&self, writer: &mut XmlWriter<'_>, options: &SerializeOptions) -> crate::Result<()>;

	/// An optional tag representing this dynamic element.
	fn tag_name(&self) -> Option<&str> {
		None
	}

	/// If this element has children, returns a reference to the vector containing the element's children.
	fn children(&self) -> Option<&Vec<Element>> {
		None
	}

	/// If this element has children, returns a mutable reference to the vector containing the element's children.
	fn children_mut(&mut self) -> Option<&mut Vec<Element>> {
		None
	}
}

dyn_clone::clone_trait_object!(DynElement);

impl Serialize for Box<dyn DynElement> {
	fn serialize_xml(&self, writer: &mut XmlWriter<'_>, options: &SerializeOptions) -> crate::Result<()> {
		DynElement::serialize_xml(self.as_ref(), writer, options)?;
		Ok(())
	}
}
