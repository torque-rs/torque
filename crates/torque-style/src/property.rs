use std::{any::Any, fmt::Debug};

pub trait Property: Any + Clone + Send + Debug {}
