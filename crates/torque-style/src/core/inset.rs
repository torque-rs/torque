use torque_geometry::Rect;

use crate::{LengthPercentAuto, Property};

pub struct Inset;

impl Property for Inset {
	type Value = Rect<LengthPercentAuto>;
}
