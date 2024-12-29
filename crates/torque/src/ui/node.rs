use std::sync::atomic::AtomicUsize;

use crate::style::Style;

use super::{NodeId, NodeRef};

pub trait Node {
	fn node_ref(&self) -> &NodeRef;

	fn id(&self) -> NodeId {
		self.node_ref().id
	}

	fn is_text(&self) -> bool {
		let NodeRef { tree, id } = self.node_ref();

		tree.with_node_content(*id, |content| matches!(content.kind, NodeKind::Text))
	}

	fn is_image(&self) -> bool {
		let NodeRef { tree, id } = self.node_ref();

		tree.with_node_content(*id, |content| matches!(content.kind, NodeKind::Image))
	}

	fn is_element(&self) -> bool {
		let NodeRef { tree, id } = self.node_ref();

		tree.with_node_content(*id, |content| matches!(content.kind, NodeKind::Element))
	}

	fn with_style_mut<R>(&self, f: impl FnOnce(&mut Style) -> R) -> R {
		let NodeRef { tree, id } = self.node_ref();

		tree.with_node_content_mut(*id, |node_content| f(&mut node_content.style))
	}
}

#[derive(Debug)]
pub enum NodeKind {
	Text,
	Image,
	Element,
}

#[derive(Debug)]
pub struct NodeContent {
	pub id: NodeId,
	pub kind: NodeKind,
	pub ref_count: AtomicUsize,
	pub parent: Option<NodeRef>,
	pub style: Style,
	pub unrounded_layout: taffy::Layout,
	pub final_layout: taffy::Layout,
}

impl NodeContent {
	pub fn get_label(&self) -> &'static str {
		match self.kind {
			NodeKind::Text => "Text",
			NodeKind::Image => "Image",
			NodeKind::Element => "Element",
		}
	}
}
