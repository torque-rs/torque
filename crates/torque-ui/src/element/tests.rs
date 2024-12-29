use test_log::test;
use torque_ecs::System;

use crate::Children;

use super::{Element, ElementMethods};

fn print_children(children: &Children) {
	for child in children.iter() {
		log::debug!("{}", child.id);
	}
}

#[test]
pub fn append_child() {
	let system = System::default();
	let parent = system.create::<Element>();
	let child = system.create::<Element>();

	parent.append_child(child);

	parent.with_children(print_children);
}

#[test]
pub fn append_child2() {
	let system = System::default();
	let parent = system.create::<Element>();
	let child1 = system.create::<Element>();
	let child2 = system.create::<Element>();

	parent.append_child(child1);
	parent.append_child(child2);

	parent.with_children(print_children);
}
