use core::{
	fmt::{self, Display},
	str::Utf8Error
};

use crate::{DecibelsError, TimeDesignationError};

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
	FmtError(fmt::Error),
	TimeDesignationError(TimeDesignationError),
	DecibelsError(DecibelsError),
	AttributesInChildContext,
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
	FmtError => fmt::Error, Utf8Error => Utf8Error, TimeDesignationError => TimeDesignationError, DecibelsError => DecibelsError
}

impl Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Error::FmtError(e) => e.fmt(f),
			Error::Utf8Error(e) => e.fmt(f),
			Error::TimeDesignationError(e) => e.fmt(f),
			Error::DecibelsError(e) => e.fmt(f),
			Error::AttributesInChildContext => f.write_str("invalid ordering: attempted to write attributes after writing children")
		}
	}
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

pub type Result<T, E = Error> = core::result::Result<T, E>;
