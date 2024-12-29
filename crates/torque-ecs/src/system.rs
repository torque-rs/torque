#[cfg(test)]
mod tests;

use std::{
	any::{Any, TypeId},
	collections::HashMap,
	fmt::Debug,
	ops::Deref,
	sync::{
		atomic::{fence, AtomicUsize, Ordering},
		Arc, Mutex,
	},
};

use fnv::FnvHashMap;
use slotmap::{SecondaryMap, SlotMap};

use crate::{Component, Entity, EntityId, EntityRef, SystemError};

#[derive(Clone, Debug, Default)]
pub struct System(Arc<Inner>);

impl Deref for System {
	type Target = Arc<Inner>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

type ComponentMap = Mutex<SecondaryMap<EntityId, FnvHashMap<TypeId, Box<dyn Any>>>>;

#[derive(Default)]
pub struct Inner {
	ref_counts: Mutex<SlotMap<EntityId, AtomicUsize>>,
	type_ids: Mutex<SecondaryMap<EntityId, &'static [TypeId]>>,
	components: ComponentMap,
}

impl Inner {
	pub fn create<T>(self: &Arc<Self>) -> T
	where
		T: Entity + 'static,
	{
		let entity_id = self.ref_counts.lock().unwrap().insert(AtomicUsize::new(1));

		self
			.type_ids
			.lock()
			.unwrap()
			.insert(entity_id, T::type_ids());

		self
			.components
			.lock()
			.unwrap()
			.insert(entity_id, HashMap::default());

		T::new(EntityRef::new(System(self.clone()), entity_id))
	}

	pub fn entity_set<C>(self: &Arc<Self>, entity_id: EntityId, value: C)
	where
		C: Component + 'static,
	{
		self.try_entity_set(entity_id, value).unwrap()
	}

	pub fn try_entity_set<C>(
		self: &Arc<Self>,
		entity_id: EntityId,
		value: C,
	) -> Result<(), SystemError>
	where
		C: Component + 'static,
	{
		let type_id = TypeId::of::<C>();

		self
			.components
			.lock()
			.unwrap()
			.get_mut(entity_id)
			.ok_or(SystemError::EntityNotFound(entity_id))?
			.insert(type_id, Box::new(value) as Box<dyn Any>);

		Ok(())
	}

	#[inline]
	pub fn entity_with<C, R>(self: &Arc<Self>, entity_id: EntityId, f: impl FnOnce(&C) -> R) -> R
	where
		C: Component + 'static,
	{
		self.try_entity_with(entity_id, f).unwrap()
	}

	#[inline]
	pub fn entity_with_or<C, R>(
		self: &Arc<Self>,
		entity_id: EntityId,
		f: impl FnOnce(&C) -> R,
		init: impl FnOnce() -> C,
	) -> R
	where
		C: Component + 'static,
	{
		self.try_entity_with_or(entity_id, f, init).unwrap()
	}

	#[inline]
	pub fn entity_with_or_default<C, R>(
		self: &Arc<Self>,
		entity_id: EntityId,
		f: impl FnOnce(&C) -> R,
	) -> R
	where
		C: Component + Default + 'static,
	{
		self
			.try_entity_with_or(entity_id, f, <C as Default>::default)
			.unwrap()
	}

	pub fn try_entity_with<C, R>(
		self: &Arc<Self>,
		entity_id: EntityId,
		f: impl FnOnce(&C) -> R,
	) -> Result<R, SystemError>
	where
		C: Component + 'static,
	{
		let type_id = TypeId::of::<C>();

		Ok(f(self
			.components
			.lock()
			.unwrap()
			.get(entity_id)
			.ok_or(SystemError::EntityNotFound(entity_id))?
			.get(&type_id)
			.ok_or(SystemError::ComponentNotFound(entity_id, C::NAME))?
			.downcast_ref::<C>()
			.unwrap_or_else(|| {
				panic!(
					"component {} is of invalid type for entity {}",
					C::NAME,
					entity_id
				)
			})))
	}

	pub fn try_entity_with_or<C, R>(
		self: &Arc<Self>,
		entity_id: EntityId,
		f: impl FnOnce(&C) -> R,
		init: impl FnOnce() -> C,
	) -> Result<R, SystemError>
	where
		C: Component + 'static,
	{
		let type_id = TypeId::of::<C>();

		Ok(f(self
			.components
			.lock()
			.unwrap()
			.get_mut(entity_id)
			.ok_or(SystemError::EntityNotFound(entity_id))?
			.entry(type_id)
			.or_insert_with(|| Box::new(init()) as Box<dyn Any>)
			.downcast_ref::<C>()
			.unwrap_or_else(|| {
				panic!(
					"component {} is of invalid type for entity {}",
					C::NAME,
					entity_id
				)
			})))
	}

	#[inline]
	pub fn try_entity_with_or_default<C, R>(
		self: &Arc<Self>,
		entity_id: EntityId,
		f: impl FnOnce(&C) -> R,
	) -> Result<R, SystemError>
	where
		C: Component + Default + 'static,
	{
		self.try_entity_with_or(entity_id, f, <C as Default>::default)
	}

	#[inline]
	pub fn entity_with_mut<C, R>(
		self: &Arc<Self>,
		entity_id: EntityId,
		f: impl FnOnce(&mut C) -> R,
	) -> R
	where
		C: Component + 'static,
	{
		self.try_entity_with_mut(entity_id, f).unwrap()
	}

	#[inline]
	pub fn entity_with_mut_or<C, R>(
		self: &Arc<Self>,
		entity_id: EntityId,
		f: impl FnOnce(&mut C) -> R,
		init: impl FnOnce() -> C,
	) -> R
	where
		C: Component + 'static,
	{
		self.try_entity_with_mut_or(entity_id, f, init).unwrap()
	}

	#[inline]
	pub fn entity_with_mut_or_default<C, R>(
		self: &Arc<Self>,
		entity_id: EntityId,
		f: impl FnOnce(&mut C) -> R,
	) -> R
	where
		C: Component + Default + 'static,
	{
		self.try_entity_with_mut_or_default(entity_id, f).unwrap()
	}

	pub fn try_entity_with_mut<C, R>(
		self: &Arc<Self>,
		entity_id: EntityId,
		f: impl FnOnce(&mut C) -> R,
	) -> Result<R, SystemError>
	where
		C: Component + 'static,
	{
		let type_id = TypeId::of::<C>();

		Ok(f(self
			.components
			.lock()
			.unwrap()
			.get_mut(entity_id)
			.ok_or(SystemError::EntityNotFound(entity_id))?
			.get_mut(&type_id)
			.ok_or(SystemError::ComponentNotFound(entity_id, C::NAME))?
			.downcast_mut::<C>()
			.unwrap_or_else(|| {
				panic!(
					"component {} is of invalid type for entity {}",
					C::NAME,
					entity_id
				)
			})))
	}

	pub fn try_entity_with_mut_or<C, R>(
		self: &Arc<Self>,
		entity_id: EntityId,
		f: impl FnOnce(&mut C) -> R,
		init: impl FnOnce() -> C,
	) -> Result<R, SystemError>
	where
		C: Component + 'static,
	{
		let type_id = TypeId::of::<C>();

		Ok(f(self
			.components
			.lock()
			.unwrap()
			.get_mut(entity_id)
			.ok_or(SystemError::EntityNotFound(entity_id))?
			.entry(type_id)
			.or_insert_with(|| Box::new(init()) as Box<dyn Any>)
			.downcast_mut::<C>()
			.unwrap_or_else(|| {
				panic!(
					"component {} is of invalid type for entity {}",
					C::NAME,
					entity_id
				)
			})))
	}

	#[inline]
	pub fn try_entity_with_mut_or_default<C, R>(
		self: &Arc<Self>,
		entity_id: EntityId,
		f: impl FnOnce(&mut C) -> R,
	) -> Result<R, SystemError>
	where
		C: Component + Default + 'static,
	{
		self.try_entity_with_mut_or(entity_id, f, <C as Default>::default)
	}

	#[inline]
	pub fn entity_cast<E>(self: &Arc<Self>, entity_id: EntityId) -> E
	where
		E: Entity + 'static,
	{
		self.try_entity_cast::<E>(entity_id).unwrap()
	}

	pub(crate) fn try_entity_cast<E>(self: &Arc<Self>, entity_id: EntityId) -> Result<E, SystemError>
	where
		E: Entity + 'static,
	{
		let type_id = TypeId::of::<E>();
		let type_ids = {
			*self
				.type_ids
				.lock()
				.unwrap()
				.get(entity_id)
				.ok_or(SystemError::EntityNotFound(entity_id))?
		};

		for target_type_id in type_ids {
			if target_type_id == &type_id {
				return Ok(E::new(crate::EntityRef::new(
					System(self.clone()),
					entity_id,
				)));
			}
		}

		Err(SystemError::InvalidCast(entity_id, E::NAME))
	}

	#[inline]
	pub fn entity_is<E>(self: &Arc<Self>, entity_id: EntityId) -> bool
	where
		E: Entity + 'static,
	{
		self.try_entity_is::<E>(entity_id).unwrap()
	}

	pub(crate) fn try_entity_is<E>(self: &Arc<Self>, entity_id: EntityId) -> Result<bool, SystemError>
	where
		E: Entity + 'static,
	{
		let type_id = TypeId::of::<E>();

		Ok(
			self
				.type_ids
				.lock()
				.unwrap()
				.get(entity_id)
				.ok_or(SystemError::EntityNotFound(entity_id))?
				.contains(&type_id),
		)
	}

	pub(crate) fn increment_ref(self: &Arc<Self>, entity_id: EntityId) {
		log::trace!("increment_ref: {}", entity_id);
		log::debug!("entity count: {}", self.ref_counts.lock().unwrap().len());

		let ref_count = self
			.ref_counts
			.lock()
			.unwrap()
			.get(entity_id)
			.unwrap_or_else(|| panic!("increment_ref called for non existent entity {}", entity_id))
			.fetch_add(1, Ordering::Relaxed);

		log::debug!("ref_count: {}", ref_count + 1);
	}

	pub(crate) fn decrement_ref(self: &Arc<Self>, entity_id: EntityId) {
		log::trace!("decrement_ref: {}", entity_id);

		let mut ref_counts = self.ref_counts.lock().unwrap();
		let ref_count = ref_counts
			.get(entity_id)
			.expect("increment_ref should be called before decrement_ref");

		if ref_count.fetch_sub(1, Ordering::Release) == 0 {
			log::debug!("disposing: {entity_id}");

			fence(Ordering::Acquire);

			if let Some(mut components) = self.components.lock().unwrap().remove(entity_id) {
				for (type_id, boxed) in components.drain() {
					// TODO: cleanup
				}
			}

			self.type_ids.lock().unwrap().remove(entity_id);
			self.ref_counts.lock().unwrap().remove(entity_id);
		}
	}
}

impl Debug for Inner {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Inner")
			.field("ref_counts", &self.ref_counts)
			.finish()
	}
}
