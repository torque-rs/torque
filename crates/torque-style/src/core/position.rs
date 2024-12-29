use crate::Property;

#[derive(Clone, Copy, Debug, Default)]
pub enum Position {
	#[default]
	Relative,
	Absolute,
}

impl Property for Position {
	type Value = Self;
}
