use std::{fmt, ops::Deref};

use futures::channel::oneshot::Canceled;
use m8::{throw_error, with_scope, TryFromV8, V8Type, V8TypeGarbageCollected, V8TypeInfo};
use serde_v8::to_v8;
use torque_ecs::{Component, System};
use torque_ui::Element;
use winit::{
	error::OsError,
	window::{WindowAttributes, WindowId},
};

use crate::{with_event_loop, Application};

#[derive(Debug, thiserror::Error)]
pub enum WindowError {
	OsError(#[from] OsError),
	ApplicationClosed(#[from] Canceled),
}

impl fmt::Display for WindowError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

#[derive(Debug, thiserror::Error, serde::Serialize)]
pub enum CreateWindowError {
	HandleError(
		#[serde(skip)]
		#[from]
		wgpu::rwh::HandleError,
	),
	CreateWindowFailed(
		#[serde(skip)]
		#[from]
		OsError,
	),
	CreateSurfaceFailed(
		#[serde(skip)]
		#[from]
		wgpu::CreateSurfaceError,
	),
	RequestAdapterFailed,
	RequestDeviceFailed(
		#[serde(skip)]
		#[from]
		wgpu::RequestDeviceError,
	),
	SurfaceNotSupported,
}

impl fmt::Display for CreateWindowError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

#[derive(Debug)]
pub struct Window(v8::cppgc::Persistent<Inner>);

impl Clone for Window {
	fn clone(&self) -> Self {
		Self(v8::cppgc::Persistent::new(&self.0))
	}
}

impl Window {
	pub async fn new() -> Result<Self, CreateWindowError> {
		let window =
			with_event_loop(|event_loop| event_loop.create_window(WindowAttributes::default()))?;

		let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
			backends: wgpu::Backends::PRIMARY,
			flags: wgpu::InstanceFlags::all(),
			..Default::default()
		});
		let surface =
			unsafe { instance.create_surface_unsafe(wgpu::SurfaceTargetUnsafe::from_window(&window)?) }?;
		let adapter = instance
			.request_adapter(&wgpu::RequestAdapterOptions {
				power_preference: wgpu::PowerPreference::HighPerformance,
				compatible_surface: Some(&surface),
				force_fallback_adapter: false,
			})
			.await
			.ok_or(CreateWindowError::RequestAdapterFailed)?;

		let (device, queue) = adapter
			.request_device(
				&wgpu::DeviceDescriptor {
					required_features: wgpu::Features::default(),
					required_limits: wgpu::Limits::default(),
					label: None,
					..Default::default()
				},
				None,
			)
			.await?;

		let size = window.inner_size();
		let mut surface_config = surface
			.get_default_config(&adapter, size.width, size.height)
			.ok_or(CreateWindowError::SurfaceNotSupported)?;

		let view_format = surface_config.format.add_srgb_suffix();
		surface_config.view_formats.push(view_format);
		surface.configure(&device, &surface_config);

		let window = with_scope(|scope| unsafe {
			v8::cppgc::make_garbage_collected(
				scope.get_cpp_heap().unwrap(),
				Inner {
					window,
					instance,
					surface,
					surface_config,
					device,
					queue,
					system: System::default(),
				},
			)
		});

		let window = Window(v8::cppgc::Persistent::new(&window));

		Application::with(|application| application.register_window(window.clone()));

		Ok(window)
	}
}

impl Deref for Window {
	type Target = Inner;

	fn deref(&self) -> &Self::Target {
		self.0.borrow().unwrap()
	}
}

pub struct Inner {
	window: winit::window::Window,
	instance: wgpu::Instance,
	surface: wgpu::Surface<'static>,
	surface_config: wgpu::SurfaceConfiguration,
	device: wgpu::Device,
	queue: wgpu::Queue,
	system: System,
}

impl Inner {
	pub fn id(&self) -> WindowId {
		self.window.id()
	}

	pub fn title(&self) -> String {
		self.window.title()
	}

	pub fn set_title(&self, title: impl AsRef<str>) {
		self.window.set_title(title.as_ref());
	}

	pub fn is_visible(&self) -> bool {
		self.window.is_visible().unwrap_or(false)
	}

	pub fn set_visible(&self, visible: bool) {
		self.window.set_visible(visible);
	}

	pub fn create_element(&self) -> Element {
		let element = self.system.create::<Element>();

		//element.set::<Window>(self.clone());

		element
	}
}

impl v8::cppgc::GarbageCollected for Inner {
	fn trace(&self, _visitor: &v8::cppgc::Visitor) {}

	fn get_name(&self) -> Option<&'static std::ffi::CStr> {
		None
	}
}

pub struct WindowV8TypeInfo {
	object_template: v8::Global<v8::ObjectTemplate>,
}

impl V8TypeInfo for WindowV8TypeInfo {
	fn object_template<'s>(
		&self,
		scope: &mut v8::HandleScope<'s>,
	) -> v8::Local<'s, v8::ObjectTemplate> {
		v8::Local::new(scope, self.object_template.clone())
	}
}

impl V8Type for Inner {
	type Info = WindowV8TypeInfo;
}

impl V8TypeGarbageCollected for Inner {
	fn wrap(scope: &mut v8::HandleScope, wrapper: v8::Local<v8::Object>, ptr: v8::cppgc::Ptr<Self>) {
		unsafe { v8::Object::wrap::<1, Inner>(scope, wrapper, &ptr) }
	}

	fn unwrap(
		scope: &mut v8::HandleScope,
		wrapper: v8::Local<v8::Object>,
	) -> Option<v8::cppgc::Ptr<Self>> {
		unsafe { v8::Object::unwrap::<1, Inner>(scope, wrapper) }
	}
}

impl Window {
	fn __v8_new(
		_scope: &mut v8::HandleScope,
		_args: v8::FunctionCallbackArguments,
		mut _rv: v8::ReturnValue,
	) {
	}

	pub async fn create() -> Result<Self, CreateWindowError> {
		Window::new().await
	}

	fn __v8_create(
		scope: &mut v8::HandleScope<'_>,
		_args: v8::FunctionCallbackArguments,
		mut rv: v8::ReturnValue,
	) {
		Application::with(move |application| {
			let promise_resolver = v8::PromiseResolver::new(scope).unwrap();
			let promise = promise_resolver.get_promise(scope);
			let promise_resolver = v8::Global::new(scope, promise_resolver);

			application.spawn(async move {
				let result = Window::create().await;

				with_scope(|scope| match result {
					Ok(window) => {
						let object_template = match Inner::object_template(scope) {
							Some(value) => value,
							None => {
								throw_error!(scope, "context not initialized");

								return;
							}
						};

						let object = match object_template.new_instance(scope) {
							Some(value) => value,
							None => {
								throw_error!(scope, "new_instance failed");

								return;
							}
						};

						unsafe { v8::Object::wrap::<1, Inner>(scope, object, &window.0) }

						promise_resolver.open(scope).resolve(scope, object.into());
					}
					Err(error) => {
						let value = to_v8(scope, error).unwrap();

						promise_resolver.open(scope).reject(scope, value);
					}
				});
			});

			rv.set(promise.into());
		});
	}

	fn __v8_get_visible(
		scope: &mut v8::HandleScope,
		args: v8::FunctionCallbackArguments,
		mut rv: v8::ReturnValue,
	) {
		let this: v8::cppgc::Ptr<Inner> =
			match <v8::cppgc::Ptr<Inner> as TryFromV8>::try_from_v8(scope, args.this().into()) {
				Ok(value) => value,
				Err(error) => {
					println!("{:?}", error);

					throw_error!(scope, &format!("{}", error));

					return;
				}
			};

		rv.set(v8::Boolean::new(scope, this.is_visible()).into());
	}

	fn __v8_set_visible(
		scope: &mut v8::HandleScope,
		args: v8::FunctionCallbackArguments,
		mut _rv: v8::ReturnValue,
	) {
		let this: v8::cppgc::Ptr<Inner> =
			match <v8::cppgc::Ptr<Inner> as TryFromV8>::try_from_v8(scope, args.this().into()) {
				Ok(value) => value,
				Err(error) => {
					println!("{:?}", error);

					throw_error!(scope, &format!("{}", error));

					return;
				}
			};

		let visible = args.get(0).boolean_value(scope);

		println!("set_visible: {}", visible);

		this.set_visible(visible);
	}

	fn __v8_get_title(
		scope: &mut v8::HandleScope,
		args: v8::FunctionCallbackArguments,
		mut rv: v8::ReturnValue,
	) {
		let this: v8::cppgc::Ptr<Inner> =
			match <v8::cppgc::Ptr<Inner> as TryFromV8>::try_from_v8(scope, args.this().into()) {
				Ok(value) => value,
				Err(error) => {
					println!("{:?}", error);

					throw_error!(scope, &format!("{}", error));

					return;
				}
			};

		rv.set(v8::String::new(scope, &this.title()).unwrap().into());
	}

	fn __v8_set_title(
		scope: &mut v8::HandleScope,
		args: v8::FunctionCallbackArguments,
		mut _rv: v8::ReturnValue,
	) {
		let this: v8::cppgc::Ptr<Inner> =
			match <v8::cppgc::Ptr<Inner> as TryFromV8>::try_from_v8(scope, args.this().into()) {
				Ok(value) => value,
				Err(error) => {
					println!("{:?}", error);

					throw_error!(scope, &format!("{}", error));

					return;
				}
			};

		let title = args.get(0).to_string(scope).unwrap();

		this.set_title(title.to_rust_string_lossy(scope));
	}

	fn __v8_create_element(
		scope: &mut v8::HandleScope,
		args: v8::FunctionCallbackArguments,
		mut _rv: v8::ReturnValue,
	) {
		let this: v8::cppgc::Ptr<Inner> =
			match <v8::cppgc::Ptr<Inner> as TryFromV8>::try_from_v8(scope, args.this().into()) {
				Ok(value) => value,
				Err(error) => {
					println!("{:?}", error);

					throw_error!(scope, &format!("{}", error));

					return;
				}
			};

		let element = this.createElement();
	}
}

pub fn init(scope: &mut v8::HandleScope, module: &v8::Local<v8::Module>) {
	let context = scope.get_current_context();

	let function_template = v8::FunctionTemplate::new(scope, Window::__v8_new);

	function_template.set_class_name(v8::String::new(scope, "Window").unwrap());

	let create = v8::FunctionTemplate::new(scope, Window::__v8_create);

	function_template.set(
		v8::String::new(scope, "create").unwrap().into(),
		create.into(),
	);

	let object_template = function_template.instance_template(scope);

	let get_visible = v8::FunctionTemplate::new(scope, Window::__v8_get_visible);
	let set_visible = v8::FunctionTemplate::new(scope, Window::__v8_set_visible);

	object_template.set_accessor_property(
		v8::String::new(scope, "visible ").unwrap().into(),
		Some(get_visible),
		Some(set_visible),
		v8::PropertyAttribute::DONT_DELETE,
	);

	let get_title = v8::FunctionTemplate::new(scope, Window::__v8_get_title);
	let set_title = v8::FunctionTemplate::new(scope, Window::__v8_set_title);

	object_template.set_accessor_property(
		v8::String::new(scope, "title").unwrap().into(),
		Some(get_title),
		Some(set_title),
		v8::PropertyAttribute::DONT_DELETE,
	);

	let object_template = v8::Global::new(scope, object_template);

	let type_info = WindowV8TypeInfo { object_template };

	context.set_slot(type_info);

	let export_name = v8::String::new(scope, "Window").unwrap();
	let export_value = function_template.get_function(scope).unwrap().into();

	module
		.set_synthetic_module_export(scope, export_name, export_value)
		.unwrap();
}
