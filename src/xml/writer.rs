use alloc::string::String;
use core::fmt::{self, Display, Write};

use crate::util;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum XmlState {
	DocumentStart,
	ElementUnclosed,
	ElementClosed
}

/// A utility for writing optionally formatted XML to a [`Write`] stream.
pub struct XmlWriter<W> {
	pub(crate) write: W,
	indent_level: u8,
	pub(crate) pretty: bool,
	state: XmlState
}

pub trait EscapedDisplay: Display {
	fn escaped_fmt<W: Write>(&self, w: &mut W) -> fmt::Result;
}
impl EscapedDisplay for &str {
	fn escaped_fmt<W: Write>(&self, w: &mut W) -> fmt::Result {
		util::escape(w, self)
	}
}
impl EscapedDisplay for String {
	fn escaped_fmt<W: Write>(&self, w: &mut W) -> fmt::Result {
		util::escape(w, self)
	}
}
impl EscapedDisplay for &String {
	fn escaped_fmt<W: Write>(&self, w: &mut W) -> fmt::Result {
		util::escape(w, self)
	}
}

pub(crate) trait TrustedNoEscape: Display {}
impl<T: TrustedNoEscape> EscapedDisplay for T {
	fn escaped_fmt<W: Write>(&self, w: &mut W) -> fmt::Result {
		w.write_fmt(format_args!("{}", self))
	}
}
impl<T: TrustedNoEscape> TrustedNoEscape for &T {}
impl TrustedNoEscape for u8 {}
impl TrustedNoEscape for f32 {}

impl<W: Write> XmlWriter<W> {
	/// Creates a new [`XmlWriter`] with the given backing [`Write`] stream.
	pub fn new(writer: W, pretty: bool) -> Self {
		Self {
			write: writer,
			indent_level: 0,
			pretty,
			state: XmlState::DocumentStart
		}
	}

	fn pretty_break(&mut self) -> crate::Result<()> {
		if self.pretty {
			self.write.write_char('\n')?;
			for _ in 0..self.indent_level {
				self.write.write_char('\t')?;
			}
		}
		Ok(())
	}

	/// Starts an XML element context.
	///
	/// Note that child elements **must** be written *after* any attributes.
	pub fn element(&mut self, tag_name: impl AsRef<str>, ctx: impl FnOnce(&mut Self) -> crate::Result<()>) -> crate::Result<()> {
		let tag_name = tag_name.as_ref();

		if self.state == XmlState::ElementUnclosed {
			self.write.write_char('>')?;
		}
		if self.state != XmlState::DocumentStart {
			self.pretty_break()?;
		}

		self.write.write_char('<')?;
		self.write.write_str(tag_name)?;

		self.state = XmlState::ElementUnclosed;
		self.indent_level = self.indent_level.saturating_add(1);
		ctx(self)?;

		self.indent_level = self.indent_level.saturating_sub(1);
		match self.state {
			XmlState::ElementUnclosed => {
				if self.pretty {
					self.write.write_char(' ')?;
				}
				self.write.write_str("/>")?;
			}
			XmlState::ElementClosed => {
				self.pretty_break()?;
				self.write.write_str("</")?;
				self.write.write_str(tag_name)?;
				self.write.write_char('>')?;
			}
			_ => {}
		}
		self.state = XmlState::ElementClosed;

		Ok(())
	}

	/// Starts an attribute context.
	///
	/// Note that attributes **must** be written *before* any child elements.
	pub fn attr(&mut self, attr_name: impl AsRef<str>, attr_value: impl EscapedDisplay) -> crate::Result<()> {
		if self.state == XmlState::ElementClosed {
			return Err(crate::Error::AttributesInChildContext);
		}

		self.write.write_char(' ')?;
		self.write.write_str(attr_name.as_ref())?;
		self.write.write_str("=\"")?;
		attr_value.escaped_fmt(&mut self.write)?;
		self.write.write_char('"')?;

		Ok(())
	}

	/// Starts an attribute context if the given `attr_value` is `Some`.
	///
	/// Note that attributes **must** be written *before* any child elements.
	pub fn attr_opt(&mut self, attr_name: impl AsRef<str>, attr_value: Option<impl EscapedDisplay>) -> crate::Result<()> {
		if let Some(attr_value) = attr_value { self.attr(attr_name, attr_value) } else { Ok(()) }
	}

	/// Escapes and inserts the given text into the XML stream.
	pub fn text(&mut self, contents: impl AsRef<str>) -> crate::Result<()> {
		if self.state == XmlState::ElementUnclosed {
			self.write.write_char('>')?;
		}
		if self.state != XmlState::DocumentStart {
			self.pretty_break()?;
		}

		util::escape(&mut self.write, contents)?;

		self.state = XmlState::ElementClosed;

		Ok(())
	}

	/// Inserts the given text into the XML stream verbatim, closing any open elements, with no escaping performed.
	pub fn raw(&mut self, contents: impl Display) -> crate::Result<()> {
		if self.state == XmlState::ElementUnclosed {
			self.write.write_char('>')?;
		}
		if self.state != XmlState::DocumentStart {
			self.pretty_break()?;
		}

		write!(self.write, "{}", contents)?;

		self.state = XmlState::ElementClosed;

		Ok(())
	}
}
