use std::{error::Error as StdError, fmt::Display, io, str::Utf8Error};

use crate::{DecibelsError, TimeDesignationError};

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
	IoError(io::Error),
	TimeDesignationError(TimeDesignationError),
	DecibelsError(DecibelsError),
	AttributesInChildContext,
	Generic(String),
	Utf8Error(Utf8Error)
}

unsafe impl Send for Error {}

macro_rules! impl_from {
	($($variant:ident => $t:ty),*) => {
		$(impl From<$t> for Error {
			fn from(e: $t) -> Self {
				Error::$variant(e)
			}
		})*
	};
}

impl_from! {
	IoError => io::Error, Utf8Error => Utf8Error, TimeDesignationError => TimeDesignationError, DecibelsError => DecibelsError
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::IoError(e) => e.fmt(f),
			Error::Utf8Error(e) => e.fmt(f),
			Error::TimeDesignationError(e) => e.fmt(f),
			Error::DecibelsError(e) => e.fmt(f),
			Error::AttributesInChildContext => f.write_str("invalid ordering: attempted to write attributes after writing children"),
			Error::Generic(s) => f.write_str(s)
		}
	}
}

impl StdError for Error {}

pub type Result<T, E = Error> = std::result::Result<T, E>;

macro_rules! error {
	($m:literal) => {
		$crate::Error::Generic(format!($m))
	};
	($fmt:expr, $($arg:tt)*) => {
		$crate::Error::Generic(format!($fmt, $($arg)*))
	};
}
pub(crate) use error;
