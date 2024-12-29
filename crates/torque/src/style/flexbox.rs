use crate::properties;

properties! {
	FlexDirection: taffy::FlexDirection,
	FlexWrap: taffy::FlexWrap,
	Gap: taffy::Size<taffy::LengthPercentage>,
	AlignContent: taffy::AlignContent,
	AlignItems: taffy::AlignItems,
	JustifyContent: taffy::JustifyContent
}

impl taffy::FlexboxContainerStyle for super::Style {
	fn flex_direction(&self) -> taffy::FlexDirection {
		self.get_or::<FlexDirection>(taffy::Style::DEFAULT.flex_direction)
	}

	fn flex_wrap(&self) -> taffy::FlexWrap {
		self.get_or::<FlexWrap>(taffy::Style::DEFAULT.flex_wrap)
	}

	fn gap(&self) -> taffy::Size<taffy::LengthPercentage> {
		self.get_or::<Gap>(taffy::Style::DEFAULT.gap)
	}

	fn align_content(&self) -> Option<taffy::AlignContent> {
		self.get::<AlignContent>()
	}

	fn align_items(&self) -> Option<taffy::AlignItems> {
		self.get::<AlignItems>()
	}

	fn justify_content(&self) -> Option<taffy::JustifyContent> {
		self.get::<JustifyContent>()
	}
}

properties! {
	FlexBasis: taffy::Dimension,
	FlexGrow: f32,
	FlexShrink: f32,
	AlignSelf: taffy::AlignSelf
}

impl taffy::FlexboxItemStyle for super::Style {
	fn flex_basis(&self) -> taffy::Dimension {
		self.get_or::<FlexBasis>(taffy::Style::DEFAULT.flex_basis)
	}

	fn flex_grow(&self) -> f32 {
		self.get_or::<FlexGrow>(taffy::Style::DEFAULT.flex_grow)
	}

	fn flex_shrink(&self) -> f32 {
		self.get_or::<FlexShrink>(taffy::Style::DEFAULT.flex_shrink)
	}

	fn align_self(&self) -> Option<taffy::AlignSelf> {
		self.get::<AlignSelf>()
	}
}
