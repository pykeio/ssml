use crate::{Audio, DynElement, Element, Meta, Speak, Text, Voice};

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

	fn visit_dyn_mut(&mut self, node: &'s mut dyn DynElement) {
		self::visit_dyn_mut(self, node)
	}

	fn visit_speakable_mut(&mut self, node: &'s mut Element) {
		self::visit_speakable_mut(self, node)
	}
}

pub fn visit_audio_mut<'s, V: VisitMut<'s> + ?Sized>(v: &mut V, node: &'s mut Audio) {
	for node in node.alternate_mut() {
		v.visit_speakable_mut(node);
	}
}

pub fn visit_meta_mut<'s, V: VisitMut<'s> + ?Sized>(_v: &mut V, _node: &'s mut Meta) {}

pub fn visit_text_mut<'s, V: VisitMut<'s> + ?Sized>(_v: &mut V, _node: &'s mut Text) {}

pub fn visit_voice_mut<'s, V: VisitMut<'s> + ?Sized>(v: &mut V, node: &'s mut Voice) {
	for node in node.children_mut() {
		v.visit_speakable_mut(node);
	}
}

pub fn visit_dyn_mut<'s, V: VisitMut<'s> + ?Sized>(_v: &mut V, _node: &'s mut dyn DynElement) {}

pub fn visit_speakable_mut<'s, V: VisitMut<'s> + ?Sized>(v: &mut V, node: &'s mut Element) {
	match node {
		Element::Audio(node) => visit_audio_mut(v, node),
		Element::Meta(node) => visit_meta_mut(v, node),
		Element::Text(node) => visit_text_mut(v, node),
		Element::Voice(node) => visit_voice_mut(v, node),
		Element::Dyn(node) => visit_dyn_mut(v, node.as_mut())
	}
}

pub fn visit_speak_mut<'s, V: VisitMut<'s> + ?Sized>(v: &mut V, node: &'s mut Speak) {
	for node in node.children_mut() {
		v.visit_speakable_mut(node);
	}
}
