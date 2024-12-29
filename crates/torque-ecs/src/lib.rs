mod component;
mod entity;
mod entity_id;
mod entity_ref;
mod extends;
mod system;
mod system_error;
mod weak_entity_ref;

pub use self::{
	component::Component,
	entity::{Entity, EntityMethods},
	entity_id::EntityId,
	entity_ref::EntityRef,
	extends::{Cast, Extends},
	system::System,
	system_error::SystemError,
	weak_entity_ref::WeakEntityRef,
};

pub use torque_ecs_macros::{Component, Entity};
