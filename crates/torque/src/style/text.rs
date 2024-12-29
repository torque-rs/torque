use crate::properties;

use super::Property;

#[derive(Clone, Copy, Debug, Default)]
pub enum WritingMode {
	#[default]
	Horizontal,
	Vertical,
}

impl Property for WritingMode {
	type Value = Self;
}

properties! {
	FontSize: f32,
	LineHeight: f32
}
