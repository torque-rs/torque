use std::sync::Arc;

use torque_ecs::System;

use crate::Element;

pub struct Window(Arc<Inner>);

pub struct Inner {
	window: winit::window::Window,
	instance: wgpu::Instance,
	surface: wgpu::Surface<'static>,
	surface_config: wgpu::SurfaceConfiguration,
	device: wgpu::Device,
	queue: wgpu::Queue,
	system: System,
	root: Element,
}

impl Inner {}
