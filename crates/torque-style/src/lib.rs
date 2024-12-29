pub mod core;

mod property;

use std::any::{Any, TypeId};

use fnv::FnvHashMap;
use torque_ecs::Component;

pub use self::{core::*, property::Property};

#[derive(Debug, Default)]
pub struct Style {
	values: FnvHashMap<TypeId, Box<dyn Any + Send>>,
}

impl Style {
	pub fn get<P>(&self) -> Option<P>
	where
		P: Property + 'static,
	{
		let type_id = TypeId::of::<P>();

		self
			.values
			.get(&type_id)
			.and_then(|v| v.downcast_ref())
			.cloned()
	}

	pub fn get_or<P>(&self, default: P) -> P
	where
		P: Property + 'static,
	{
		self.get::<P>().unwrap_or(default)
	}

	pub fn get_or_default<P>(&self) -> P
	where
		P: Property + Default + 'static,
	{
		self.get::<P>().unwrap_or_default()
	}

	pub fn set<P>(&mut self, value: P) -> &mut Self
	where
		P: Property + 'static,
	{
		let type_id = TypeId::of::<P>();

		self.values.insert(type_id, Box::new(value));

		self
	}
}

impl Component for Style {
	const NAME: &str = "Style";
}
