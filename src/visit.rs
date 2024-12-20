//! Walk through all elements of an SSML document.
//!
//! ## Example
//!
//! ```
//! # use ssml::{Flavor, Serialize};
//! # fn main() -> ssml::Result<()> {
//! use ssml::visit::{self, Visit};
//!
//! #[derive(Default)]
//! struct VoiceVisitor {
//! 	used_voices: Vec<String>
//! }
//!
//! impl<'s> Visit<'s> for VoiceVisitor {
//! 	fn visit_voice(&mut self, node: &'s ssml::Voice) {
//! 		if let Some(names) = &node.config().names {
//! 			self.used_voices.extend(names.iter().map(|n| n.to_string()));
//! 		}
//!
//! 		// Make sure to call the default implementation so we can also visit children.
//! 		visit::visit_voice(self, node);
//! 	}
//! }
//!
//! let doc = ssml::speak(None, [ssml::voice("en-US-Neural2-F", ["Hello, world!"])]);
//!
//! let mut visitor = VoiceVisitor::default();
//! visitor.visit_speak(&doc);
//! println!("Used voices: {:?}", visitor.used_voices);
//! # Ok(())
//! # }
//! ```

use crate::{Audio, Break, CustomElement, Element, Emphasis, Lang, Mark, Meta, Prosody, SayAs, Speak, Text, Voice, mstts};

pub trait Visit<'s> {
	fn visit_speak(&mut self, node: &'s Speak) {
		self::visit_speak(self, node)
	}

	fn visit_audio(&mut self, node: &'s Audio) {
		self::visit_audio(self, node)
	}

	fn visit_meta(&mut self, node: &'s Meta) {
		self::visit_meta(self, node)
	}

	fn visit_text(&mut self, node: &'s Text) {
		self::visit_text(self, node)
	}

	fn visit_voice(&mut self, node: &'s Voice) {
		self::visit_voice(self, node)
	}

	fn visit_break(&mut self, node: &'s Break) {
		self::visit_break(self, node)
	}

	fn visit_emphasis(&mut self, node: &'s Emphasis) {
		self::visit_emphasis(self, node)
	}

	fn visit_mark(&mut self, node: &'s Mark) {
		self::visit_mark(self, node)
	}

	fn visit_say_as(&mut self, node: &'s SayAs) {
		self::visit_say_as(self, node)
	}

	fn visit_lang(&mut self, node: &'s Lang) {
		self::visit_lang(self, node)
	}

	fn visit_prosody(&mut self, node: &'s Prosody) {
		self::visit_prosody(self, node)
	}

	fn visit_custom(&mut self, node: &'s CustomElement) {
		self::visit_custom(self, node)
	}

	fn visit_mstts_element(&mut self, node: &'s mstts::Element) {
		self::visit_mstts_element(self, node)
	}

	fn visit_mstts_express(&mut self, node: &'s mstts::Express) {
		self::visit_mstts_express(self, node)
	}

	fn visit_element(&mut self, node: &'s Element) {
		self::visit_element(self, node)
	}
}

pub fn visit_audio<'s, V: Visit<'s> + ?Sized>(v: &mut V, node: &'s Audio) {
	for node in node.alternate() {
		v.visit_element(node);
	}
}

pub fn visit_meta<'s, V: Visit<'s> + ?Sized>(_v: &mut V, _node: &'s Meta) {}

pub fn visit_text<'s, V: Visit<'s> + ?Sized>(_v: &mut V, _node: &'s Text) {}

pub fn visit_voice<'s, V: Visit<'s> + ?Sized>(v: &mut V, node: &'s Voice) {
	for node in node.children() {
		v.visit_element(node);
	}
}

pub fn visit_break<'s, V: Visit<'s> + ?Sized>(_v: &mut V, _node: &'s Break) {}

pub fn visit_emphasis<'s, V: Visit<'s> + ?Sized>(v: &mut V, node: &'s Emphasis) {
	for node in node.children() {
		v.visit_element(node);
	}
}

pub fn visit_mark<'s, V: Visit<'s> + ?Sized>(_v: &mut V, _node: &'s Mark) {}

pub fn visit_say_as<'s, V: Visit<'s> + ?Sized>(_v: &mut V, _node: &'s SayAs) {}

pub fn visit_lang<'s, V: Visit<'s> + ?Sized>(v: &mut V, node: &'s Lang) {
	for node in node.children() {
		v.visit_element(node);
	}
}

pub fn visit_prosody<'s, V: Visit<'s> + ?Sized>(v: &mut V, node: &'s Prosody) {
	for node in node.children() {
		v.visit_element(node);
	}
}

pub fn visit_custom<'s, V: Visit<'s> + ?Sized>(_v: &mut V, _node: &'s CustomElement) {}

pub fn visit_mstts_element<'s, V: Visit<'s> + ?Sized>(v: &mut V, node: &'s mstts::Element) {
	match node {
		mstts::Element::Express(node) => visit_mstts_express(v, node)
	}
}

pub fn visit_mstts_express<'s, V: Visit<'s> + ?Sized>(v: &mut V, node: &'s mstts::Express) {
	for node in node.children() {
		v.visit_element(node);
	}
}

pub fn visit_element<'s, V: Visit<'s> + ?Sized>(v: &mut V, node: &'s Element) {
	match node {
		Element::Audio(node) => visit_audio(v, node),
		Element::Meta(node) => visit_meta(v, node),
		Element::Text(node) => visit_text(v, node),
		Element::Voice(node) => visit_voice(v, node),
		Element::Break(node) => visit_break(v, node),
		Element::Emphasis(node) => visit_emphasis(v, node),
		Element::Mark(node) => visit_mark(v, node),
		Element::SayAs(node) => visit_say_as(v, node),
		Element::Lang(node) => visit_lang(v, node),
		Element::Prosody(node) => visit_prosody(v, node),
		Element::FlavorMSTTS(node) => visit_mstts_element(v, node),
		Element::Custom(node) => visit_custom(v, node),
		Element::Group(node) => {
			for child in node.children() {
				visit_element(v, child);
			}
		}
	}
}

pub fn visit_speak<'s, V: Visit<'s> + ?Sized>(v: &mut V, node: &'s Speak) {
	for node in node.children() {
		v.visit_element(node);
	}
}
