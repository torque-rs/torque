use crate::Property;

#[derive(Clone, Copy, Debug, Default)]
pub enum BoxSizing {
	#[default]
	Border,
	Content,
}

impl Property for BoxSizing {}

impl From<BoxSizing> for taffy::BoxSizing {
	fn from(value: BoxSizing) -> Self {
		match value {
			BoxSizing::Border => Self::BorderBox,
			BoxSizing::Content => Self::ContentBox,
		}
	}
}
