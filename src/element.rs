use std::fmt::{Debug, Write};

use dyn_clone::DynClone;

use crate::{Audio, Break, Emphasis, Mark, Meta, Serialize, SerializeOptions, Text, Voice, XmlWriter};

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
			fn serialize_xml<W: ::std::fmt::Write>(&self, writer: &mut $crate::XmlWriter<W>, options: &$crate::SerializeOptions) -> crate::Result<()> {
				match self {
					$($name::$variant(inner) => inner.serialize_xml(writer, options),)*
				}
			}
		}
	};
}
pub(crate) use el;

el! {
	/// Represents all SSML elements.
	#[derive(Clone, Debug)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize))]
	#[non_exhaustive]
	pub enum Element {
		Text(Text),
		Audio(Audio),
		Voice(Voice),
		Meta(Meta),
		Break(Break),
		Emphasis(Emphasis),
		Mark(Mark),
		FlavorMSTTS(crate::mstts::Element),
		/// A dyn element can be used to implement your own custom elements outside of the `ssml` crate. See
		/// [`DynElement`] for more information and examples.
		Dyn(Box<dyn DynElement>)
		// Lang(LangElement),
		// Paragraph(ParagraphElement),
		// Phoneme(PhonemeElement),
		// Prosody(ProsodyElement),
		// SayAs(SayAsElement),
		// Sub(SubElement),
		// Sentence(SentenceElement),
		// Word(WordElement)
	}
}

#[cfg(feature = "serde")]
impl<'a> serde::Deserialize<'a> for Element {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'a>
	{
		#[allow(non_camel_case_types)]
		enum ElementField {
			Text,
			Audio,
			Voice,
			Meta,
			Break,
			Emphasis,
			Mark
		}

		struct ElementFieldVisitor;

		impl<'de> serde::de::Visitor<'de> for ElementFieldVisitor {
			type Value = ElementField;

			fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				f.write_str("variant identifier")
			}

			fn visit_u64<__E>(self, val: u64) -> serde::__private::Result<Self::Value, __E>
			where
				__E: serde::de::Error
			{
				match val {
					0u64 => Ok(ElementField::Text),
					1u64 => Ok(ElementField::Audio),
					2u64 => Ok(ElementField::Voice),
					3u64 => Ok(ElementField::Meta),
					4u64 => Ok(ElementField::Break),
					5u64 => Ok(ElementField::Emphasis),
					6u64 => Ok(ElementField::Mark),
					7u64 => Err(serde::de::Error::custom("DynElements cannot be deserialized")),
					_ => Err(serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(val), &"variant index 0 <= i < 8"))
				}
			}

			fn visit_str<E>(self, val: &str) -> Result<Self::Value, E>
			where
				E: serde::de::Error
			{
				match val {
					"Text" => Ok(ElementField::Text),
					"Audio" => Ok(ElementField::Audio),
					"Voice" => Ok(ElementField::Voice),
					"Meta" => Ok(ElementField::Meta),
					"Break" => Ok(ElementField::Break),
					"Emphasis" => Ok(ElementField::Emphasis),
					"Mark" => Ok(ElementField::Mark),
					"Dyn" => Err(serde::de::Error::custom("DynElements cannot be deserialized")),
					_ => Err(serde::de::Error::unknown_variant(val, VARIANTS))
				}
			}

			fn visit_bytes<E>(self, val: &[u8]) -> serde::__private::Result<Self::Value, E>
			where
				E: serde::de::Error
			{
				match val {
					b"Text" => Ok(ElementField::Text),
					b"Audio" => Ok(ElementField::Audio),
					b"Voice" => Ok(ElementField::Voice),
					b"Meta" => Ok(ElementField::Meta),
					b"Break" => Ok(ElementField::Break),
					b"Emphasis" => Ok(ElementField::Emphasis),
					b"Mark" => Ok(ElementField::Mark),
					b"Dyn" => Err(serde::de::Error::custom("DynElements cannot be deserialized")),
					_ => {
						let __value = &String::from_utf8_lossy(val);
						Err(serde::de::Error::unknown_variant(__value, VARIANTS))
					}
				}
			}
		}

		impl<'de> serde::Deserialize<'de> for ElementField {
			#[inline]
			fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
			where
				D: serde::Deserializer<'de>
			{
				serde::Deserializer::deserialize_identifier(deserializer, ElementFieldVisitor)
			}
		}

		#[doc(hidden)]
		struct Visitor<'de> {
			marker: std::marker::PhantomData<Element>,
			lifetime: std::marker::PhantomData<&'de ()>
		}
		impl<'de> serde::de::Visitor<'de> for Visitor<'de> {
			type Value = Element;

			fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				f.write_str("enum Element")
			}

			fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
			where
				A: serde::de::EnumAccess<'de>
			{
				match serde::de::EnumAccess::variant(data)? {
					(ElementField::Text, variant) => serde::de::VariantAccess::newtype_variant::<Text>(variant).map(Element::Text),
					(ElementField::Audio, variant) => serde::de::VariantAccess::newtype_variant::<Audio>(variant).map(Element::Audio),
					(ElementField::Voice, variant) => serde::de::VariantAccess::newtype_variant::<Voice>(variant).map(Element::Voice),
					(ElementField::Meta, variant) => serde::de::VariantAccess::newtype_variant::<Meta>(variant).map(Element::Meta),
					(ElementField::Break, variant) => serde::de::VariantAccess::newtype_variant::<Break>(variant).map(Element::Break),
					(ElementField::Emphasis, variant) => serde::de::VariantAccess::newtype_variant::<Emphasis>(variant).map(Element::Emphasis),
					(ElementField::Mark, variant) => serde::de::VariantAccess::newtype_variant::<Mark>(variant).map(Element::Mark)
				}
			}
		}

		#[doc(hidden)]
		const VARIANTS: &[&str] = &["Text", "Audio", "Voice", "Meta", "Break", "Emphasis", "Mark"];
		serde::Deserializer::deserialize_enum(deserializer, "Element", VARIANTS, Visitor {
			marker: serde::__private::PhantomData::<Element>,
			lifetime: serde::__private::PhantomData
		})
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
/// use std::fmt;
///
/// use ssml::{DynElement, Element, Serialize, SerializeOptions, XmlWriter};
///
/// #[derive(Debug, Clone)]
/// #[cfg_attr(feature = "serde", derive(serde::Serialize))]
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
/// 	fn serialize_xml(
/// 		&self,
/// 		writer: &mut XmlWriter<&mut dyn fmt::Write>,
/// 		options: &SerializeOptions
/// 	) -> ssml::Result<()> {
/// 		writer.element("tomfoolery", |writer| {
/// 			writer.attr("influence", self.value.to_string())?;
/// 			ssml::util::serialize_elements(writer, &self.children, options)
/// 		})
/// 	}
/// }
///
/// # fn main() -> ssml::Result<()> {
/// let doc = ssml::speak(Some("en-US"), [TomfooleryElement::new(2.0, [
/// 	"Approaching dangerous levels of tomfoolery!"
/// ])
/// .into_dyn()]);
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
#[allow(private_bounds)]
pub trait DynElement: Debug + DynClone + Send + OptionalErasedSerialize {
	/// Serialize this dynamic element into an [`XmlWriter`].
	///
	/// See [`Serialize::serialize_xml`] for more information.
	fn serialize_xml(&self, writer: &mut XmlWriter<&mut dyn Write>, options: &SerializeOptions) -> crate::Result<()>;

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

#[cfg(feature = "serde")]
erased_serde::serialize_trait_object!(DynElement);

#[cfg(feature = "serde")]
trait OptionalErasedSerialize: erased_serde::Serialize {}
#[cfg(feature = "serde")]
impl<T: serde::Serialize> OptionalErasedSerialize for T {}

#[cfg(not(feature = "serde"))]
trait OptionalErasedSerialize {}
#[cfg(not(feature = "serde"))]
impl<T> OptionalErasedSerialize for T {}

dyn_clone::clone_trait_object!(DynElement);

impl Serialize for Box<dyn DynElement> {
	fn serialize_xml<W: Write>(&self, writer: &mut XmlWriter<W>, options: &SerializeOptions) -> crate::Result<()> {
		let state = {
			let mut as_dyn = writer.to_dyn();
			DynElement::serialize_xml(self.as_ref(), &mut as_dyn, options)?;
			as_dyn.into_state()
		};
		writer.synchronize_state(state);
		Ok(())
	}
}
