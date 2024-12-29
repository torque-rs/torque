use crate::Property;

use super::LengthPercentAuto;

#[derive(Clone, Copy, Debug, Default)]
pub struct MaxSize {
	pub width: LengthPercentAuto,
	pub height: LengthPercentAuto,
}

impl Property for MaxSize {}

impl From<MaxSize> for taffy::Size<taffy::Dimension> {
	fn from(value: MaxSize) -> Self {
		Self {
			width: value.width.into(),
			height: value.height.into(),
		}
	}
}
