pub(crate) trait SpeechFormat {
	fn into_format(self) -> (String, Option<String>, Option<String>);
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SayAs {
	interpret_as: String,
	format: Option<String>,
	detail: Option<String>
}
