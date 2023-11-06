use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XmlState {
	DocumentStart,
	ElementUnclosed,
	ElementClosed
}

/// A utility for writing optionally formatted XML to a [`Write`] stream.
pub struct XmlWriter<'w> {
	pub(crate) write: &'w mut dyn Write,
	indent_level: u8,
	pub(crate) pretty: bool,
	state: XmlState
}

impl<'w> XmlWriter<'w> {
	/// Creates a new [`XmlWriter`] with the given backing [`Write`] stream.
	pub fn new(writer: &'w mut dyn Write, pretty: bool) -> Self {
		Self {
			write: writer,
			indent_level: 0,
			pretty,
			state: XmlState::DocumentStart
		}
	}

	fn pretty_break(&mut self) -> crate::Result<()> {
		if self.pretty {
			self.write.write_all(b"\n")?;
			for _ in 0..self.indent_level {
				self.write.write_all(b"\t")?;
			}
		}
		Ok(())
	}

	/// Escape the given text for use in XML.
	pub fn escape(text: impl AsRef<str>) -> String {
		let text = text.as_ref();
		let mut str = String::with_capacity(text.len() + 6);
		for char in text.chars() {
			match char {
				'"' => str.push_str("&quot;"),
				'\'' => str.push_str("&apos;"),
				'<' => str.push_str("&lt;"),
				'>' => str.push_str("&gt;"),
				'&' => str.push_str("&amp;"),
				_ => str.push(char)
			}
		}
		str
	}

	/// Starts an XML element context.
	///
	/// Note that child elements **must** be written *after* any attributes.
	pub fn element(&mut self, tag_name: impl AsRef<str>, ctx: impl FnOnce(&mut Self) -> crate::Result<()>) -> crate::Result<()> {
		let tag_name = tag_name.as_ref();

		if self.state == XmlState::ElementUnclosed {
			self.write.write_all(b">")?;
		}
		if self.state != XmlState::DocumentStart {
			self.pretty_break()?;
		}

		self.write.write_all(b"<")?;
		self.write.write_all(tag_name.as_bytes())?;

		self.state = XmlState::ElementUnclosed;
		self.indent_level = self.indent_level.saturating_add(1);
		ctx(self)?;

		self.indent_level = self.indent_level.saturating_sub(1);
		match self.state {
			XmlState::ElementUnclosed => {
				if self.pretty {
					self.write.write_all(b" ")?;
				}
				self.write.write_all(b"/>")?;
			}
			XmlState::ElementClosed => {
				self.pretty_break()?;
				self.write.write_all(b"</")?;
				self.write.write_all(tag_name.as_bytes())?;
				self.write.write_all(b">")?;
			}
			_ => {}
		}
		self.state = XmlState::ElementClosed;

		Ok(())
	}

	/// Starts an attribute context.
	///
	/// Note that attributes **must** be written *before* any child elements.
	pub fn attr(&mut self, attr_name: impl AsRef<str>, attr_value: impl AsRef<str>) -> crate::Result<()> {
		if self.state == XmlState::ElementClosed {
			return Err(crate::Error::AttributesInChildContext);
		}

		self.write.write_all(b" ")?;
		self.write.write_all(attr_name.as_ref().as_bytes())?;
		self.write.write_all(b"=\"")?;
		self.write.write_all(XmlWriter::escape(attr_value).as_bytes())?;
		self.write.write_all(b"\"")?;

		Ok(())
	}

	/// Starts an attribute context if the given `attr_value` is `Some`.
	///
	/// Note that attributes **must** be written *before* any child elements.
	pub fn attr_opt(&mut self, attr_name: impl AsRef<str>, attr_value: Option<impl AsRef<str>>) -> crate::Result<()> {
		if let Some(attr_value) = attr_value { self.attr(attr_name, attr_value) } else { Ok(()) }
	}

	/// Escapes and inserts the given text into the XML stream.
	pub fn text(&mut self, contents: impl AsRef<str>) -> crate::Result<()> {
		self.raw(XmlWriter::escape(contents))
	}

	/// Inserts the given text into the XML stream verbatim, closing any open elements, with no escaping performed.
	pub fn raw(&mut self, contents: impl AsRef<str>) -> crate::Result<()> {
		if self.state == XmlState::ElementUnclosed {
			self.write.write_all(b">")?;
		}
		if self.state != XmlState::DocumentStart {
			self.pretty_break()?;
		}

		self.write.write_all(contents.as_ref().as_bytes())?;

		self.state = XmlState::ElementClosed;

		Ok(())
	}
}
