use num_traits::Zero;
use torque_geometry::Size;

pub trait Resolve<In, Out> {
	fn resolve(self, context: In) -> Out;
}

pub trait ResolveOrZero<In, Out>
where
	Out: Zero,
{
	fn resolve_or_zero(self, context: In) -> Out;
}

impl<T> Resolve<f32, Option<f32>> for T
where
	T: Resolve<Option<f32>, Option<f32>>,
{
	fn resolve(self, context: f32) -> Option<f32> {
		self.resolve(Some(context))
	}
}

impl<In, Out, T> Resolve<Size<In>, Size<Out>> for Size<T>
where
	T: Resolve<In, Out>,
{
	fn resolve(self, context: Size<In>) -> Size<Out> {
		Size {
			width: self.width.resolve(context.width),
			height: self.height.resolve(context.height),
		}
	}
}
