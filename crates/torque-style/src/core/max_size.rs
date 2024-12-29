use torque_geometry::Size;

use crate::{LengthPercentAuto, Property};

pub struct MaxSize;

impl Property for MaxSize {
	type Value = Size<LengthPercentAuto>;
}
