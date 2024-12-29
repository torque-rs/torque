use crate::{LengthPercentAuto, Property};

pub struct Size;

impl Property for Size {
	type Value = torque_geometry::Size<LengthPercentAuto>;
}
