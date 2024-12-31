use std::{future::Future, sync::OnceLock, thread::ThreadId};

use fnv::FnvHashMap;
use futures::executor::{LocalPool, LocalSpawner};
use scoped_tls_hkt::scoped_thread_local;
use tracing::trace;

use crate::{winit, RuntimeError, RuntimeEvent, RuntimeHandle, Thread, ThreadHandle};

scoped_thread_local!(static PLATFORM: v8::SharedRef<v8::Platform>);
scoped_thread_local!(static EVENT_LOOP: winit::ActiveEventLoop);
scoped_thread_local!(static SPAWNER: LocalSpawner);
scoped_thread_local!(static mut THREADS: FnvHashMap<ThreadId, Thread>);

pub fn with_platform<R>(f: impl FnOnce(&v8::SharedRef<v8::Platform>) -> R) -> R {
	PLATFORM.with(f)
}

pub fn with_event_loop<R>(f: impl FnOnce(&winit::ActiveEventLoop) -> R) -> R {
	EVENT_LOOP.with(f)
}

pub fn with_spawner<R>(f: impl FnOnce(&LocalSpawner) -> R) -> R {
	SPAWNER.with(f)
}

pub fn with_threads<R>(f: impl FnOnce(&mut FnvHashMap<ThreadId, Thread>) -> R) -> R {
	THREADS.with(f)
}

pub struct Runtime {
	platform: v8::SharedRef<v8::Platform>,
	local_pool: LocalPool,
	threads: FnvHashMap<ThreadId, Thread>,
	event_loop_proxy: winit::EventLoopProxy<RuntimeEvent>,
}

impl Runtime {
	fn new(
		platform: v8::SharedRef<v8::Platform>,
		event_loop_proxy: winit::EventLoopProxy<RuntimeEvent>,
		main_thread: Thread,
	) -> Self {
		let local_pool = LocalPool::new();
		let mut threads: FnvHashMap<ThreadId, Thread> = Default::default();

		threads.insert(main_thread.id(), main_thread);

		Self {
			platform,
			local_pool,
			threads,
			event_loop_proxy,
		}
	}

	pub fn run_sync<R>(f: impl FnOnce() -> R + Send + Sync + 'static) -> Result<R, RuntimeError>
	where
		R: Send + Sync + 'static,
	{
		Self::run(|| async { f() })
	}

	pub fn run<Fut, R>(f: impl FnOnce() -> Fut + Send + Sync + 'static) -> Result<R, RuntimeError>
	where
		Fut: Future<Output = R> + Send + Sync + 'static,
		R: Send + Sync + 'static,
	{
		#[cfg(feature = "tracing-subscriber")]
		{
			let subscriber = tracing_subscriber::FmtSubscriber::builder()
				.with_thread_names(true)
				.with_max_level(tracing::Level::TRACE)
				.finish();

			tracing::subscriber::set_global_default(subscriber)
				.expect("setting default subscriber failed");
		}

		static PLATFORM: OnceLock<v8::SharedRef<v8::Platform>> = OnceLock::new();

		let platform = PLATFORM
			.get_or_init(|| {
				trace!("initializing v8 platform");

				let platform = v8::new_default_platform(0, false).make_shared();

				v8::V8::initialize_platform(platform.clone());
				v8::V8::initialize();
				v8::cppgc::initalize_process(platform.clone());

				platform
			})
			.clone();

		let event_loop = winit::EventLoop::<RuntimeEvent>::with_user_event()
			.build()
			.unwrap();

		event_loop.set_control_flow(winit::ControlFlow::Poll);

		let event_loop_proxy = event_loop.create_proxy();
		let runtime_handle = RuntimeHandle::new(event_loop_proxy.clone());

		let (main_thread, join_handle) = Thread::new(platform.clone(), runtime_handle, f);
		let thread_handle = ThreadHandle::<R>::new(main_thread.clone(), join_handle);

		let mut app = Runtime::new(platform, event_loop_proxy, main_thread);

		event_loop.run_app(&mut app).unwrap();

		thread_handle.join()
	}
}

fn enter<R>(
	platform: &v8::SharedRef<v8::Platform>,
	event_loop: &winit::ActiveEventLoop,
	spawner: &LocalSpawner,
	threads: &mut FnvHashMap<ThreadId, Thread>,
	f: impl FnOnce() -> R,
) -> R {
	PLATFORM.set(platform, || {
		EVENT_LOOP.set(event_loop, || {
			SPAWNER.set(spawner, || THREADS.set(threads, f))
		})
	})
}

impl winit::ApplicationHandler<RuntimeEvent> for Runtime {
	fn new_events(&mut self, event_loop: &winit::ActiveEventLoop, cause: winit::StartCause) {
		match cause {
			winit::StartCause::ResumeTimeReached {
				start,
				requested_resume,
			} => (),
			winit::StartCause::WaitCancelled {
				start,
				requested_resume,
			} => (),
			winit::StartCause::Poll => {
				let Self {
					platform,
					local_pool,
					threads,
					..
				} = self;

				if threads.is_empty() {
					event_loop.exit();

					return;
				}

				enter(platform, event_loop, &local_pool.spawner(), threads, || {
					local_pool.run_until_stalled()
				});
			}
			winit::StartCause::Init => {}
		}
	}

	fn resumed(&mut self, event_loop: &winit::ActiveEventLoop) {}

	fn user_event(&mut self, event_loop: &winit::ActiveEventLoop, event: RuntimeEvent) {
		match event {
			RuntimeEvent::Invoke(v) => {
				let Self {
					platform,
					local_pool,
					threads,
					..
				} = self;

				enter(platform, event_loop, &local_pool.spawner(), threads, || v());
			}
		}
	}

	fn window_event(
		&mut self,
		event_loop: &winit::ActiveEventLoop,
		window_id: winit::WindowId,
		event: winit::WindowEvent,
	) {
		trace!("window_event: {:?}", event);
		//todo!()
	}
}
