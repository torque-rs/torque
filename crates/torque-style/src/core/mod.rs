mod aspect_ratio;
mod border;
mod box_sizing;
mod display;
mod inset;
mod layout;
mod length_percent;
mod length_percent_auto;
mod margin;
mod max_size;
mod min_size;
mod overflow;
mod padding;
mod position;
mod scrollbar_width;
mod size;

pub use self::{
	aspect_ratio::AspectRatio, border::Border, box_sizing::BoxSizing, display::Display, inset::Inset,
	layout::Layout, length_percent::LengthPercent, length_percent_auto::LengthPercentAuto,
	margin::Margin, max_size::MaxSize, min_size::MinSize, overflow::Overflow, padding::Padding,
	position::Position, scrollbar_width::ScrollbarWidth, size::Size,
};

impl taffy::CoreStyle for super::Style {
	fn box_generation_mode(&self) -> taffy::BoxGenerationMode {
		self
			.get::<Display>()
			.map_or(taffy::BoxGenerationMode::DEFAULT, |v| v.into())
	}

	fn is_block(&self) -> bool {
		matches!(self.get_or::<Layout>(Layout::Block), Layout::Block)
	}

	fn box_sizing(&self) -> taffy::BoxSizing {
		self
			.get::<BoxSizing>()
			.map_or(taffy::BoxSizing::BorderBox, |v| v.into())
	}

	fn overflow(&self) -> taffy::Point<taffy::Overflow> {
		self
			.get::<Overflow>()
			.map_or(taffy::Style::DEFAULT.overflow, |v| v.into())
	}

	fn scrollbar_width(&self) -> f32 {
		self.get::<ScrollbarWidth>().map_or(0.0, |v| v.into())
	}

	fn position(&self) -> taffy::Position {
		self
			.get::<Position>()
			.map_or(taffy::Style::DEFAULT.position, |v| v.into())
	}

	fn inset(&self) -> taffy::Rect<taffy::LengthPercentageAuto> {
		self
			.get::<Inset>()
			.map_or(taffy::Style::DEFAULT.inset, |v| v.into())
	}

	fn size(&self) -> taffy::Size<taffy::Dimension> {
		self
			.get::<Size>()
			.map_or(taffy::Style::DEFAULT.size, |v| v.into())
	}

	fn min_size(&self) -> taffy::Size<taffy::Dimension> {
		self
			.get::<MinSize>()
			.map_or(taffy::Style::DEFAULT.min_size, |v| v.into())
	}

	fn max_size(&self) -> taffy::Size<taffy::Dimension> {
		self
			.get::<MaxSize>()
			.map_or(taffy::Style::DEFAULT.max_size, |v| v.into())
	}

	fn aspect_ratio(&self) -> Option<f32> {
		self.get::<AspectRatio>().map(|v| v.into())
	}

	fn margin(&self) -> taffy::Rect<taffy::LengthPercentageAuto> {
		self
			.get::<Margin>()
			.map_or(taffy::Style::DEFAULT.margin, |v| v.into())
	}

	fn padding(&self) -> taffy::Rect<taffy::LengthPercentage> {
		self
			.get::<Padding>()
			.map_or(taffy::Style::DEFAULT.padding, |v| v.into())
	}

	fn border(&self) -> taffy::Rect<taffy::LengthPercentage> {
		self
			.get::<Border>()
			.map_or(taffy::Style::DEFAULT.border, |v| v.into())
	}
}
