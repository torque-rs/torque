use super::{Node, NodeContent, NodeRef};

#[derive(Clone)]
pub struct Text(pub(crate) NodeRef);

impl Node for Text {
	fn node_ref(&self) -> &NodeRef {
		&self.0
	}
}

#[derive(Debug)]
pub struct TextContent {
	text: String,
	metrics: cosmic_text::Metrics,
	attrs: cosmic_text::AttrsOwned,
	buffer: Option<cosmic_text::Buffer>,
}

impl TextContent {
	pub fn new(
		text: impl AsRef<str>,
		metrics: cosmic_text::Metrics,
		attrs: cosmic_text::AttrsOwned,
	) -> Self {
		Self {
			text: text.as_ref().to_owned(),
			metrics,
			attrs,
			buffer: None,
		}
	}

	pub fn measure(
		&mut self,
		known_dimensions: taffy::Size<Option<f32>>,
		available_space: taffy::Size<taffy::AvailableSpace>,
		_node: &NodeContent,
		font_system: &mut cosmic_text::FontSystem,
	) -> taffy::Size<f32> {
		// Set width constraint
		let width_constraint = known_dimensions.width.or(match available_space.width {
			taffy::AvailableSpace::MinContent => Some(0.0),
			taffy::AvailableSpace::MaxContent => None,
			taffy::AvailableSpace::Definite(width) => Some(width),
		});

		let buffer = self.buffer.get_or_insert_with(|| {
			let mut buffer = cosmic_text::Buffer::new(font_system, self.metrics);

			buffer.set_size(font_system, None, None);
			buffer.set_text(
				font_system,
				&self.text,
				self.attrs.as_attrs(),
				cosmic_text::Shaping::Advanced,
			);

			buffer
		});

		buffer.set_size(font_system, width_constraint, None);

		// Compute layout
		buffer.shape_until_scroll(font_system, false);

		// Determine measured size of text
		let (width, total_lines) = buffer
			.layout_runs()
			.fold((0.0, 0usize), |(width, total_lines), run| {
				(run.line_w.max(width), total_lines + 1)
			});
		let height = total_lines as f32 * buffer.metrics().line_height;

		taffy::Size { width, height }
	}
}
