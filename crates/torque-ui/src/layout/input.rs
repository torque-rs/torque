use torque_geometry::Size;

use super::{AvailableSpace, Pass};

pub struct Input {
	pub pass: Pass,
	pub size: Size<Option<f32>>,
	pub parent_size: Size<Option<f32>>,
	pub available_space: AvailableSpace,
}
