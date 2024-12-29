#[derive(Clone, Copy, Debug, Default)]
pub struct Rect<T> {
	pub left: T,
	pub right: T,
	pub top: T,
	pub bottom: T,
}
