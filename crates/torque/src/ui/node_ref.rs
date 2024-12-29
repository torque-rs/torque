use super::{NodeId, Tree};

#[derive(Debug)]
pub struct NodeRef {
	pub(crate) tree: Tree,
	pub(crate) id: NodeId,
}

impl NodeRef {}

impl Clone for NodeRef {
	fn clone(&self) -> Self {
		self.tree.increment_ref(self.id);

		Self {
			tree: self.tree.clone(),
			id: self.id,
		}
	}
}

impl Drop for NodeRef {
	fn drop(&mut self) {
		self.tree.decrement_ref(self.id);
	}
}
