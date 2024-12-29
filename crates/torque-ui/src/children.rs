use std::{
	collections::{vec_deque::Iter, VecDeque},
	ops::{Deref, DerefMut},
};

use torque_ecs::Component;

use crate::Node;

#[derive(Default)]
pub struct Children(VecDeque<Node>);

impl Children {
	pub fn iter(&self) -> ChildIter {
		ChildIter(self.0.iter())
	}
}

impl Deref for Children {
	type Target = VecDeque<Node>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for Children {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

pub struct ChildIter<'a>(Iter<'a, Node>);

impl Iterator for ChildIter<'_> {
	type Item = Node;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().cloned()
	}
}

impl Component for Children {
	const NAME: &str = "Children";
}
