use std::marker::PhantomData;

use crate::{Entity, EntityId, EntityMethods, System};

#[derive(Debug)]
pub struct EntityRef<E>
where
	E: Entity,
{
	pub system: System,
	pub id: EntityId,
	_phantom: PhantomData<E>,
}

impl<E> EntityRef<E>
where
	E: Entity + 'static,
{
	pub(crate) fn new(system: System, id: EntityId) -> Self {
		system.increment_ref(id);

		Self {
			system,
			id,
			_phantom: PhantomData,
		}
	}
}

impl<E> EntityMethods<E> for EntityRef<E>
where
	E: Entity + 'static,
{
	fn system(&self) -> &System {
		&self.system
	}

	fn id(&self) -> EntityId {
		self.id
	}
}

impl<E> Clone for EntityRef<E>
where
	E: Entity,
{
	fn clone(&self) -> Self {
		self.system.increment_ref(self.id);

		Self {
			system: self.system.clone(),
			id: self.id,
			_phantom: PhantomData,
		}
	}
}

impl<E> Drop for EntityRef<E>
where
	E: Entity,
{
	fn drop(&mut self) {
		self.system.decrement_ref(self.id);
	}
}

#[cfg(feature = "v8")]
impl<E> v8::cppgc::GarbageCollected for EntityRef<E>
where
	E: Entity + 'static,
{
	fn trace(&self, _visitor: &v8::cppgc::Visitor) {}

	fn get_name(&self) -> Option<&'static std::ffi::CStr> {
		None
	}
}
