use crate::Property;

#[derive(Clone, Copy, Debug, Default)]
pub enum Position {
	#[default]
	Relative,
	Absolute,
}

impl Property for Position {}

impl From<Position> for taffy::Position {
	fn from(value: Position) -> Self {
		match value {
			Position::Relative => Self::Relative,
			Position::Absolute => Self::Absolute,
		}
	}
}
