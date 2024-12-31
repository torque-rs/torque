use std::{
	future::Future,
	thread::{current, spawn, JoinHandle, ThreadId},
};

use futures::{
	channel::mpsc::{self, UnboundedReceiver, UnboundedSender},
	executor::LocalPool,
	task::LocalSpawnExt,
	FutureExt, StreamExt,
};
use m8::enter_scope;
use scoped_tls_hkt::scoped_thread_local;
use torque_compiler::Compiler;
use tracing::trace;

use crate::{console, with_threads, BoxSendSyncAny, RuntimeHandle, ThreadContext};

pub enum ThreadEvent {}

#[derive(Clone, Debug)]
pub struct Thread {
	id: ThreadId,
	runtime_handle: RuntimeHandle,
	event_tx: mpsc::UnboundedSender<ThreadEvent>,
}

scoped_thread_local! (static CONTEXT: ThreadContext);

impl Thread {
	pub(crate) fn new<Fut, R>(
		platform: v8::SharedRef<v8::Platform>,
		runtime_handle: RuntimeHandle,
		f: impl FnOnce() -> Fut + Send + 'static,
	) -> (Self, JoinHandle<BoxSendSyncAny>)
	where
		Fut: Future<Output = R> + 'static,
		R: Send + Sync + 'static,
	{
		let (event_tx, event_rx) = mpsc::unbounded::<ThreadEvent>();
		let runtime_handle_clone = runtime_handle.clone();
		let event_tx_clone = event_tx.clone();

		trace!("spawning thread");

		let join_handle =
			spawn(move || Self::run(platform, runtime_handle_clone, f, event_tx_clone, event_rx));

		let id = join_handle.thread().id();

		(
			Self {
				id,
				runtime_handle,
				event_tx,
			},
			join_handle,
		)
	}

	fn run<Fut, R>(
		platform: v8::SharedRef<v8::Platform>,
		runtime_handle: RuntimeHandle,
		f: impl (FnOnce() -> Fut) + Send,
		event_tx: UnboundedSender<ThreadEvent>,
		mut event_rx: UnboundedReceiver<ThreadEvent>,
	) -> BoxSendSyncAny
	where
		Fut: Future<Output = R> + 'static,
		R: Send + Sync + 'static,
	{
		let thread_id = current().id();
		let mut local_pool = LocalPool::new();

		let compiler = Compiler::new();
		let thread_context = ThreadContext::new(
			thread_id,
			local_pool.spawner(),
			compiler.clone(),
			runtime_handle.clone(),
			event_tx,
		);

		trace!("setting up v8 isolate and context");

		let heap = v8::cppgc::Heap::create(platform.clone(), v8::cppgc::HeapCreateParams::default());
		let isolate = &mut v8::Isolate::new(v8::CreateParams::default().cpp_heap(heap));
		let context = {
			let scope = &mut v8::HandleScope::new(isolate);
			let context = v8::Context::new(scope, v8::ContextOptions::default());

			context.set_slot(compiler.clone());
			context.set_slot(thread_context.clone());

			v8::Global::new(scope, context)
		};

		{
			let scope = &mut v8::HandleScope::with_context(isolate, context.clone());

			console::init(scope);

			m8::init(scope, |scope, specifier, module| {
				compiler.add_module(specifier.to_string(), v8::Global::new(scope, module));
			});
		}

		trace!("setting up local pool");

		let spawner = local_pool.spawner();

		let main_future = &mut spawner.spawn_local_with_handle(f()).unwrap();

		// TODO: handle error gracefully
		local_pool
			.spawner()
			.spawn_local(async move {
				loop {
					if let Some(event) = event_rx.next().await {
						match event {}
					}
				}
			})
			.unwrap();

		trace!("entering loop");

		loop {
			if let Some(result) = main_future.now_or_never() {
				trace!("exiting thread gracefully");

				let _ = runtime_handle.invoke(move || {
					with_threads(|threads| threads.remove(&thread_id));
				});

				break Box::new(result);
			}

			v8::Platform::pump_message_loop(&platform, isolate, false);
			v8::Platform::run_idle_tasks(&platform, isolate, 0.0);

			let scope = &mut v8::HandleScope::with_context(isolate, context.clone());

			scope.perform_microtask_checkpoint();

			enter_scope(scope, || {
				CONTEXT.set(&thread_context, || {
					local_pool.run_until_stalled();
				})
			});
		}
	}

	pub fn id(&self) -> ThreadId {
		self.id
	}

	pub fn context() -> ThreadContext {
		CONTEXT.with(|context| context.clone())
	}
}
