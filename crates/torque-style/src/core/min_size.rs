use torque_geometry::Size;

use crate::{LengthPercentAuto, Property};

pub struct MinSize;

impl Property for MinSize {
	type Value = Size<LengthPercentAuto>;
}
