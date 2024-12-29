use std::{
	cell::OnceCell,
	sync::{Once, OnceLock},
};

use futures::{executor::LocalPool, task::LocalSpawnExt};
use m8::{enter_scope, with_scope};
use scoped_tls_hkt::scoped_thread_local;
use winit::{
	application::ApplicationHandler,
	event::StartCause,
	event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy},
};

use crate::{application, Application, Compiler};

scoped_thread_local!(static mut LOCAL_POOL: LocalPool);
scoped_thread_local!(static EVENT_LOOP: ActiveEventLoop);

pub fn with_local_pool<R>(f: impl FnOnce(&mut LocalPool) -> R) -> R {
	LOCAL_POOL.with(f)
}

pub fn with_event_loop<R>(f: impl FnOnce(&ActiveEventLoop) -> R) -> R {
	EVENT_LOOP.with(f)
}

pub struct Runtime {
	isolate: v8::OwnedIsolate,
	context: OnceCell<v8::Global<v8::Context>>,
	context_init: Once,
	local_pool: LocalPool,
	main_fn: Option<Box<dyn FnOnce()>>,
	application: Application,
	compiler: Compiler,
	event_loop_proxy: EventLoopProxy<application::Event>,
}

impl Runtime {
	fn new(
		isolate: v8::OwnedIsolate,
		compiler: Compiler,
		event_loop_proxy: EventLoopProxy<application::Event>,
		main_fn: Box<dyn FnOnce()>,
	) -> Self {
		let local_pool = LocalPool::new();
		let local_spawner = local_pool.spawner();

		Self {
			isolate,
			context: OnceCell::new(),
			context_init: Once::new(),
			local_pool,
			main_fn: Some(main_fn),
			application: Application::new(event_loop_proxy.clone(), local_spawner, compiler.clone()),
			compiler,
			event_loop_proxy,
		}
	}

	pub(crate) fn run(main_fn: impl FnOnce() + 'static) {
		static PLATFORM: OnceLock<v8::SharedRef<v8::Platform>> = OnceLock::new();

		let platform = PLATFORM
			.get_or_init(|| {
				let platform = v8::new_default_platform(0, false).make_shared();

				v8::V8::initialize_platform(platform.clone());
				v8::V8::initialize();
				v8::cppgc::initalize_process(platform.clone());

				platform
			})
			.clone();

		let compiler = Compiler::new();

		let heap = v8::cppgc::Heap::create(platform, v8::cppgc::HeapCreateParams::default());
		let isolate = v8::Isolate::new(v8::CreateParams::default().cpp_heap(heap));

		let event_loop = EventLoop::<application::Event>::with_user_event()
			.build()
			.unwrap();

		event_loop.set_control_flow(ControlFlow::Poll);

		let event_loop_proxy = event_loop.create_proxy();

		let mut app = Runtime::new(isolate, compiler, event_loop_proxy, Box::new(main_fn));

		event_loop.run_app(&mut app).unwrap();
	}

	fn enter<R>(&mut self, event_loop: &ActiveEventLoop, f: impl FnOnce() -> R) -> R {
		let Self {
			isolate,
			context,
			context_init,
			application,
			local_pool,
			..
		} = self;

		let context = context.get_or_init(|| {
			let scope = &mut v8::HandleScope::new(isolate);
			let context = v8::Context::new(scope, v8::ContextOptions::default());

			context.set_slot(self.compiler.clone());
			/*context.set_slot(Application::Client(application::Client::new(
				self.event_loop_proxy.clone(),
				self.local_pool.spawner(),
			)));*/

			v8::Global::new(scope, context)
		});

		context_init.call_once(|| {
			let scope = &mut v8::HandleScope::with_context(isolate, context);

			crate::jsx_runtime::init(scope);
			crate::ui::init(scope);
			crate::console::init(scope);

			m8::init(scope);
		});

		let scope = &mut v8::HandleScope::with_context(isolate, context);

		LOCAL_POOL.set(local_pool, || {
			EVENT_LOOP.set(event_loop, || application.enter(|| enter_scope(scope, f)))
		})
	}
}

impl ApplicationHandler<application::Event> for Runtime {
	fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
		match cause {
			StartCause::ResumeTimeReached {
				start,
				requested_resume,
			} => (),
			StartCause::WaitCancelled {
				start,
				requested_resume,
			} => (),
			StartCause::Poll => {
				self.enter(event_loop, || {
					with_local_pool(|local_pool| local_pool.run_until_stalled());
					with_scope(|scope| scope.perform_microtask_checkpoint());
				});
			}
			StartCause::Init => {
				if let Some(v) = self.main_fn.take() {
					self.enter(event_loop, v);
				}
			}
		}
	}

	fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {}

	fn user_event(&mut self, event_loop: &ActiveEventLoop, event: application::Event) {
		match event {
			application::Event::Invoke(v) => self.enter(event_loop, v),
			application::Event::InvokeAsync(v, sender) => {
				let result = self.local_pool.spawner().spawn_local_with_handle(v());

				// TODO: handle error gracefully
				sender.send(result).unwrap();
			}
			application::Event::Spawn(future) => {
				// TODO: handle error gracefully
				self.local_pool.spawner().spawn_local(future).unwrap();
			}
		}
	}

	fn window_event(
		&mut self,
		event_loop: &winit::event_loop::ActiveEventLoop,
		window_id: winit::window::WindowId,
		event: winit::event::WindowEvent,
	) {
		log::trace!("window_event: {:?}", event);
		//todo!()
	}
}
