use std::any::TypeId;

use crate::EntityRef;

pub trait Entity {
	const NAME: &'static str;

	fn type_id() -> TypeId;
	fn type_ids() -> &'static [TypeId];
	fn new(entity_ref: EntityRef) -> Self;
}
