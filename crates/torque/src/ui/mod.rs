mod children;
mod element;
mod image;
mod node;
mod node_id;
mod node_ref;
mod text;
mod tree;
mod window;

use crate::Compiler;

pub use self::{
	children::Children,
	element::{Element, ElementContent},
	image::ImageContent,
	node::{Node, NodeContent, NodeKind},
	node_id::NodeId,
	node_ref::NodeRef,
	text::{Text, TextContent},
	tree::Tree,
	window::{CreateWindowError, Window},
};

pub(crate) use self::tree::Inner;

pub fn init(scope: &mut v8::HandleScope) {
	let specifier = "@torque-rs/ui";

	let module_name = v8::String::new(scope, specifier).unwrap();
	let export_names = [v8::String::new(scope, "Window").unwrap()];

	let module = v8::Module::create_synthetic_module(scope, module_name, &export_names, evaluate);
	let module = v8::Global::new(scope, module);
	let context = scope.get_current_context();

	let compiler = context.get_slot::<Compiler>().expect("current context");

	compiler.add_module(specifier.into(), module);
}

fn evaluate<'a>(
	context: v8::Local<'a, v8::Context>,
	module: v8::Local<'a, v8::Module>,
) -> Option<v8::Local<'a, v8::Value>> {
	let scope = &mut unsafe { v8::CallbackScope::new(context) };
	let scope = &mut v8::EscapableHandleScope::new(scope);
	let scope = &mut v8::ContextScope::new(scope, context);

	window::init(scope, &module);

	let value = v8::Boolean::new(scope, true).into();

	Some(scope.escape(value))
}

m8::module! {
	name: "@torque-rs/ui",
	exports: [
		Window
	]
}
