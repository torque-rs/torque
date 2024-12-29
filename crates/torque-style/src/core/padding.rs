use crate::Property;

use super::LengthPercent;

#[derive(Clone, Copy, Debug)]
pub struct Padding {
	left: LengthPercent,
	right: LengthPercent,
	top: LengthPercent,
	bottom: LengthPercent,
}

impl Property for Padding {}

impl From<Padding> for taffy::Rect<taffy::LengthPercentage> {
	fn from(value: Padding) -> Self {
		Self {
			left: value.left.into(),
			right: value.right.into(),
			top: value.top.into(),
			bottom: value.bottom.into(),
		}
	}
}
