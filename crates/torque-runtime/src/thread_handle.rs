use std::{marker::PhantomData, thread::JoinHandle};

use crate::{BoxSendSyncAny, RuntimeError, Thread};

#[derive(Debug)]
pub struct ThreadHandle<R> {
	thread: Thread,
	join_handle: JoinHandle<BoxSendSyncAny>,
	_phantom: PhantomData<R>,
}

impl<R> ThreadHandle<R>
where
	R: Send + Sync + 'static,
{
	pub(crate) fn new(thread: Thread, join_handle: JoinHandle<BoxSendSyncAny>) -> Self {
		Self {
			thread,
			join_handle,
			_phantom: PhantomData,
		}
	}

	pub fn thread(&self) -> &Thread {
		&self.thread
	}

	pub fn is_finished(&self) -> bool {
		self.join_handle.is_finished()
	}

	pub fn join(self) -> Result<R, RuntimeError> {
		let Self { join_handle, .. } = self;

		Ok(
			*join_handle
				.join()
				.map(|v| v.downcast::<R>())
				.map_err(|error| {
					if let Some(value) = error.downcast_ref::<&str>() {
						RuntimeError::ThreadPanic(Some(value.to_string()))
					} else if let Some(value) = error.downcast_ref::<String>() {
						RuntimeError::ThreadPanic(Some(value.to_owned()))
					} else {
						RuntimeError::ThreadPanic(None)
					}
				})?
				.map_err(RuntimeError::TypeMismatch)?,
		)
	}
}
