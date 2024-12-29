use torque_geometry::Rect;

use crate::{LengthPercent, Property};

pub struct Padding;

impl Property for Padding {
	type Value = Rect<LengthPercent>;
}
