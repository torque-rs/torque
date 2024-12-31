use std::sync::Arc;

use crate::winit;

pub struct Window {
	inner: Arc<winit::Window>,
}
