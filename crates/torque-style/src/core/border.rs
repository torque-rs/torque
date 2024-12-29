use crate::Property;

use super::LengthPercent;

#[derive(Clone, Copy, Debug)]
pub struct Border {
	left: LengthPercent,
	right: LengthPercent,
	top: LengthPercent,
	bottom: LengthPercent,
}

impl Property for Border {}

impl From<Border> for taffy::Rect<taffy::LengthPercentage> {
	fn from(value: Border) -> Self {
		Self {
			left: value.left.into(),
			right: value.right.into(),
			top: value.top.into(),
			bottom: value.bottom.into(),
		}
	}
}
