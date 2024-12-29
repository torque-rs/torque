#[derive(Clone, Copy, Debug)]
pub enum LengthPercent {
	Length(f32),
	Percent(f32),
}

impl From<LengthPercent> for taffy::LengthPercentage {
	fn from(value: LengthPercent) -> Self {
		match value {
			LengthPercent::Length(length) => Self::Length(length),
			LengthPercent::Percent(percent) => Self::Percent(percent),
		}
	}
}
