#[derive(Clone, Copy, Debug, Default)]
pub enum LengthPercentAuto {
	Length(f32),
	Percent(f32),
	#[default]
	Auto,
}

impl From<LengthPercentAuto> for taffy::LengthPercentageAuto {
	fn from(value: LengthPercentAuto) -> Self {
		match value {
			LengthPercentAuto::Length(length) => Self::Length(length),
			LengthPercentAuto::Percent(percent) => Self::Percent(percent),
			LengthPercentAuto::Auto => Self::Auto,
		}
	}
}

impl From<LengthPercentAuto> for taffy::Dimension {
	fn from(value: LengthPercentAuto) -> Self {
		match value {
			LengthPercentAuto::Length(length) => Self::Length(length),
			LengthPercentAuto::Percent(percent) => Self::Percent(percent),
			LengthPercentAuto::Auto => Self::Auto,
		}
	}
}
