use futures::{channel::oneshot, task::SpawnError};

use crate::{winit, BoxSendSyncAny, RuntimeEvent};

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
	#[error("spawn error")]
	Spawn(#[from] SpawnError),

	#[error("event loop closed")]
	EventLoopClosed(#[from] winit::EventLoopClosed<RuntimeEvent>),

	#[error("cancelled")]
	Cancelled(#[from] oneshot::Canceled),

	#[error("exited early")]
	ExitedEarly,

	#[error("thread panic")]
	ThreadPanic(Option<String>),

	#[error("type mismatch")]
	TypeMismatch(BoxSendSyncAny),
}
