use std::any::TypeId;

pub trait Entity {
	const NAME: &'static str;

	fn type_id() -> TypeId;
	fn type_ids() -> &'static [TypeId];
}
