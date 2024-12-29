use crate::Property;

#[derive(Clone, Copy, Debug, Default)]
pub enum Layout {
	#[default]
	Row,
	Column,
}

impl Property for Layout {
	type Value = Self;
}
