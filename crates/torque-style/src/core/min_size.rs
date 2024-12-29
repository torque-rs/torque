use crate::Property;

use super::LengthPercentAuto;

#[derive(Clone, Copy, Debug, Default)]
pub struct MinSize {
	pub width: LengthPercentAuto,
	pub height: LengthPercentAuto,
}

impl Property for MinSize {}

impl From<MinSize> for taffy::Size<taffy::Dimension> {
	fn from(value: MinSize) -> Self {
		Self {
			width: value.width.into(),
			height: value.height.into(),
		}
	}
}
