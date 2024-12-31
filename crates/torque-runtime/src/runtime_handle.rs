use std::future::Future;

use futures::{
	channel::oneshot::{self},
	executor::block_on,
	task::LocalSpawnExt,
};
use scoped_tls_hkt::scoped_thread_local;
use tracing::{instrument, trace};

use crate::{
	winit, with_platform, with_spawner, BoxSendSyncAny, RuntimeError, Thread, ThreadHandle,
};

use super::RuntimeEvent;

#[derive(Clone, Debug)]
pub struct RuntimeHandle {
	event_loop_proxy: winit::EventLoopProxy<RuntimeEvent>,
}

scoped_thread_local! {
	static CURRENT: RuntimeHandle
}

impl RuntimeHandle {
	pub(crate) fn new(event_loop_proxy: winit::EventLoopProxy<RuntimeEvent>) -> Self {
		Self { event_loop_proxy }
	}

	pub fn current() -> RuntimeHandle {
		CURRENT.with(|handle| handle.clone())
	}

	pub fn enter<R>(&self, f: impl FnOnce() -> R) -> R {
		CURRENT.set(self, f)
	}

	pub fn invoke(&self, f: impl FnOnce() + Send + Sync + 'static) -> Result<(), RuntimeError> {
		self
			.event_loop_proxy
			.send_event(RuntimeEvent::Invoke(Box::new(f)))?;

		Ok(())
	}

	pub fn spawn_thread<R>(
		&self,
		f: impl FnOnce() -> R + Send + Sync + 'static,
	) -> Result<ThreadHandle<R>, RuntimeError>
	where
		R: Send + Sync + 'static,
	{
		self.spawn_thread_async(|| async { f() })
	}

	#[instrument(skip(f))]
	pub fn spawn_thread_async<Fut, R>(
		&self,
		f: impl FnOnce() -> Fut + Send + Sync + 'static,
	) -> Result<ThreadHandle<R>, RuntimeError>
	where
		Fut: Future<Output = R> + Send + 'static,
		R: Send + Sync + 'static,
	{
		let (tx, rx) = oneshot::channel();
		let runtime_handle = self.clone();

		trace!("sending runtime event requesting spawn of new thread");

		self
			.event_loop_proxy
			.send_event(RuntimeEvent::Invoke(Box::new(move || {
				with_platform(|platform| {
					let (thread, join_handle) = Thread::new(platform.clone(), runtime_handle.clone(), f);

					let _ = tx.send(ThreadHandle::new(thread, join_handle));
				});
			})))?;

		Ok(block_on(rx)?)
	}

	pub async fn spawn_future<Fut, R>(
		&self,
		f: impl FnOnce() -> Fut + Send + Sync + 'static,
	) -> Result<R, RuntimeError>
	where
		R: 'static,
		Fut: Future<Output = BoxSendSyncAny> + Send + 'static,
	{
		let (tx, rx) = oneshot::channel();

		self
			.event_loop_proxy
			.send_event(RuntimeEvent::Invoke(Box::new(move || {
				with_spawner(|spawner| {
					let handle = spawner.spawn_local_with_handle(f());

					tx.send(handle).unwrap();
				})
			})))?;

		rx.await??
			.await
			.downcast::<R>()
			.map(|v| *v)
			.map_err(RuntimeError::TypeMismatch)
	}
}
