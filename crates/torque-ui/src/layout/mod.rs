mod available_space;
mod input;
mod output;

use torque_ecs::Component;
use torque_geometry::Rect;

pub use self::{available_space::AvailableSpace, input::Input, output::Output};

#[derive(Component)]
pub struct Layout {
	pub bounds: Rect<f32>,
	pub border: Rect<f32>,
	pub padding: Rect<f32>,
	pub margin: Rect<f32>,
}

pub enum Pass {
	Measure,
	Place,
}
