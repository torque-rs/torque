use std::{any::Any, fmt, future::Future};

use futures::{
	channel::oneshot::{self},
	task::SpawnError,
};
use scoped_tls_hkt::scoped_thread_local;
use winit::event_loop::{EventLoopClosed, EventLoopProxy};

use super::Event;

#[derive(Debug, thiserror::Error)]
pub enum HandleError {
	Spawn(#[from] SpawnError),
	EventLoopClosed(#[from] EventLoopClosed<Event>),
	Cancelled(#[from] oneshot::Canceled),
	TypeMismatch(Box<dyn Any>),
}

impl fmt::Display for HandleError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

#[derive(Clone, Debug)]
pub struct Handle {
	event_loop_proxy: EventLoopProxy<Event>,
}

scoped_thread_local! {
	static CURRENT: Handle
}

impl Handle {
	pub(crate) fn new(event_loop_proxy: EventLoopProxy<Event>) -> Self {
		Self { event_loop_proxy }
	}

	pub fn current() -> Handle {
		CURRENT.with(|handle| handle.clone())
	}

	pub fn enter<R>(&self, f: impl FnOnce() -> R) -> R {
		CURRENT.set(self, f)
	}

	pub fn spawn(
		&self,
		future: impl Future<Output = ()> + Send + 'static,
	) -> Result<(), HandleError> {
		self
			.event_loop_proxy
			.send_event(Event::Spawn(Box::pin(future)))?;

		Ok(())
	}

	/*pub async fn spawn<R>(
		&self,
		future: impl Future<Output = R> + Send + 'static,
	) -> Result<R, ClientError>
	where
		R: 'static,
	{
		let (tx, rx) = oneshot::channel::<RemoteHandle<Box<dyn Any>>>();

		self.event_loop_proxy.send_event(Event::Spawn(
			Box::pin(async move { Box::new(future.await) as Box<dyn Any> }),
			tx,
		))?;

		Ok(
			rx.await?
				.await
				.downcast::<R>()
				.map(|v| *v)
				.map_err(|error| ClientError::TypeMismatch(error))?,
		)
	}*/

	/*pub async fn create_window(&self) -> Result<Window, ClientError> {
		self
			.spawn(Box::new(move || async {
				with_application(|application| Box::new(application.create_window().await) as Box<dyn Any>
			}))
			.await
	}*/

	pub async fn invoke_async<Fut, R>(
		&self,
		f: impl FnOnce() -> Fut + 'static,
	) -> Result<R, HandleError>
	where
		R: 'static,
		Fut: Future<Output = Box<dyn Any>> + 'static,
	{
		let (tx, rx) = oneshot::channel();

		self
			.event_loop_proxy
			.send_event(Event::InvokeAsync(Box::new(|| Box::pin(f())), tx))?;

		rx.await??
			.await
			.downcast::<R>()
			.map(|v| *v)
			.map_err(HandleError::TypeMismatch)
	}
}
