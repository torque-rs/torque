use super::{Children, Inner, Node, NodeId, NodeRef};

pub struct Element(pub(crate) NodeRef);

impl Element {
	fn attach_child(
		inner: &mut Inner,
		parent: NodeRef,
		child: NodeRef,
		f: impl FnOnce(&mut ElementContent),
	) {
		let increment = {
			let Inner {
				node_contents,
				element_contents,
				..
			} = inner;

			let child = node_contents.get_mut(child.id).unwrap();

			let increment = if let Some(old_parent) = child.parent.take() {
				!element_contents
					.get_mut(old_parent.id)
					.unwrap()
					.detach_child(child.id)
			} else {
				true
			};

			f(element_contents.entry(parent.id).unwrap().or_default());

			child.parent = Some(parent);

			increment
		};
	}

	pub fn prepend_child(&self, child: impl Node) {
		let NodeRef { tree, id } = &self.0;
		let parent_id = *id;
		let child_id = child.id();

		tree.with_mut(|inner| {
			Self::attach_child(inner, parent_id, child_id, |element_content| {
				element_content.children.push_front(child_id);
			});
		});
	}

	pub fn append_child(&self, child: impl Node) {
		let NodeRef { tree, id } = &self.0;
		let parent_id = *id;
		let child_id = child.id();

		tree.with_mut(|inner| {
			Self::attach_child(inner, parent_id, child_id, |element_content| {
				element_content.children.push_back(child_id);
			});
		});
	}

	pub fn remove_child(&self, child: impl Node) {
		let NodeRef { tree, id } = &self.0;

		tree::with_mut(|inner| Self::detach_child(inner, parent_id, child_id));

		tree.remove_child(*id, child.id());
	}
}

impl Node for Element {
	fn node_ref(&self) -> &NodeRef {
		&self.0
	}
}

#[derive(Debug, Default)]
pub struct ElementContent {
	children: Children,
}

impl ElementContent {
	fn detach_child(&mut self, child_id: NodeId) -> bool {
		let iterate = || {
			for (i, id) in self.children.iter().enumerate() {
				if id == &child_id {
					return Some(i);
				}
			}

			None
		};

		iterate()
			.inspect(|index| {
				self.children.remove(*index);
			})
			.map(|_| true)
			.unwrap_or_default()
	}

	/*pub fn remove_child(&mut self, parent_id: NodeId, child_id: NodeId) {
		let children = self.children.get_mut(parent_id).expect("a valid node id");

		if remove_child(children, child_id) {
			let child = self.node_contents.get_mut(child_id).unwrap();

			child.parent = None;

			self.decrement_ref(child_id);
		}
	}*/
}

/*
#[m8::class]
#[derive(Clone)]
pub struct Element {
	#[m8::weak]
	window: Window,
	node_ref: NodeRef,
}

#[m8::class]
impl Element {}
*/
