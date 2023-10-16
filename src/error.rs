use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub(crate) struct GenericError(pub String);

impl Display for GenericError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.0)
	}
}

impl Error for GenericError {}

macro_rules! error {
	($m:literal) => {
		$crate::GenericError(format!($m))
	};
	($fmt:expr, $($arg:tt)*) => {
		$crate::GenericError(format!($fmt, $($arg)*))
	};
}
pub(crate) use error;
