use std::ops::Deref;

use torque_ecs::Component;

use crate::Element;

#[derive(Clone, Default)]
pub struct Parent(Option<Element>);

impl Parent {
	pub fn unwrap(self) -> Option<Element> {
		self.0
	}
}

impl Deref for Parent {
	type Target = Option<Element>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl From<Element> for Parent {
	fn from(value: Element) -> Self {
		Self(Some(value))
	}
}

impl From<Option<Element>> for Parent {
	fn from(value: Option<Element>) -> Self {
		Self(value)
	}
}

impl From<Parent> for Option<Element> {
	fn from(value: Parent) -> Self {
		value.0
	}
}

impl Component for Parent {
	const NAME: &str = "Parent";
}
