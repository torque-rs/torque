use crate::{Component, Entity, EntityId, System, SystemError};

#[derive(Debug)]
pub struct EntityRef {
	pub system: System,
	pub id: EntityId,
}

impl EntityRef {
	pub fn new(system: System, id: EntityId) -> Self {
		Self { system, id }
	}

	#[inline]
	pub fn get_or_default<C>(&self) -> C
	where
		C: Component + Default + Clone + 'static,
	{
		self.with_or_default::<C, C>(|component| component.clone())
	}

	#[inline]
	pub fn get<C>(&self) -> C
	where
		C: Component + Clone + 'static,
	{
		self.with::<C, C>(|component| component.clone())
	}

	#[inline]
	pub fn set<C>(&self, value: C) -> &Self
	where
		C: Component + 'static,
	{
		self.system.entity_set(self.id, value);

		self
	}

	#[inline]
	pub fn with<C, R>(&self, f: impl FnOnce(&C) -> R) -> R
	where
		C: Component + 'static,
	{
		self.system.entity_with::<C, R>(self.id, f)
	}

	#[inline]
	pub fn with_or<C, R>(&self, f: impl FnOnce(&C) -> R, init: impl FnOnce() -> C) -> R
	where
		C: Component + 'static,
	{
		self.system.entity_with_or(self.id, f, init)
	}

	#[inline]
	pub fn with_or_default<C, R>(&self, f: impl FnOnce(&C) -> R) -> R
	where
		C: Component + Default + 'static,
	{
		self.system.entity_with_or_default(self.id, f)
	}

	#[inline]
	pub fn with_mut<C, R>(&self, f: impl FnOnce(&mut C) -> R) -> R
	where
		C: Component + 'static,
	{
		self.system.entity_with_mut(self.id, f)
	}

	#[inline]
	pub fn with_mut_or<C, R>(&self, f: impl FnOnce(&mut C) -> R, init: impl FnOnce() -> C) -> R
	where
		C: Component + 'static,
	{
		self.system.entity_with_mut_or(self.id, f, init)
	}

	#[inline]
	pub fn with_mut_or_default<C, R>(&self, f: impl FnOnce(&mut C) -> R) -> R
	where
		C: Component + Default + 'static,
	{
		self.system.entity_with_mut_or_default(self.id, f)
	}

	#[inline]
	pub fn try_with<C, R>(&self, f: impl FnOnce(&C) -> R) -> Result<R, SystemError>
	where
		C: Component + 'static,
	{
		self.system.try_entity_with(self.id, f)
	}

	#[inline]
	pub fn try_with_mut<C, R>(&self, f: impl FnOnce(&mut C) -> R) -> Result<R, SystemError>
	where
		C: Component + 'static,
	{
		self.system.try_entity_with_mut(self.id, f)
	}

	#[inline]
	pub fn cast<E>(&self) -> E
	where
		E: Entity + 'static,
	{
		self.system.entity_cast::<E>(self.id)
	}

	#[inline]
	pub fn try_cast<E>(&self) -> Result<E, SystemError>
	where
		E: Entity + 'static,
	{
		self.system.try_entity_cast::<E>(self.id)
	}

	#[inline]
	pub fn is<E>(&self) -> bool
	where
		E: Entity + 'static,
	{
		self.system.entity_is::<E>(self.id)
	}

	#[inline]
	pub fn try_is<E>(&self) -> Result<bool, SystemError>
	where
		E: Entity + 'static,
	{
		self.system.try_entity_is::<E>(self.id)
	}
}

impl Clone for EntityRef {
	fn clone(&self) -> Self {
		self.system.increment_ref(self.id);

		Self {
			system: self.system.clone(),
			id: self.id,
		}
	}
}

impl Drop for EntityRef {
	fn drop(&mut self) {
		self.system.decrement_ref(self.id);
	}
}
