use crate::Property;

#[derive(Clone, Copy, Debug, Default)]
pub enum OverflowKind {
	#[default]
	Visible,
	Clip,
	Hidden,
	Scroll,
}

impl From<OverflowKind> for taffy::Overflow {
	fn from(value: OverflowKind) -> Self {
		match value {
			OverflowKind::Visible => Self::Visible,
			OverflowKind::Clip => Self::Clip,
			OverflowKind::Hidden => Self::Hidden,
			OverflowKind::Scroll => Self::Scroll,
		}
	}
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Overflow {
	pub x: OverflowKind,
	pub y: OverflowKind,
}

impl Property for Overflow {}

impl From<Overflow> for taffy::Point<taffy::Overflow> {
	fn from(value: Overflow) -> Self {
		Self {
			x: value.x.into(),
			y: value.y.into(),
		}
	}
}
