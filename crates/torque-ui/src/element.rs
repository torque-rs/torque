#[cfg(test)]
mod tests;

use std::collections::VecDeque;

use torque_ecs::{Entity, EntityMethods, EntityRef};

use crate::{Children, Node, NodeMethods, Parent};

pub trait ElementMethods<E>: NodeMethods<E>
where
	E: Entity + 'static,
{
	fn element_self(&self) -> EntityRef<Element>;

	fn with_children<R>(&self, f: impl FnOnce(&VecDeque<EntityRef<Node>>) -> R) -> R {
		self.with_or_default::<Children, _>(f)
	}

	fn with_children_mut<R>(&self, f: impl FnOnce(&mut VecDeque<EntityRef<Node>>) -> R) -> R {
		self.with_mut_or_default::<Children, _>(f)
	}

	fn prepend_child(&self, child: EntityRef<Node>) {
		if let Some(parent) = child.parent().as_ref() {
			if let Some(parent) = parent.upgrade() {
				parent.remove_child(child.clone())
			}
		}

		self.with_children_mut(|children| children.push_front(child.clone()));

		child.set::<Parent>(self.element_self().downgrade().into());
	}

	fn append_child(&self, child: EntityRef<Node>) {
		if let Some(parent) = child.parent().as_ref() {
			if let Some(parent) = parent.upgrade() {
				parent.remove_child(child.clone())
			}
		}

		self.with_children_mut(|children| children.push_back(child.clone()));

		child.set::<Parent>(self.element_self().downgrade().into());
	}

	fn remove_child(&self, child: EntityRef<Node>) {
		child.set::<Parent>(None);

		self.with_children_mut(|children| children.retain(|node| child.id == node.id));
	}
}

#[derive(Entity)]
#[extends(Node)]
pub struct Element;

impl NodeMethods<Element> for EntityRef<Element> {}

impl ElementMethods<Element> for EntityRef<Element> {
	fn element_self(&self) -> EntityRef<Element> {
		self.clone()
	}
}
