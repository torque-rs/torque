use crate::Property;

use super::LengthPercentAuto;

#[derive(Clone, Copy, Debug, Default)]
pub struct Size {
	pub width: LengthPercentAuto,
	pub height: LengthPercentAuto,
}

impl Property for Size {}

impl From<Size> for taffy::Size<taffy::Dimension> {
	fn from(value: Size) -> Self {
		Self {
			width: value.width.into(),
			height: value.height.into(),
		}
	}
}
