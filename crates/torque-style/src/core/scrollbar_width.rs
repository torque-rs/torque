use crate::Property;

#[derive(Clone, Copy, Debug, Default)]
pub struct ScrollbarWidth(f32);

impl Property for ScrollbarWidth {}

impl From<f32> for ScrollbarWidth {
	fn from(value: f32) -> Self {
		Self(value)
	}
}

impl From<ScrollbarWidth> for f32 {
	fn from(value: ScrollbarWidth) -> Self {
		value.0
	}
}
