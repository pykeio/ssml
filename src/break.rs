use std::fmt::Write;

use crate::{Serialize, SerializeOptions, TimeDesignation, XmlWriter};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BreakStrength {
	None,
	ExtraWeak,
	Weak,
	#[default]
	Medium,
	Strong,
	ExtraStrong
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Break {
	Strength(BreakStrength),
	Time(TimeDesignation)
}

impl Break {
	pub fn new_with_strength(strength: BreakStrength) -> Self {
		Break::Strength(strength)
	}

	pub fn new_with_time(time: impl Into<TimeDesignation>) -> Self {
		Break::Time(time.into())
	}
}

impl From<BreakStrength> for Break {
	fn from(value: BreakStrength) -> Self {
		Break::new_with_strength(value)
	}
}

impl<S> From<S> for Break
where
	S: Into<TimeDesignation>
{
	fn from(value: S) -> Self {
		Break::new_with_time(value)
	}
}

impl Serialize for Break {
	fn serialize_xml<W: Write>(&self, writer: &mut XmlWriter<W>, _: &SerializeOptions) -> crate::Result<()> {
		writer.element("break", |writer| match self {
			Break::Strength(strength) => writer.attr("strength", match strength {
				BreakStrength::None => "none",
				BreakStrength::ExtraWeak => "x-weak",
				BreakStrength::Weak => "weak",
				BreakStrength::Medium => "medium",
				BreakStrength::Strong => "strong",
				BreakStrength::ExtraStrong => "x-strong"
			}),
			Break::Time(time) => writer.attr("time", time)
		})
	}
}

pub fn breaks(value: impl Into<Break>) -> Break {
	value.into()
}
