use crate::Property;

#[derive(Clone, Copy, Debug, Default)]
pub enum OverflowKind {
	#[default]
	Visible,
	Clip,
	Hidden,
	Scroll,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Overflow {
	pub x: OverflowKind,
	pub y: OverflowKind,
}

impl Property for Overflow {
	type Value = Self;
}
