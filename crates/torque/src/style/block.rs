use crate::properties;

properties! {
	TextAlign: taffy::TextAlign
}

impl taffy::BlockContainerStyle for super::Style {
	fn text_align(&self) -> taffy::TextAlign {
		taffy::Style::DEFAULT.text_align
	}
}

impl taffy::BlockItemStyle for super::Style {
	fn is_table(&self) -> bool {
		false
	}
}
