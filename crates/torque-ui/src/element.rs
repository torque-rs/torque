#[cfg(test)]
mod tests;

use std::{any::TypeId, ops::Deref, sync::LazyLock};

use torque_ecs::{Entity, EntityRef};

use crate::{Children, Node, NodeMethods, Parent};

pub trait ElementMethods: NodeMethods {
	fn element_self(&self) -> Element;

	fn with_children<R>(&self, f: impl FnOnce(&Children) -> R) -> R {
		self.entity_ref().with_or_default(f)
	}

	fn with_children_mut<R>(&self, f: impl FnOnce(&mut Children) -> R) -> R {
		self.entity_ref().with_mut_or_default(f)
	}

	fn prepend_child(&self, child: impl Into<Node>) {
		let child = child.into();

		if let Some(v) = child.parent().as_ref() {
			v.remove_child(child.clone())
		}

		self.with_children_mut(|children| children.push_front(child.clone()));

		child.set::<Parent>(self.element_self().into());
	}

	fn append_child(&self, child: impl Into<Node>) {
		let child = child.into();

		if let Some(v) = child.parent().as_ref() {
			v.remove_child(child.clone())
		}

		self.with_children_mut(|children| children.push_back(child.clone()));

		child.set::<Parent>(self.element_self().into());
	}

	fn remove_child(&self, child: impl Into<Node>) {
		let child = child.into();

		child.set::<Parent>(None.into());

		self.with_children_mut(|children| children.retain(|node| child.id == node.id));
	}
}

#[derive(Clone)]
pub struct Element(EntityRef);

impl NodeMethods for Element {
	fn entity_ref(&self) -> &EntityRef {
		&self.0
	}
}

impl ElementMethods for Element {
	fn element_self(&self) -> Element {
		self.clone()
	}
}

impl Deref for Element {
	type Target = EntityRef;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

static TYPE_ID: LazyLock<TypeId> = LazyLock::new(TypeId::of::<Element>);
static TYPE_IDS: LazyLock<[TypeId; 2]> = LazyLock::new(|| [Node::type_id(), Element::type_id()]);

impl Entity for Element {
	const NAME: &'static str = "Element";

	fn type_id() -> TypeId {
		*TYPE_ID
	}

	fn type_ids() -> &'static [TypeId] {
		&*TYPE_IDS
	}

	fn new(entity_ref: EntityRef) -> Self {
		Self(entity_ref)
	}
}
