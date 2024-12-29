use crate::Resolve;

#[derive(Clone, Copy, Debug)]
pub enum LengthPercent {
	Length(f32),
	Percent(f32),
}

impl Resolve<Option<f32>, Option<f32>> for LengthPercent {
	fn resolve(self, context: Option<f32>) -> Option<f32> {
		match self {
			LengthPercent::Length(length) => Some(length),
			LengthPercent::Percent(percent) => context.map(|value| value * percent * 0.01),
		}
	}
}
