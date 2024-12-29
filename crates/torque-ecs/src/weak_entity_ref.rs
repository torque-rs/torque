use std::marker::PhantomData;

use crate::{Entity, EntityId, EntityRef, System};

#[derive(Clone)]
pub struct WeakEntityRef<E>
where
	E: Entity,
{
	system: System,
	id: EntityId,
	_phantom: PhantomData<E>,
}

impl<E> WeakEntityRef<E>
where
	E: Entity + 'static,
{
	pub(crate) fn new(system: System, id: EntityId) -> Self {
		Self {
			system,
			id,
			_phantom: PhantomData,
		}
	}

	pub fn upgrade(&self) -> Option<EntityRef<E>> {
		self.system.get(self.id)
	}
}
