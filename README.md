# `ssml-rs`
A Rust library for writing SSML.

Currently, `ssml-rs` focuses on supporting the subsets of SSML supported by major cloud text-to-speech services ([Microsoft Azure Cognitive Speech Services](https://learn.microsoft.com/en-us/azure/ai-services/speech-service/speech-synthesis-markup-structure), [Google Cloud Text-to-Speech](https://cloud.google.com/text-to-speech/docs/ssml), & [Amazon Polly](https://docs.aws.amazon.com/polly/latest/dg/supportedtags.html)) & pyke Songbird.

```rs
let doc = ssml::speak(Some("en-US"), ["Hello, world!"]);

use ssml::Serialize;
let str = doc.serialize_to_string(ssml::Flavor::AmazonPolly)?;
assert_eq!(str, r#"<speak xml:lang="en-US">Hello, world! </speak>"#);
```
