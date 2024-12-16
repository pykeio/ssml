use ssml::{IntoElement, Serialize, SerializeOptions};

fn main() {
	let doc = ssml::speak(Some("en-US"), [
		ssml::text("Hello, world!").into_element(),
		ssml::voice("en-US-Neural2-F", [
			"This is an example of".into_element(),
			ssml::say_as(ssml::SpeechFormat::SpellOut, "SSML").into(),
			" in ".into(),
			ssml::emphasis(ssml::EmphasisLevel::Moderate, ["Rust."]).into()
		])
		.into(),
		ssml::breaks(ssml::Break::new_with_time("1s")).into(),
		"How cool!".into()
	]);
	println!("{}", doc.serialize_to_string(&SerializeOptions::default().pretty()).unwrap());
}
