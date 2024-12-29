use std::marker::PhantomData;

use crate::{Component, Entity, EntityId, System, SystemError, WeakEntityRef};

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

	pub fn downgrade(self) -> WeakEntityRef<E> {
		WeakEntityRef::new(self.system.clone(), self.id)
	}
}

pub trait EntityRefMethods {
	fn system(&self) -> &System;

	fn id(&self) -> EntityId;

	#[inline]
	fn get_or_default<C>(&self) -> C::Value
	where
		C: Component + 'static,
		C::Value: Clone + Default,
	{
		self.with_or_default::<C, _>(|component| component.clone())
	}

	#[inline]
	fn get<C>(&self) -> C::Value
	where
		C: Component + 'static,
		C::Value: Clone,
	{
		self.with::<C, _>(|component| component.clone())
	}

	#[inline]
	fn set<C>(&self, value: C::Value) -> &Self
	where
		C: Component + 'static,
	{
		self.system().entity_set::<C>(self.id(), value);

		self
	}

	#[inline]
	fn with<C, R>(&self, f: impl FnOnce(&C::Value) -> R) -> R
	where
		C: Component + 'static,
	{
		self.system().entity_with::<C, R>(self.id(), f)
	}

	#[inline]
	fn with_or<C, R>(&self, f: impl FnOnce(&C::Value) -> R, init: impl FnOnce() -> C::Value) -> R
	where
		C: Component + 'static,
	{
		self.system().entity_with_or::<C, _>(self.id(), f, init)
	}

	#[inline]
	fn with_or_default<C, R>(&self, f: impl FnOnce(&C::Value) -> R) -> R
	where
		C: Component + 'static,
		C::Value: Default,
	{
		self.system().entity_with_or_default::<C, _>(self.id(), f)
	}

	#[inline]
	fn with_mut<C, R>(&self, f: impl FnOnce(&mut C::Value) -> R) -> R
	where
		C: Component + 'static,
	{
		self.system().entity_with_mut::<C, _>(self.id(), f)
	}

	#[inline]
	fn with_mut_or<C, R>(
		&self,
		f: impl FnOnce(&mut C::Value) -> R,
		init: impl FnOnce() -> C::Value,
	) -> R
	where
		C: Component + 'static,
	{
		self.system().entity_with_mut_or::<C, _>(self.id(), f, init)
	}

	#[inline]
	fn with_mut_or_default<C, R>(&self, f: impl FnOnce(&mut C::Value) -> R) -> R
	where
		C: Component + 'static,
		C::Value: Default,
	{
		self
			.system()
			.entity_with_mut_or_default::<C, _>(self.id(), f)
	}

	#[inline]
	fn try_with<C, R>(&self, f: impl FnOnce(&C::Value) -> R) -> Result<R, SystemError>
	where
		C: Component + 'static,
	{
		self.system().try_entity_with::<C, _>(self.id(), f)
	}

	#[inline]
	fn try_with_mut<C, R>(&self, f: impl FnOnce(&mut C::Value) -> R) -> Result<R, SystemError>
	where
		C: Component + 'static,
	{
		self.system().try_entity_with_mut::<C, _>(self.id(), f)
	}

	#[inline]
	fn cast<E2>(&self) -> EntityRef<E2>
	where
		E2: Entity + 'static,
	{
		self.system().entity_cast::<E2>(self.id())
	}

	#[inline]
	fn try_cast<E2>(&self) -> Result<EntityRef<E2>, SystemError>
	where
		E2: Entity + 'static,
	{
		self.system().try_entity_cast::<E2>(self.id())
	}

	#[inline]
	fn is<E2>(&self) -> bool
	where
		E2: Entity + 'static,
	{
		self.system().entity_is::<E2>(self.id())
	}

	#[inline]
	fn try_is<E2>(&self) -> Result<bool, SystemError>
	where
		E2: Entity + 'static,
	{
		self.system().try_entity_is::<E2>(self.id())
	}
}

impl<E> EntityRefMethods for EntityRef<E>
where
	E: Entity,
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
