use crate::Property;

use super::LengthPercentAuto;

#[derive(Clone, Copy, Debug, Default)]
pub struct Inset {
	pub left: LengthPercentAuto,
	pub right: LengthPercentAuto,
	pub top: LengthPercentAuto,
	pub bottom: LengthPercentAuto,
}

impl Property for Inset {}

impl From<Inset> for taffy::Rect<taffy::LengthPercentageAuto> {
	fn from(value: Inset) -> Self {
		Self {
			left: value.left.into(),
			right: value.right.into(),
			top: value.top.into(),
			bottom: value.bottom.into(),
		}
	}
}
