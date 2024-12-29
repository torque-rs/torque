use std::any::TypeId;

use crate::{Component, EntityId, EntityRef, System, SystemError};

pub trait Entity {
	const NAME: &'static str;

	fn type_id() -> TypeId;
	fn type_ids() -> &'static [TypeId];
}

pub trait EntityMethods {
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
