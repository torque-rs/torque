pub mod core;

mod length_percent;
mod length_percent_auto;
mod property;
mod resolve;

use std::any::{Any, TypeId};

use fnv::FnvHashMap;
use torque_ecs::Component;

pub use self::{
	core::*,
	length_percent::LengthPercent,
	length_percent_auto::LengthPercentAuto,
	property::Property,
	resolve::{Resolve, ResolveOrZero},
};

#[derive(Debug, Default)]
pub struct Style {
	values: FnvHashMap<TypeId, Box<dyn Any + Send>>,
}

impl Style {
	pub fn get<P>(&self) -> Option<P::Value>
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

	pub fn get_or<P>(&self, default: P::Value) -> P::Value
	where
		P: Property + 'static,
	{
		self.get::<P>().unwrap_or(default)
	}

	pub fn get_or_default<P>(&self) -> P::Value
	where
		P: Property + 'static,
		P::Value: Default,
	{
		self.get::<P>().unwrap_or_default()
	}

	pub fn set<P>(&mut self, value: P::Value) -> &mut Self
	where
		P: Property + 'static,
	{
		let type_id = TypeId::of::<P>();

		self.values.insert(type_id, Box::new(value));

		self
	}
}

impl Component for Style {
	type Value = Self;

	const NAME: &str = "Style";
}
