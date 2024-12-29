pub mod block;
pub mod core;
pub mod flexbox;
mod property;
pub mod text;

use std::any::{Any, TypeId};

use fnv::FnvHashMap;

pub use self::{core::Layout, property::Property};

#[macro_export]
macro_rules! properties {
	($($name:ident: $ty:path),*) => {
		$(
			pub struct $name;

			impl $crate::style::Property for $name {
				type Value = $ty;
			}
		)*
	};
}

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
