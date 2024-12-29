use crate::Property;

#[derive(Clone, Copy, Debug, Default)]
pub enum Display {
	None,
	#[default]
	Default,
}

impl Property for Display {
	type Value = Self;
}
