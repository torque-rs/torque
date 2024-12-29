use crate::Resolve;

#[derive(Clone, Copy, Debug, Default)]
pub enum LengthPercentAuto {
	Length(f32),
	Percent(f32),
	#[default]
	Auto,
}

impl Resolve<Option<f32>, Option<f32>> for LengthPercentAuto {
	fn resolve(self, context: Option<f32>) -> Option<f32> {
		match self {
			LengthPercentAuto::Length(length) => Some(length),
			LengthPercentAuto::Percent(percent) => context.map(|value| value * percent * 0.01),
			LengthPercentAuto::Auto => None,
		}
	}
}
