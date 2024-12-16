use core::{
	fmt::{self, Debug, Display},
	num::ParseFloatError,
	str::FromStr
};

use crate::xml::TrustedNoEscape;

#[derive(Debug, PartialEq)]
pub enum TimeDesignationError {
	BadUnit,
	BadLength,
	Negative,
	ParseFloat(ParseFloatError)
}

impl Display for TimeDesignationError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			TimeDesignationError::BadUnit => f.write_str("time designation has invalid unit (allowed are ms, s)"),
			TimeDesignationError::BadLength => f.write_str("string is too short to be a valid time designation"),
			TimeDesignationError::Negative => f.write_str("time designations cannot be negative"),
			TimeDesignationError::ParseFloat(e) => f.write_fmt(format_args!("couldn't parse float: {e}"))
		}
	}
}

#[cfg(feature = "std")]
impl std::error::Error for TimeDesignationError {}

/// A time designation is a representation of a non-negative offset of time.
///
/// Time designations can be provided in either seconds (`s`) or milliseconds (`ms`):
/// ```
/// # use ssml::TimeDesignation;
/// # fn main() -> ssml::Result<()> {
/// assert_eq!("15s".parse::<TimeDesignation>()?, TimeDesignation::from_millis(15_000.));
/// assert_eq!("750ms".parse::<TimeDesignation>()?, TimeDesignation::from_millis(750.));
/// assert_eq!("+0.75s".parse::<TimeDesignation>()?, TimeDesignation::from_millis(750.));
///
/// // Fails
/// assert!("-5s".parse::<TimeDesignation>().is_err());
/// assert!("5 s".parse::<TimeDesignation>().is_err());
/// assert!("15sec".parse::<TimeDesignation>().is_err());
/// assert!("5m".parse::<TimeDesignation>().is_err());
/// # Ok(())
/// # }
/// ```
#[derive(Default, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TimeDesignation {
	millis: f32
}

impl TimeDesignation {
	/// Create a [`TimeDesignation`] from a set number of milliseconds.
	pub fn from_millis(millis: f32) -> Self {
		Self { millis }
	}

	/// Convert this time designation to milliseconds.
	pub fn to_millis(&self) -> f32 {
		self.millis
	}
}

impl FromStr for TimeDesignation {
	type Err = TimeDesignationError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let len = s.len();
		if len < 2 {
			return Err(TimeDesignationError::BadLength);
		}

		let (unit, skip) = if s.ends_with("ms") {
			(1., 2)
		} else if s.ends_with('s') && matches!(s.chars().nth(len - 2), Some('0'..='9') | Some('.')) {
			(1000., 1)
		} else {
			return Err(TimeDesignationError::BadUnit);
		};

		let f = s[..len - skip].parse::<f32>().map_err(TimeDesignationError::ParseFloat)?;
		if f.is_sign_negative() {
			return Err(TimeDesignationError::Negative);
		}

		Ok(Self::from_millis(f * unit))
	}
}

impl From<&str> for TimeDesignation {
	fn from(value: &str) -> Self {
		value.parse().unwrap_or_default()
	}
}

impl Display for TimeDesignation {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_fmt(format_args!("{:+}ms", self.to_millis()))
	}
}
impl TrustedNoEscape for TimeDesignation {}

impl Debug for TimeDesignation {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		Display::fmt(self, f)
	}
}

#[derive(Debug, PartialEq)]
pub enum DecibelsError {
	BadUnit,
	BadLength,
	ParseFloat(ParseFloatError)
}

impl Display for DecibelsError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			DecibelsError::BadUnit => f.write_str("decibels has invalid unit (allowed are dB)"),
			DecibelsError::BadLength => f.write_str("string is too short to be a valid decibel value"),
			DecibelsError::ParseFloat(e) => f.write_fmt(format_args!("couldn't parse float: {e}"))
		}
	}
}

#[cfg(feature = "std")]
impl std::error::Error for DecibelsError {}

/// A string representation of a signed amplitude offset in decibels (`dB`).
///
/// ```
/// # use ssml::Decibels;
/// # fn main() -> ssml::Result<()> {
/// assert_eq!("+0.0dB".parse::<Decibels>()?, Decibels::new(0.));
/// assert_eq!("-6dB".parse::<Decibels>()?, Decibels::new(-6.));
/// assert_eq!("2dB".parse::<Decibels>()?, Decibels::new(2.));
///
/// // Fails
/// assert!("-3DB".parse::<Decibels>().is_err());
/// assert!("0 dB".parse::<Decibels>().is_err());
/// assert!("6".parse::<Decibels>().is_err());
/// # Ok(())
/// # }
/// ```
#[derive(Default, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Decibels(f32);

impl Decibels {
	pub fn new(value: f32) -> Self {
		Self(value)
	}

	pub fn value(&self) -> f32 {
		self.0
	}
}

impl FromStr for Decibels {
	type Err = DecibelsError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let len = s.len();
		if len < 2 {
			return Err(DecibelsError::BadLength);
		}

		if !s.ends_with("dB") {
			return Err(DecibelsError::BadUnit);
		}

		let f = s[..len - 2].parse::<f32>().map_err(DecibelsError::ParseFloat)?;
		Ok(Self(f))
	}
}

impl From<f32> for Decibels {
	fn from(value: f32) -> Self {
		Decibels(value)
	}
}

impl From<&str> for Decibels {
	fn from(value: &str) -> Self {
		value.parse().unwrap_or_default()
	}
}

impl Display for Decibels {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_fmt(format_args!("{:+}dB", self.0))
	}
}
impl TrustedNoEscape for Decibels {}

impl Debug for Decibels {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		Display::fmt(self, f)
	}
}

pub(crate) struct SpeedFormatter(pub(crate) f32);
impl Display for SpeedFormatter {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_fmt(format_args!("{}%", self.0 * 100.))
	}
}
impl TrustedNoEscape for SpeedFormatter {}

#[cfg(test)]
mod tests {
	use super::{Decibels, TimeDesignation};

	#[test]
	fn parse_time_designation() {
		assert_eq!("+7s".parse::<TimeDesignation>(), Ok(TimeDesignation::from_millis(7000.0)));
		assert_eq!("700ms".parse::<TimeDesignation>(), Ok(TimeDesignation::from_millis(700.0)));
		assert!("-.7s".parse::<TimeDesignation>().is_err());
	}

	#[test]
	fn parse_decibels() {
		assert_eq!("+6dB".parse::<Decibels>(), Ok(Decibels(6.0)));
		assert_eq!("-.6dB".parse::<Decibels>(), Ok(Decibels(-0.6)));
		assert!("6".parse::<Decibels>().is_err());
		assert!("6db".parse::<Decibels>().is_err());
	}
}
