use crate::Property;

use super::LengthPercentAuto;

#[derive(Clone, Copy, Debug, Default)]
pub struct Margin {
	left: LengthPercentAuto,
	right: LengthPercentAuto,
	top: LengthPercentAuto,
	bottom: LengthPercentAuto,
}

impl Property for Margin {}

impl From<Margin> for taffy::Rect<taffy::LengthPercentageAuto> {
	fn from(value: Margin) -> Self {
		Self {
			left: value.left.into(),
			right: value.right.into(),
			top: value.top.into(),
			bottom: value.bottom.into(),
		}
	}
}
