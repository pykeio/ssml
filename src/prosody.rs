use alloc::vec::Vec;
use core::{
	fmt::{self, Display, Write},
	ops::{Add, AddAssign}
};

use crate::{Decibels, Element, Serialize, SerializeOptions, TimeDesignation, XmlWriter, unit::SpeedFormatter, util, xml::TrustedNoEscape};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ProsodyPitch {
	Lower,
	Low,
	Medium,
	#[default]
	Default,
	High,
	Higher,
	Semitone(f32),
	Hz(f32)
}

impl ProsodyPitch {
	pub fn st(value: f32) -> Self {
		Self::Semitone(value)
	}

	pub fn hz(value: f32) -> Self {
		Self::Hz(value)
	}
}
impl Display for ProsodyPitch {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Lower => f.write_str("x-low"),
			Self::Low => f.write_str("low"),
			Self::Medium => f.write_str("medium"),
			Self::Default => f.write_str("default"),
			Self::High => f.write_str("high"),
			Self::Higher => f.write_str("x-high"),
			Self::Semitone(v) => f.write_fmt(format_args!("{v:+}st")),
			Self::Hz(v) => f.write_fmt(format_args!("{v:+}Hz"))
		}
	}
}
impl TrustedNoEscape for ProsodyPitch {}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ProsodyRate {
	Slower,
	Slow,
	Medium,
	#[default]
	Default,
	Fast,
	Faster,
	Rate(f32)
}
impl ProsodyRate {
	pub fn new(rate: f32) -> Self {
		Self::Rate(rate.max(0.))
	}
}
impl Display for ProsodyRate {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Slower => f.write_str("x-slow"),
			Self::Slow => f.write_str("slow"),
			Self::Medium => f.write_str("medium"),
			Self::Default => f.write_str("default"),
			Self::Fast => f.write_str("fast"),
			Self::Faster => f.write_str("x-fast"),
			Self::Rate(v) => SpeedFormatter(v.max(0.)).fmt(f)
		}
	}
}
impl TrustedNoEscape for ProsodyRate {}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ProsodyVolume {
	Silent,
	Softer,
	Soft,
	Medium,
	#[default]
	Default,
	Loud,
	Louder,
	Db(Decibels)
}

impl ProsodyVolume {
	pub fn db(db: impl Into<Decibels>) -> Self {
		Self::Db(db.into())
	}
}

impl Display for ProsodyVolume {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Silent => f.write_str("silent"),
			Self::Softer => f.write_str("x-soft"),
			Self::Soft => f.write_str("soft"),
			Self::Medium => f.write_str("medium"),
			Self::Default => f.write_str("default"),
			Self::Loud => f.write_str("loud"),
			Self::Louder => f.write_str("x-loud"),
			Self::Db(v) => v.fmt(f)
		}
	}
}
impl TrustedNoEscape for ProsodyVolume {}

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProsodyContour {
	points: Vec<(f32, ProsodyPitch)>
}

impl ProsodyContour {
	pub fn new() -> Self {
		Self { points: Vec::new() }
	}

	pub fn and(mut self, time: f32, pitch: impl Into<ProsodyPitch>) -> Self {
		self.points.push((time, pitch.into()));
		self
	}

	pub fn points(&self) -> &[(f32, ProsodyPitch)] {
		&self.points
	}

	pub fn points_mut(&mut self) -> &mut Vec<(f32, ProsodyPitch)> {
		&mut self.points
	}

	pub fn push(&mut self, time: f32, pitch: impl Into<ProsodyPitch>) {
		self.points.push((time, pitch.into()));
	}
}

impl Display for ProsodyContour {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut after_first = false;
		for (time, pitch) in &self.points {
			if after_first {
				f.write_char(' ')?;
			}
			f.write_char('(')?;
			SpeedFormatter(*time).fmt(f)?;
			f.write_char(',')?;
			pitch.fmt(f)?;
			f.write_char(')')?;
			after_first = true;
		}
		Ok(())
	}
}
impl TrustedNoEscape for ProsodyContour {}

impl<I: IntoIterator<Item = (f32, ProsodyPitch)>> From<I> for ProsodyContour {
	fn from(value: I) -> Self {
		ProsodyContour { points: value.into_iter().collect() }
	}
}

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProsodyControl {
	pub pitch: Option<ProsodyPitch>,
	pub contour: Option<ProsodyContour>,
	pub range: Option<ProsodyPitch>,
	pub rate: Option<ProsodyRate>,
	pub duration: Option<TimeDesignation>,
	pub volume: Option<ProsodyVolume>
}

impl ProsodyControl {
	pub fn with_pitch(mut self, pitch: impl Into<ProsodyPitch>) -> Self {
		self.pitch = Some(pitch.into());
		self
	}

	pub fn with_contour(mut self, contour: impl Into<ProsodyContour>) -> Self {
		self.contour = Some(contour.into());
		self
	}

	pub fn with_range(mut self, pitch: impl Into<ProsodyPitch>) -> Self {
		self.range = Some(pitch.into());
		self
	}

	pub fn with_rate(mut self, rate: impl Into<ProsodyRate>) -> Self {
		self.rate = Some(rate.into());
		self
	}

	pub fn with_duration(mut self, duration: impl Into<TimeDesignation>) -> Self {
		self.duration = Some(duration.into());
		self
	}

	pub fn with_volume(mut self, volume: impl Into<ProsodyVolume>) -> Self {
		self.volume = Some(volume.into());
		self
	}
}

impl From<ProsodyPitch> for ProsodyControl {
	fn from(pitch: ProsodyPitch) -> Self {
		Self {
			pitch: Some(pitch),
			..Default::default()
		}
	}
}
impl From<ProsodyContour> for ProsodyControl {
	fn from(contour: ProsodyContour) -> Self {
		Self {
			contour: Some(contour),
			..Default::default()
		}
	}
}
impl From<ProsodyRate> for ProsodyControl {
	fn from(rate: ProsodyRate) -> Self {
		Self {
			rate: Some(rate),
			..Default::default()
		}
	}
}
impl From<TimeDesignation> for ProsodyControl {
	fn from(rate: TimeDesignation) -> Self {
		Self {
			duration: Some(rate),
			..Default::default()
		}
	}
}
impl From<ProsodyVolume> for ProsodyControl {
	fn from(volume: ProsodyVolume) -> Self {
		Self {
			volume: Some(volume),
			..Default::default()
		}
	}
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prosody<'s> {
	control: ProsodyControl,
	pub(crate) children: Vec<Element<'s>>
}

impl<'s> Prosody<'s> {
	pub fn new<S: Into<Element<'s>>, I: IntoIterator<Item = S>>(control: impl Into<ProsodyControl>, elements: I) -> Self {
		Self {
			control: control.into(),
			children: elements.into_iter().map(|f| f.into()).collect()
		}
	}

	pub fn with_pitch(mut self, pitch: impl Into<ProsodyPitch>) -> Self {
		self.control.pitch = Some(pitch.into());
		self
	}

	pub fn with_contour(mut self, contour: impl Into<ProsodyContour>) -> Self {
		self.control.contour = Some(contour.into());
		self
	}

	pub fn with_range(mut self, pitch: impl Into<ProsodyPitch>) -> Self {
		self.control.range = Some(pitch.into());
		self
	}

	pub fn with_rate(mut self, rate: impl Into<ProsodyRate>) -> Self {
		self.control.rate = Some(rate.into());
		self
	}

	pub fn with_duration(mut self, duration: impl Into<TimeDesignation>) -> Self {
		self.control.duration = Some(duration.into());
		self
	}

	pub fn with_volume(mut self, volume: impl Into<ProsodyVolume>) -> Self {
		self.control.volume = Some(volume.into());
		self
	}

	pub fn control(&self) -> &ProsodyControl {
		&self.control
	}

	pub fn control_mut(&mut self) -> &mut ProsodyControl {
		&mut self.control
	}

	pub fn set_control(&mut self, control: ProsodyControl) {
		self.control = control;
	}

	pub fn children(&self) -> &[Element<'s>] {
		&self.children
	}

	pub fn children_mut(&mut self) -> &mut Vec<Element<'s>> {
		&mut self.children
	}

	pub fn push(&mut self, element: impl Into<Element<'s>>) {
		self.children.push(element.into());
	}

	pub fn extend<S: Into<Element<'s>>, I: IntoIterator<Item = S>>(&mut self, elements: I) {
		self.children.extend(elements.into_iter().map(|f| f.into()));
	}

	pub fn to_owned(&self) -> Prosody<'static> {
		self.clone().into_owned()
	}

	pub fn into_owned(self) -> Prosody<'static> {
		Prosody {
			control: self.control,
			children: self.children.into_iter().map(Element::into_owned).collect()
		}
	}
}

impl<'s> Serialize for Prosody<'s> {
	fn serialize_xml<W: Write>(&self, writer: &mut XmlWriter<W>, options: &SerializeOptions) -> crate::Result<()> {
		writer.element("prosody", |writer| {
			writer.attr_opt("pitch", self.control.pitch.as_ref())?;
			writer.attr_opt("range", self.control.range.as_ref())?;
			writer.attr_opt("rate", self.control.rate.as_ref())?;
			writer.attr_opt("duration", self.control.duration.as_ref())?;
			writer.attr_opt("volume", self.control.volume.as_ref())?;
			util::serialize_elements(writer, &self.children, options)
		})
	}
}

impl<'s, 's2: 's, T: Into<Element<'s2>>> Add<T> for Prosody<'s> {
	type Output = Prosody<'s>;

	fn add(mut self, rhs: T) -> Self::Output {
		self.push(rhs.into());
		self
	}
}

impl<'s, 's2: 's, T: Into<Element<'s2>>> AddAssign<T> for Prosody<'s> {
	fn add_assign(&mut self, rhs: T) {
		self.push(rhs.into());
	}
}

pub fn prosody<'s, S: Into<Element<'s>>, I: IntoIterator<Item = S>>(control: impl Into<ProsodyControl>, elements: I) -> Prosody<'s> {
	Prosody::new(control, elements)
}
