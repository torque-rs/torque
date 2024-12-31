mod console;
mod runtime;
mod runtime_error;
mod runtime_event;
mod runtime_handle;
mod thread;
mod thread_context;
mod thread_handle;
mod window;

use std::any::Any;

pub use torque_runtime_macros::main;

pub type BoxSendAny = Box<dyn Any + Send + 'static>;
pub type BoxSendSyncAny = Box<dyn Any + Send + Sync + 'static>;

pub use self::{
	runtime::Runtime,
	runtime_error::RuntimeError,
	runtime_handle::RuntimeHandle,
	thread::{Thread, ThreadEvent},
	thread_context::ThreadContext,
	thread_handle::ThreadHandle,
	window::Window,
};

pub(crate) use self::{
	runtime::{with_event_loop, with_platform, with_spawner, with_threads},
	runtime_event::RuntimeEvent,
};

// this enables short qualified references to all winit types, much like wgpu
pub(crate) mod winit {
	pub use ::winit::{application::*, event::*, event_loop::*, window::*};
}
