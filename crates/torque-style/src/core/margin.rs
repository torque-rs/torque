use torque_geometry::Rect;

use crate::{LengthPercentAuto, Property};

pub struct Margin;

impl Property for Margin {
	type Value = Rect<LengthPercentAuto>;
}
