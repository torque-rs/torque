use crate::Property;

#[derive(Clone, Copy, Debug, Default)]
pub enum BoxSizing {
	#[default]
	Border,
	Content,
}

impl Property for BoxSizing {
	type Value = Self;
}
