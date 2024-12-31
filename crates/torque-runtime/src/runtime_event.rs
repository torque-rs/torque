use std::fmt::Debug;

pub type BoxInvokeFn = Box<dyn FnOnce() + Send + Sync>;

pub enum RuntimeEvent {
	Invoke(BoxInvokeFn),
}

impl Debug for RuntimeEvent {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Invoke(_) => f.debug_tuple("Invoke").finish(),
		}
	}
}
