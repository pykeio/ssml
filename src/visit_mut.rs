use crate::{Audio, Break, CustomElement, Element, Emphasis, Lang, Mark, Meta, SayAs, Speak, Text, Voice, mstts};

pub trait VisitMut<'s> {
	fn visit_speak_mut(&mut self, node: &'s mut Speak) {
		self::visit_speak_mut(self, node)
	}

	fn visit_audio_mut(&mut self, node: &'s mut Audio) {
		self::visit_audio_mut(self, node)
	}

	fn visit_meta_mut(&mut self, node: &'s mut Meta) {
		self::visit_meta_mut(self, node)
	}

	fn visit_text_mut(&mut self, node: &'s mut Text) {
		self::visit_text_mut(self, node)
	}

	fn visit_voice_mut(&mut self, node: &'s mut Voice) {
		self::visit_voice_mut(self, node)
	}

	fn visit_break_mut(&mut self, node: &'s mut Break) {
		self::visit_break_mut(self, node)
	}

	fn visit_emphasis_mut(&mut self, node: &'s mut Emphasis) {
		self::visit_emphasis_mut(self, node)
	}

	fn visit_mark_mut(&mut self, node: &'s mut Mark) {
		self::visit_mark_mut(self, node)
	}

	fn visit_say_as_mut(&mut self, node: &'s mut SayAs) {
		self::visit_say_as_mut(self, node)
	}

	fn visit_lang_mut(&mut self, node: &'s mut Lang) {
		self::visit_lang_mut(self, node)
	}

	fn visit_custom_mut(&mut self, node: &'s mut CustomElement) {
		self::visit_custom_mut(self, node)
	}

	fn visit_mstts_element_mut(&mut self, node: &'s mut mstts::Element) {
		self::visit_mstts_element_mut(self, node)
	}

	fn visit_mstts_express_mut(&mut self, node: &'s mut mstts::Express) {
		self::visit_mstts_express_mut(self, node)
	}

	fn visit_element_mut(&mut self, node: &'s mut Element) {
		self::visit_element_mut(self, node)
	}
}

pub fn visit_audio_mut<'s, V: VisitMut<'s> + ?Sized>(v: &mut V, node: &'s mut Audio) {
	for node in node.alternate_mut() {
		v.visit_element_mut(node);
	}
}

pub fn visit_meta_mut<'s, V: VisitMut<'s> + ?Sized>(_v: &mut V, _node: &'s mut Meta) {}

pub fn visit_text_mut<'s, V: VisitMut<'s> + ?Sized>(_v: &mut V, _node: &'s mut Text) {}

pub fn visit_voice_mut<'s, V: VisitMut<'s> + ?Sized>(v: &mut V, node: &'s mut Voice) {
	for node in node.children_mut() {
		v.visit_element_mut(node);
	}
}

pub fn visit_break_mut<'s, V: VisitMut<'s> + ?Sized>(_v: &mut V, _node: &'s mut Break) {}

pub fn visit_emphasis_mut<'s, V: VisitMut<'s> + ?Sized>(v: &mut V, node: &'s mut Emphasis) {
	for node in node.children_mut() {
		v.visit_element_mut(node);
	}
}

pub fn visit_mark_mut<'s, V: VisitMut<'s> + ?Sized>(_v: &mut V, _node: &'s mut Mark) {}

pub fn visit_say_as_mut<'s, V: VisitMut<'s> + ?Sized>(_v: &mut V, _node: &'s mut SayAs) {}

pub fn visit_lang_mut<'s, V: VisitMut<'s> + ?Sized>(v: &mut V, node: &'s mut Lang) {
	for node in node.children_mut() {
		v.visit_element_mut(node);
	}
}

pub fn visit_custom_mut<'s, V: VisitMut<'s> + ?Sized>(_v: &mut V, _node: &'s mut CustomElement) {}

pub fn visit_mstts_element_mut<'s, V: VisitMut<'s> + ?Sized>(v: &mut V, node: &'s mut mstts::Element) {
	match node {
		mstts::Element::Express(node) => visit_mstts_express_mut(v, node)
	}
}

pub fn visit_mstts_express_mut<'s, V: VisitMut<'s> + ?Sized>(v: &mut V, node: &'s mut mstts::Express) {
	for node in node.children_mut() {
		v.visit_element_mut(node);
	}
}

pub fn visit_element_mut<'s, V: VisitMut<'s> + ?Sized>(v: &mut V, node: &'s mut Element) {
	match node {
		Element::Audio(node) => visit_audio_mut(v, node),
		Element::Meta(node) => visit_meta_mut(v, node),
		Element::Text(node) => visit_text_mut(v, node),
		Element::Voice(node) => visit_voice_mut(v, node),
		Element::Break(node) => visit_break_mut(v, node),
		Element::Emphasis(node) => visit_emphasis_mut(v, node),
		Element::Mark(node) => visit_mark_mut(v, node),
		Element::SayAs(node) => visit_say_as_mut(v, node),
		Element::Lang(node) => visit_lang_mut(v, node),
		Element::FlavorMSTTS(node) => visit_mstts_element_mut(v, node),
		Element::Custom(node) => visit_custom_mut(v, node),
		Element::Group(node) => {
			for child in node.children_mut() {
				visit_element_mut(v, child);
			}
		}
	}
}

pub fn visit_speak_mut<'s, V: VisitMut<'s> + ?Sized>(v: &mut V, node: &'s mut Speak) {
	for node in node.children_mut() {
		v.visit_element_mut(node);
	}
}
