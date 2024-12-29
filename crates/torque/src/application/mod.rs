mod event;
mod handle;

use std::{cell::RefCell, future::Future, path::Path};

use fnv::FnvHashMap;
use futures::{executor::LocalSpawner, task::LocalSpawnExt};
use m8::with_scope;
use scoped_tls_hkt::scoped_thread_local;
use winit::{event_loop::EventLoopProxy, window::WindowId};

use crate::{ui::Window, Compiler, Runtime};

pub use self::handle::{Handle, HandleError};

pub(crate) use self::event::Event;

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {}

/*impl fmt::Display for ApplicationError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}*/

pub struct Application {
	event_loop_proxy: EventLoopProxy<Event>,
	spawner: LocalSpawner,
	compiler: Compiler,
	windows: RefCell<FnvHashMap<WindowId, Window>>,
}

scoped_thread_local!(static mut APPLICATION: Application);

impl Application {
	pub fn run(main_fn: impl FnOnce() + 'static) {
		Runtime::run(main_fn);
	}

	pub(crate) fn new(
		event_loop_proxy: EventLoopProxy<Event>,
		spawner: LocalSpawner,
		compiler: Compiler,
	) -> Self {
		Self {
			event_loop_proxy,
			spawner,
			compiler,
			windows: RefCell::new(FnvHashMap::default()),
		}
	}

	pub fn handle(&mut self) -> Handle {
		Handle::new(self.event_loop_proxy.clone())
	}

	pub fn enter<R>(&mut self, f: impl FnOnce() -> R) -> R {
		APPLICATION.set(self, f)
	}

	pub fn with<R>(f: impl FnOnce(&mut Self) -> R) -> R {
		APPLICATION.with(f)
	}

	pub fn spawn(&mut self, future: impl Future<Output = ()> + 'static) {
		// TODO: handle error gracefully
		self.spawner.spawn_local(future).unwrap();
	}

	pub(crate) fn register_window(&mut self, window: Window) {
		let window_id = window.id();

		self.windows.borrow_mut().insert(window_id, window);
	}
	/*pub async fn spawn<R>(
		&self,
		future: impl Future<Output = R> + Send + 'static,
	) -> Result<R, ApplicationError>
	where
		R: 'static,
	{
		match self {
			Application::Client(client) => Ok(client.spawn(future).await?),
			Application::Server(server) => Ok(server.spawn(future).await?),
		}
	}*/

	/*pub async fn create_window(&self) -> Result<Window, ServerError> {
		let window =
			with_event_loop(|event_loop| event_loop.create_window(WindowAttributes::default()))?;

		Ok(Window::new(window).await?)
	}*/

	pub fn load_module(&mut self, path: &Path, specifier: Option<String>) {
		let compiler = self.compiler.clone();

		Self::enter(self, || {
			let path = path.to_path_buf();

			with_scope(|scope| {
				compiler.load_module(scope, specifier, Some(path));
			});
		});
	}
}
