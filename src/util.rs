use std::io::{self, Write};

pub fn escape_xml<S: AsRef<str>>(str: S) -> String {
	let str = str.as_ref();
	str.replace('"', "&quot;")
		.replace('\'', "&apos;")
		.replace('<', "&lt;")
		.replace('>', "&gt;")
		.replace('&', "&amp;")
}

pub fn write_attr<W: Write>(writer: &mut W, key: impl AsRef<str>, val: impl AsRef<str>) -> io::Result<()> {
	write!(writer, " {}=\"{}\"", key.as_ref(), escape_xml(val))?;
	Ok(())
}
