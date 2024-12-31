use std::thread::ThreadId;

use futures::{channel::mpsc::UnboundedSender, executor::LocalSpawner};
use torque_compiler::Compiler;

use crate::{RuntimeHandle, ThreadEvent};

#[derive(Clone)]
pub struct ThreadContext {
	pub id: ThreadId,
	pub spawner: LocalSpawner,
	pub compiler: Compiler,
	pub runtime_handle: RuntimeHandle,
	pub event_tx: UnboundedSender<ThreadEvent>,
}

impl ThreadContext {
	pub fn new(
		id: ThreadId,
		spawner: LocalSpawner,
		compiler: Compiler,
		runtime_handle: RuntimeHandle,
		event_tx: UnboundedSender<ThreadEvent>,
	) -> Self {
		Self {
			id,
			spawner,
			compiler,
			runtime_handle,
			event_tx,
		}
	}
}
