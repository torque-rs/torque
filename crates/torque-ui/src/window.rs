use std::{fmt, ops::Deref, sync::Arc};

use torque_ecs::{EntityRef, System};
use winit::{error::OsError, window::WindowId};

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
	/*pub async fn new() -> Result<Self, CreateWindowError> {
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

		//Application::with(|application| application.register_window(window.clone()));

		Ok(window)
	}*/
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
	//root: EntityRef<Element>,
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

	/*pub fn create_element(&self) -> EntityRef<Element> {
		let element = self.system.create::<Element>();

		//element.set::<Window>(self.clone());

		element
	}*/
}

impl v8::cppgc::GarbageCollected for Inner {
	fn trace(&self, _visitor: &v8::cppgc::Visitor) {}

	fn get_name(&self) -> Option<&'static std::ffi::CStr> {
		None
	}
}

impl Window {
	pub(crate) fn __m8_init(scope: &mut v8::HandleScope, module: &v8::Local<v8::Module>) {}
}
