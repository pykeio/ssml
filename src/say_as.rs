use alloc::{borrow::Cow, boxed::Box, string::ToString};
use core::fmt::Write;

use crate::{Flavor, Serialize, SerializeOptions, XmlWriter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DateFormat {
	DateMonthYear,
	MonthDateYear,
	YearMonthDate,
	YearMonth,
	MonthYear,
	MonthDate,
	DateMonth,
	Date,
	Month,
	Year
}

impl DateFormat {
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::DateMonthYear => "dmy",
			Self::MonthDateYear => "mdy",
			Self::YearMonthDate => "ymd",
			Self::YearMonth => "ym",
			Self::MonthYear => "my",
			Self::MonthDate => "md",
			Self::DateMonth => "dm",
			Self::Date => "d",
			Self::Month => "m",
			Self::Year => "y"
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpeechFormat {
	SpellOut,
	Currency,
	Cardinal,
	Ordinal,
	Digits,
	Date(DateFormat),
	Time,
	Telephone,
	Custom {
		interpret_as: Box<str>,
		format: Option<Box<str>>,
		detail: Option<Box<str>>
	}
}

impl SpeechFormat {
	pub fn interpret_as(&self, flavor: Flavor) -> &str {
		match self {
			Self::SpellOut => "spell-out",
			Self::Currency => "currency",
			Self::Cardinal => "cardinal",
			Self::Ordinal => "ordinal",
			Self::Digits => {
				match flavor {
					Flavor::MicrosoftAzureCognitiveSpeechServices => "number_digit",
					Flavor::GoogleCloudTextToSpeech => "spell-out", // they don't have digits, but maybe this will work...?
					_ => "digits"
				}
			}
			Self::Date(_) => "date",
			Self::Time => "time",
			Self::Telephone => "telephone",
			Self::Custom { interpret_as, .. } => &*interpret_as
		}
	}

	pub fn format(&self) -> Option<&str> {
		match self {
			Self::Date(format) => Some(format.as_str()),
			Self::Custom { format, .. } => format.as_deref(),
			_ => None
		}
	}

	pub fn detail(&self) -> Option<&str> {
		match self {
			Self::Custom { detail, .. } => detail.as_deref(),
			_ => None
		}
	}
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SayAs<'s> {
	format: SpeechFormat,
	pub(crate) text: Cow<'s, str>
}

impl<'s> SayAs<'s> {
	pub fn new(format: SpeechFormat, text: impl Into<Cow<'s, str>>) -> Self {
		Self { format, text: text.into() }
	}

	pub fn format(&self) -> &SpeechFormat {
		&self.format
	}

	pub fn set_format(&mut self, format: SpeechFormat) {
		self.format = format;
	}

	pub fn text(&self) -> &str {
		&self.text
	}

	pub fn set_text(&mut self, text: impl Into<Cow<'s, str>>) {
		self.text = text.into();
	}

	pub fn to_owned(&self) -> SayAs<'static> {
		self.clone().into_owned()
	}

	pub fn into_owned(self) -> SayAs<'static> {
		SayAs {
			format: self.format.clone(),
			text: match self.text {
				Cow::Borrowed(b) => Cow::Owned(b.to_string()),
				Cow::Owned(b) => Cow::Owned(b)
			}
		}
	}
}

impl<'s> Serialize for SayAs<'s> {
	fn serialize_xml<W: Write>(&self, writer: &mut XmlWriter<W>, options: &SerializeOptions) -> crate::Result<()> {
		writer.element("say-as", |writer| {
			writer.attr("interpret-as", self.format.interpret_as(options.flavor))?;
			writer.attr_opt("format", self.format.format())?;
			writer.attr_opt("detail", self.format.detail())?;
			writer.text(&self.text)
		})
	}
}

pub fn say_as<'s>(format: SpeechFormat, text: impl Into<Cow<'s, str>>) -> SayAs<'s> {
	SayAs::new(format, text)
}
