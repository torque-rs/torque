use crate::Property;

#[derive(Clone, Copy, Debug, Default)]
pub enum Layout {
	#[default]
	Block,
	Grid,
	FlexBox,
}

impl Property for Layout {}
