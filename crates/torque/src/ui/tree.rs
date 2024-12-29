use std::{
	collections::vec_deque::Iter,
	sync::{
		atomic::{fence, AtomicUsize, Ordering},
		Arc, Mutex,
	},
};

use slotmap::{SecondaryMap, SlotMap, SparseSecondaryMap};

use crate::style::{Layout, Style};

use super::{
	Children, Element, ElementContent, ImageContent, NodeContent, NodeId, NodeKind, NodeRef, Text,
	TextContent,
};

#[derive(Clone, Debug)]
pub struct Tree(Arc<Mutex<Inner>>);

impl Tree {
	pub fn new(font_system: cosmic_text::FontSystem) -> Self {
		Self(Arc::new(Mutex::new(Inner::new(font_system))))
	}

	fn create_node(&self, kind: NodeKind) -> NodeId {
		self
			.0
			.lock()
			.unwrap()
			.node_contents
			.insert_with_key(|id| NodeContent {
				id,
				kind,
				ref_count: AtomicUsize::new(1),
				parent: None,
				style: Default::default(),
				unrounded_layout: Default::default(),
				final_layout: Default::default(),
			})
	}

	pub fn create_text(&self, text: impl AsRef<str>) -> Text {
		let id = self.create_node(NodeKind::Text);

		self.0.lock().unwrap().text_contents.insert(
			id,
			TextContent::new(
				text,
				cosmic_text::Metrics {
					font_size: 16.0,
					line_height: 16.0,
				},
				cosmic_text::AttrsOwned {
					color_opt: None,
					family_owned: cosmic_text::FamilyOwned::SansSerif,
					stretch: cosmic_text::Stretch::Normal,
					style: cosmic_text::Style::Normal,
					weight: cosmic_text::Weight(100),
					metadata: 0,
					cache_key_flags: cosmic_text::CacheKeyFlags::all(),
					metrics_opt: None,
				},
			),
		);

		Text(NodeRef {
			tree: self.clone(),
			id,
		})
	}
	pub fn create_element(&self) -> Element {
		let id = self.create_node(NodeKind::Element);

		Element(NodeRef {
			tree: self.clone(),
			id,
		})
	}

	pub(crate) fn with_node_content<R>(
		&self,
		node_id: NodeId,
		f: impl FnOnce(&NodeContent) -> R,
	) -> R {
		f(self
			.0
			.lock()
			.unwrap()
			.node_contents
			.get(node_id)
			.expect("valid node id"))
	}

	pub(crate) fn with_node_content_mut<R>(
		&self,
		node_id: NodeId,
		f: impl FnOnce(&mut NodeContent) -> R,
	) -> R {
		f(self
			.0
			.lock()
			.unwrap()
			.node_contents
			.get_mut(node_id)
			.expect("valid node id"))
	}

	pub(crate) fn with_mut<R>(&self, f: impl FnOnce(&mut Inner) -> R) -> R {
		f(&mut *self.0.lock().unwrap())
	}

	pub(crate) fn prepend_child(&self, parent_id: NodeId, child_id: NodeId) {
		self.0.lock().unwrap().prepend_child(parent_id, child_id);
	}

	pub(crate) fn append_child(&self, parent_id: NodeId, child_id: NodeId) {
		self.0.lock().unwrap().append_child(parent_id, child_id);
	}

	pub(crate) fn remove_child(&self, parent_id: NodeId, child_id: NodeId) {
		self.0.lock().unwrap().remove_child(parent_id, child_id);
	}

	pub(crate) fn increment_ref(&self, node_id: NodeId) {
		self.0.lock().unwrap().increment_ref(node_id);
	}

	pub(crate) fn decrement_ref(&mut self, node_id: NodeId) {
		self.0.lock().unwrap().decrement_ref(node_id);
	}

	pub fn compute_layout(
		&mut self,
		root: NodeId,
		available_space: taffy::Size<taffy::AvailableSpace>,
		use_rounding: bool,
	) {
		let inner = &mut *self.0.lock().unwrap();

		taffy::compute_root_layout(inner, root.into(), available_space);

		if use_rounding {
			taffy::round_layout(inner, root.into())
		}
	}

	pub fn print(&self, root: NodeId) {
		let inner = &mut *self.0.lock().unwrap();

		for (k, v) in inner.node_contents.iter() {
			log::trace!("{:?}: {:?}", k, v.kind);
		}

		taffy::print_tree(inner, root.into())
	}
}

#[derive(Debug)]
pub(crate) struct Inner {
	pub(crate) node_contents: SlotMap<NodeId, NodeContent>,
	pub(crate) text_contents: SparseSecondaryMap<NodeId, TextContent>,
	pub(crate) image_contents: SparseSecondaryMap<NodeId, ImageContent>,
	pub(crate) element_contents: SparseSecondaryMap<NodeId, ElementContent>,
	pub(crate) children: SparseSecondaryMap<NodeId, Children>,
	pub(crate) caches: SecondaryMap<NodeId, taffy::Cache>,
	pub(crate) font_system: cosmic_text::FontSystem,
	pub(crate) default_style: Style,
}

impl Inner {
	pub fn new(font_system: cosmic_text::FontSystem) -> Self {
		Self {
			node_contents: Default::default(),
			text_contents: Default::default(),
			image_contents: Default::default(),
			element_contents: Default::default(),
			children: Default::default(),
			caches: Default::default(),
			font_system,
			default_style: Default::default(),
		}
	}

	fn attach_child(&mut self, parent_id: NodeId, child_id: NodeId, f: impl FnOnce(&mut Children)) {
		let increment = {
			let Self {
				node_contents,
				children,
				..
			} = self;

			let child = node_contents.get_mut(child_id).unwrap();

			let increment = if let Some(old_parent_id) = child.parent.take() {
				!remove_child(children.get_mut(old_parent_id).unwrap(), child_id)
			} else {
				true
			};

			f(children.entry(parent_id).unwrap().or_default());

			child.parent = Some(parent_id);

			increment
		};

		if increment {
			self.increment_ref(child_id);
		}
	}

	pub fn prepend_child(&mut self, parent_id: NodeId, child_id: NodeId) {
		self.attach_child(parent_id, child_id, |children| {
			children.push_front(child_id);
		});
	}

	pub fn append_child(&mut self, parent_id: NodeId, child_id: NodeId) {
		self.attach_child(parent_id, child_id, |children| {
			children.push_back(child_id);
		})
	}

	pub fn remove_child(&mut self, parent_id: NodeId, child_id: NodeId) {
		let children = self.children.get_mut(parent_id).expect("a valid node id");

		if remove_child(children, child_id) {
			let child = self.node_contents.get_mut(child_id).unwrap();

			child.parent = None;

			self.decrement_ref(child_id);
		}
	}

	pub fn increment_ref(&self, node_id: NodeId) {
		if let Some(node) = self.node_contents.get(node_id) {
			node.ref_count.fetch_add(1, Ordering::Relaxed);
		}
	}

	pub fn decrement_ref(&mut self, node_id: NodeId) {
		if let Some(node) = self.node_contents.get(node_id) {
			if node.ref_count.fetch_sub(1, Ordering::Release) == 1 {
				fence(Ordering::Acquire);

				match node.kind {
					NodeKind::Text => {
						self.text_contents.remove(node_id);
					}
					NodeKind::Image => {
						self.image_contents.remove(node_id);
					}
					NodeKind::Element => {
						self.element_contents.remove(node_id);
					}
				}

				self.node_contents.remove(node_id);
			}
		}
	}
}

fn remove_child(children: &mut Children, child_id: NodeId) -> bool {
	let iterate = || {
		for (i, id) in children.iter().enumerate() {
			if id == &child_id {
				return Some(i);
			}
		}

		None
	};

	iterate()
		.inspect(|index| {
			children.remove(*index);
		})
		.map(|_| true)
		.unwrap_or_default()
}

pub struct ChildIter<'a>(Option<Iter<'a, NodeId>>);

impl Iterator for ChildIter<'_> {
	type Item = taffy::NodeId;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.as_mut().and_then(|v| v.next().map(|v| (*v).into()))
	}
}

impl taffy::TraversePartialTree for Inner {
	type ChildIter<'a>
		= ChildIter<'a>
	where
		Self: 'a;

	fn child_ids(&self, parent_node_id: taffy::NodeId) -> Self::ChildIter<'_> {
		ChildIter(
			self
				.children
				.get(parent_node_id.into())
				.map(|v| Some(v.iter()))
				.unwrap_or(None),
		)
	}

	fn child_count(&self, parent_node_id: taffy::NodeId) -> usize {
		self
			.children
			.get(parent_node_id.into())
			.map(|v| v.len())
			.unwrap_or(0)
	}

	fn get_child_id(&self, parent_node_id: taffy::NodeId, child_index: usize) -> taffy::NodeId {
		self
			.children
			.get(parent_node_id.into())
			.map(|v| v[child_index])
			.unwrap()
			.into()
	}
}

impl taffy::TraverseTree for Inner {}

impl taffy::RoundTree for Inner {
	fn get_unrounded_layout(&self, node_id: taffy::NodeId) -> &taffy::Layout {
		let node = self.node_contents.get(node_id.into()).unwrap();

		&node.unrounded_layout
	}

	fn set_final_layout(&mut self, node_id: taffy::NodeId, layout: &taffy::Layout) {
		let node = self.node_contents.get_mut(node_id.into()).unwrap();

		node.final_layout = *layout;
	}
}

impl taffy::LayoutPartialTree for Inner {
	type CoreContainerStyle<'a>
		= &'a Style
	where
		Self: 'a;

	fn get_core_container_style(&self, node_id: taffy::NodeId) -> Self::CoreContainerStyle<'_> {
		self
			.node_contents
			.get(node_id.into())
			.map(|v| &v.style)
			.unwrap_or(&self.default_style)
	}

	fn set_unrounded_layout(&mut self, node_id: taffy::NodeId, layout: &taffy::Layout) {
		let node = self.node_contents.get_mut(node_id.into()).unwrap();

		node.unrounded_layout = *layout;
	}

	fn compute_child_layout(
		&mut self,
		node_id: taffy::NodeId,
		inputs: taffy::LayoutInput,
	) -> taffy::LayoutOutput {
		taffy::compute_cached_layout(self, node_id, inputs, |tree, node_id, inputs| {
			//let tree = self;
			let node = tree.node_contents.get(node_id.into()).expect("valid node");

			match node.kind {
				NodeKind::Text => {
					let text_content = tree
						.text_contents
						.get_mut(node_id.into())
						.expect("valid text node");

					taffy::compute_leaf_layout(inputs, &node.style, |known_dimensions, available_space| {
						text_content.measure(
							known_dimensions,
							available_space,
							node,
							&mut tree.font_system,
						)
					})
				}
				NodeKind::Image => {
					let image_content = tree
						.image_contents
						.get_mut(node_id.into())
						.expect("valid image node");

					taffy::compute_leaf_layout(inputs, &node.style, |known_dimensions, available_space| {
						image_content.measure(known_dimensions, available_space)
					})
				}
				NodeKind::Element => match node.style.get_or_default::<Layout>() {
					Layout::Block => taffy::compute_block_layout(tree, node_id, inputs),
					Layout::Grid => todo!(),
					Layout::FlexBox => taffy::compute_flexbox_layout(tree, node_id, inputs),
				},
			}
		})
	}
}

impl taffy::LayoutBlockContainer for Inner {
	type BlockContainerStyle<'a>
		= &'a Style
	where
		Self: 'a;

	type BlockItemStyle<'a>
		= &'a Style
	where
		Self: 'a;

	fn get_block_container_style(&self, node_id: taffy::NodeId) -> Self::BlockContainerStyle<'_> {
		self
			.node_contents
			.get(node_id.into())
			.map(|v| &v.style)
			.unwrap_or(&self.default_style)
	}

	fn get_block_child_style(&self, child_node_id: taffy::NodeId) -> Self::BlockItemStyle<'_> {
		self
			.node_contents
			.get(child_node_id.into())
			.map(|v| &v.style)
			.unwrap_or(&self.default_style)
	}
}

impl taffy::LayoutFlexboxContainer for Inner {
	type FlexboxContainerStyle<'a>
		= &'a Style
	where
		Self: 'a;

	type FlexboxItemStyle<'a>
		= &'a Style
	where
		Self: 'a;

	fn get_flexbox_container_style(&self, node_id: taffy::NodeId) -> Self::FlexboxContainerStyle<'_> {
		self
			.node_contents
			.get(node_id.into())
			.map(|v| &v.style)
			.unwrap_or(&self.default_style)
	}

	fn get_flexbox_child_style(&self, child_node_id: taffy::NodeId) -> Self::FlexboxItemStyle<'_> {
		self
			.node_contents
			.get(child_node_id.into())
			.map(|v| &v.style)
			.unwrap_or(&self.default_style)
	}
}

impl taffy::CacheTree for Inner {
	fn cache_get(
		&self,
		node_id: taffy::NodeId,
		known_dimensions: taffy::Size<Option<f32>>,
		available_space: taffy::Size<taffy::AvailableSpace>,
		run_mode: taffy::RunMode,
	) -> Option<taffy::LayoutOutput> {
		self
			.caches
			.get(node_id.into())
			.and_then(|v| v.get(known_dimensions, available_space, run_mode))
	}

	fn cache_store(
		&mut self,
		node_id: taffy::NodeId,
		known_dimensions: taffy::Size<Option<f32>>,
		available_space: taffy::Size<taffy::AvailableSpace>,
		run_mode: taffy::RunMode,
		layout_output: taffy::LayoutOutput,
	) {
		self
			.caches
			.entry(node_id.into())
			.unwrap()
			.or_default()
			.store(known_dimensions, available_space, run_mode, layout_output);
	}

	fn cache_clear(&mut self, node_id: taffy::NodeId) {
		if let Some(cache) = self.caches.get_mut(node_id.into()) {
			cache.clear()
		}
	}
}

impl taffy::PrintTree for Inner {
	fn get_debug_label(&self, node_id: taffy::NodeId) -> &'static str {
		self
			.node_contents
			.get(node_id.into())
			.expect("valid node id")
			.get_label()
	}

	fn get_final_layout(&self, node_id: taffy::NodeId) -> &taffy::Layout {
		&self
			.node_contents
			.get(node_id.into())
			.expect("valid node id")
			.final_layout
	}
}
