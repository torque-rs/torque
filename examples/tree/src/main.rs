use cosmic_text::FontSystem;
use torque::{
	style::{
		core::{Border, MinSize, Padding},
		flexbox::{FlexDirection, FlexGrow},
		Layout,
	},
	ui::{Node, Tree},
};

fn main() {
	pretty_env_logger::init();

	let font_system = FontSystem::new();

	let mut tree = Tree::new(font_system);

	let root = tree.create_element();
	let child1 = tree.create_text("Hello World!");
	let child2 = tree.create_text("Banana!");
	let child3 = tree.create_text("Terracotta pie!");

	root.with_style_mut(|style| {
		style
			.set::<Layout>(Layout::FlexBox)
			.set::<FlexDirection>(taffy::FlexDirection::Row);
	});

	child1.with_style_mut(|style| {
		style
			.set::<MinSize>(taffy::Size::from_percent(50.0, 50.0))
			.set::<FlexGrow>(100.0);
	});
	child2.with_style_mut(|style| {
		style
			.set::<MinSize>(taffy::Size::from_percent(50.0, 50.0))
			.set::<Border>(taffy::Rect::length(10.0))
			.set::<FlexGrow>(30.0);
	});
	child3.with_style_mut(|style| {
		style
			.set::<MinSize>(taffy::Size::from_percent(50.0, 50.0))
			.set::<Border>(taffy::Rect::length(10.0))
			.set::<FlexGrow>(30.0);
	});

	root.append_child(child1);
	root.append_child(child2.clone());
	root.append_child(child3);

	tree.print(root.id());

	tree.compute_layout(
		root.id(),
		taffy::Size {
			height: taffy::AvailableSpace::Definite(100.0),
			width: taffy::AvailableSpace::Definite(100.0),
		},
		true,
	);

	tree.print(root.id());

	root.remove_child(child2);

	tree.compute_layout(
		root.id(),
		taffy::Size {
			height: taffy::AvailableSpace::Definite(100.0),
			width: taffy::AvailableSpace::Definite(100.0),
		},
		true,
	);

	tree.print(root.id());
}
