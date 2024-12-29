use super::{Node, NodeRef};

pub struct Image(pub(crate) NodeRef);

impl Node for Image {
	fn node_ref(&self) -> &NodeRef {
		&self.0
	}
}

#[derive(Debug)]
pub struct ImageContent {
	width: f32,
	height: f32,
}

impl ImageContent {
	pub fn measure(
		&mut self,
		known_dimensions: taffy::Size<Option<f32>>,
		_available_space: taffy::Size<taffy::AvailableSpace>,
	) -> taffy::Size<f32> {
		match (known_dimensions.width, known_dimensions.height) {
			(Some(width), Some(height)) => taffy::Size { width, height },
			(Some(width), None) => taffy::Size {
				width,
				height: (width / self.width) * self.height,
			},
			(None, Some(height)) => taffy::Size {
				width: (height / self.height) * self.width,
				height,
			},
			(None, None) => taffy::Size {
				width: self.width,
				height: self.height,
			},
		}
	}
}
