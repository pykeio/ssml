use ssml::{Serialize, SerializeOptions};

fn main() {
	let mut doc = ssml::speak(Some("en-US"), ["Hello, world!"])
		+ (ssml::voice("en-US-Neural2-F", ["This is an example of "])
			+ ssml::say_as(ssml::SpeechFormat::SpellOut, "SSML")
			+ " in " + ssml::emphasis(ssml::EmphasisLevel::Moderate, ["Rust."]))
		+ ssml::breaks(ssml::Break::new_with_time("1s"));
	doc += "How cool!";
	println!("{}", doc.serialize_to_string(&SerializeOptions::default().pretty()).unwrap());
}
