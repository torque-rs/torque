use std::{any::TypeId, ops::Deref, sync::LazyLock};

use torque_ecs::{Entity, EntityRef};
use torque_style::Style;

use crate::{Element, Parent};

pub trait NodeMethods {
	fn entity_ref(&self) -> &EntityRef;

	fn parent(&self) -> Parent {
		self.entity_ref().get_or_default::<Parent>()
	}

	fn with_style<R>(&self, f: impl FnOnce(&Style) -> R) -> R {
		self.entity_ref().with_or_default(f)
	}

	fn with_style_mut<R>(&self, f: impl FnOnce(&mut Style) -> R) -> R {
		self.entity_ref().with_mut_or_default(f)
	}
}

#[derive(Clone)]
pub struct Node(EntityRef);

impl NodeMethods for Node {
	fn entity_ref(&self) -> &EntityRef {
		&self.0
	}
}

impl Deref for Node {
	type Target = EntityRef;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl From<Element> for Node {
	fn from(value: Element) -> Self {
		Self(value.deref().clone())
	}
}

static TYPE_ID: LazyLock<TypeId> = LazyLock::new(TypeId::of::<Node>);
static TYPE_IDS: LazyLock<[TypeId; 1]> = LazyLock::new(|| [Node::type_id()]);

impl Entity for Node {
	const NAME: &'static str = "Node";

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
