use std::sync::Arc;

#[derive(Clone)]
pub struct Renderer(Arc<Inner>);

impl Renderer {
	//pub fn new() -> Self(Arc::new(Inner {}))
}

impl torque_renderer::Renderer for Renderer {}

pub struct Inner {
	window: winit::window::Window,
	instance: wgpu::Instance,
	surface: wgpu::Surface<'static>,
	surface_config: wgpu::SurfaceConfiguration,
	device: wgpu::Device,
	queue: wgpu::Queue,
}
