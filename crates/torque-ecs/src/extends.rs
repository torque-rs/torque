use crate::Entity;

pub trait Extends<E>: Entity
where
	E: Entity,
{
}
