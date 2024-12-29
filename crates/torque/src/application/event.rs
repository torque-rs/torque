use std::{any::Any, fmt::Debug};

use futures::{
	channel::oneshot::Sender,
	future::{BoxFuture, LocalBoxFuture, RemoteHandle},
	task::SpawnError,
};

pub type BoxInvokeFn = Box<dyn FnOnce()>;
pub type BoxInvokeAsyncFn = Box<dyn FnOnce() -> LocalBoxFuture<'static, Box<dyn Any>>>;

pub enum Event {
	Invoke(BoxInvokeFn),
	InvokeAsync(
		BoxInvokeAsyncFn,
		Sender<Result<RemoteHandle<Box<dyn Any>>, SpawnError>>,
	),
	Spawn(BoxFuture<'static, ()>),
}

impl Debug for Event {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Invoke(_) => f.debug_tuple("Invoke").finish(),
			Self::InvokeAsync(_, _) => f.debug_tuple("InvokeAsync").finish(),
			Self::Spawn(_) => f.debug_tuple("Spawn").finish(),
		}
	}
}
