use crate::properties;

use super::Property;

#[derive(Clone, Copy, Debug, Default)]
pub enum Layout {
	#[default]
	Block,
	Grid,
	FlexBox,
}

impl Property for Layout {
	type Value = Self;
}

properties! {
	BoxGenerationMode: taffy::BoxGenerationMode,
	BoxSizing: taffy::BoxSizing,
	ScrollbarWidth: f32,
	Overflow: taffy::Point<taffy::Overflow>,
	Size: taffy::Size<taffy::Dimension>,
	MinSize: taffy::Size<taffy::Dimension>,
	MaxSize: taffy::Size<taffy::Dimension>,
	Position: taffy::Position,
	Inset: taffy::Rect<taffy::LengthPercentageAuto>,
	AspectRatio: f32,
	Margin: taffy::Rect<taffy::LengthPercentageAuto>,
	Padding: taffy::Rect<taffy::LengthPercentage>,
	Border: taffy::Rect<taffy::LengthPercentage>
}

impl taffy::CoreStyle for super::Style {
	fn box_generation_mode(&self) -> taffy::BoxGenerationMode {
		self.get_or::<BoxGenerationMode>(taffy::BoxGenerationMode::DEFAULT)
	}

	fn is_block(&self) -> bool {
		matches!(self.get_or::<Layout>(Layout::Block), Layout::Block)
	}

	fn box_sizing(&self) -> taffy::BoxSizing {
		self.get_or::<BoxSizing>(taffy::BoxSizing::BorderBox)
	}

	fn overflow(&self) -> taffy::Point<taffy::Overflow> {
		self.get_or::<Overflow>(taffy::Style::DEFAULT.overflow)
	}

	fn scrollbar_width(&self) -> f32 {
		self.get_or::<ScrollbarWidth>(0.0)
	}

	fn position(&self) -> taffy::Position {
		self.get_or::<Position>(taffy::Style::DEFAULT.position)
	}

	fn inset(&self) -> taffy::Rect<taffy::LengthPercentageAuto> {
		self.get_or::<Inset>(taffy::Style::DEFAULT.inset)
	}

	fn size(&self) -> taffy::Size<taffy::Dimension> {
		self.get_or::<Size>(taffy::Style::DEFAULT.size)
	}

	fn min_size(&self) -> taffy::Size<taffy::Dimension> {
		self.get_or::<MinSize>(taffy::Style::DEFAULT.min_size)
	}

	fn max_size(&self) -> taffy::Size<taffy::Dimension> {
		self.get_or::<MaxSize>(taffy::Style::DEFAULT.max_size)
	}

	fn aspect_ratio(&self) -> Option<f32> {
		self.get::<AspectRatio>()
	}

	fn margin(&self) -> taffy::Rect<taffy::LengthPercentageAuto> {
		self.get_or::<Margin>(taffy::Style::DEFAULT.margin)
	}

	fn padding(&self) -> taffy::Rect<taffy::LengthPercentage> {
		self.get_or::<Padding>(taffy::Style::DEFAULT.padding)
	}

	fn border(&self) -> taffy::Rect<taffy::LengthPercentage> {
		self.get_or::<Border>(taffy::Style::DEFAULT.border)
	}
}
