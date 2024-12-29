use crate::Property;

#[derive(Clone, Copy, Debug, Default)]
pub enum Display {
	None,
	#[default]
	Default,
}

impl Property for Display {}

impl From<Display> for taffy::BoxGenerationMode {
	fn from(value: Display) -> Self {
		match value {
			Display::None => Self::None,
			Display::Default => Self::Normal,
		}
	}
}
