use crate::Property;

#[derive(Clone, Copy, Debug, Default)]
pub struct AspectRatio(f32);

impl From<f32> for AspectRatio {
	fn from(value: f32) -> Self {
		Self(value)
	}
}

impl From<AspectRatio> for f32 {
	fn from(value: AspectRatio) -> Self {
		value.0
	}
}

impl Property for AspectRatio {}
