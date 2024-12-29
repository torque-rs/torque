use std::collections::VecDeque;

use torque_ecs::{Component, EntityRef};

use crate::Node;

#[derive(Default)]
pub struct Children;

impl Component for Children {
	const NAME: &str = "Children";

	type Value = VecDeque<EntityRef<Node>>;
}
