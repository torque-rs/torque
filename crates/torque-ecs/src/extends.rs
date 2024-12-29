use crate::{Entity, EntityRef};

pub trait Cast<E>: Sized + Entity
where
	E: Entity + 'static,
{
	fn cast(value: EntityRef<Self>) -> EntityRef<E>;
}

pub trait Extends<E>: Entity {}

impl<EBase, E> Cast<E> for EBase
where
	EBase: Entity + 'static,
	E: Entity + Extends<EBase> + 'static,
{
	fn cast(value: EntityRef<Self>) -> EntityRef<E> {
		value.system.entity_cast::<E>(value.id)
	}
}
