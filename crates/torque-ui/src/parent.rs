use torque_ecs::{Component, WeakEntityRef};

use crate::Element;

#[derive(Clone, Default)]
pub struct Parent;

impl Component for Parent {
	const NAME: &str = "Parent";

	type Value = Option<WeakEntityRef<Element>>;
}
