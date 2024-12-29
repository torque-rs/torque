use std::any::TypeId;

use crate::{Component, EntityId, EntityRef, Extends, System, SystemError, WeakEntityRef};

pub trait Entity {
	const NAME: &'static str;
	type Base;

	fn type_id() -> TypeId;
	fn type_ids() -> &'static [TypeId];
}

impl Entity for () {
	const NAME: &'static str = "()";

	type Base = Self;

	fn type_id() -> TypeId {
		TypeId::of::<()>()
	}

	fn type_ids() -> &'static [TypeId] {
		&[]
	}
}

pub trait EntityMethods<E>: Sized
where
	E: Entity + 'static,
{
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
	fn downgrade(self) -> WeakEntityRef<E> {
		WeakEntityRef::new(self.system().clone(), self.id())
	}

	#[inline]
	fn upcast<Base>(&self) -> EntityRef<Base>
	where
		Base: Entity + 'static,
		E: Extends<Base>,
	{
		self.system().entity_cast(self.id())
	}

	#[inline]
	fn downcast<Sub>(&self) -> EntityRef<Sub>
	where
		Sub: Entity + 'static,
	{
		self.system().entity_cast(self.id())
	}

	fn try_downcast<Sub>(&self) -> Result<EntityRef<Sub>, SystemError>
	where
		Sub: Entity + 'static,
	{
		self.system().try_entity_cast(self.id())
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
