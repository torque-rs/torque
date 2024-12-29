use torque_geometry::Rect;

use crate::{LengthPercent, Property};

pub struct Border;

impl Property for Border {
	type Value = Rect<LengthPercent>;
}
