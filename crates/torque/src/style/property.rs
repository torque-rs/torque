use std::{any::Any, fmt::Debug};

pub trait Property {
	type Value: Any + Clone + Send + Debug;
}
