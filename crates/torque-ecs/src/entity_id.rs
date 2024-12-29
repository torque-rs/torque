use std::fmt::{self};

use slotmap::new_key_type;

new_key_type! {
	pub struct EntityId;
}

impl fmt::Display for EntityId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}
