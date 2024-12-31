fn create_element(
	scope: &mut v8::HandleScope,
	args: v8::FunctionCallbackArguments,
	mut rv: v8::ReturnValue,
) {
	println!("createElement");

	let type_ = args.get(0);

	if type_.is_undefined() {
		let message = v8::String::new(scope, "type not specified").unwrap().into();

		scope.throw_exception(message);

		return;
	}

	if let Ok(function) = type_.try_cast::<v8::Function>() {
		println!(
			"type: {}",
			function.get_name(scope).to_rust_string_lossy(scope)
		);
		let element = v8::Object::new(scope);

		let key = v8::String::new(scope, "type").unwrap();
		element.set(scope, key.into(), type_);

		if let Ok(props) = args.get(1).try_cast::<v8::Object>() {
			let key = v8::String::new(scope, "ref").unwrap().into();
			if let Some(value) = props.get(scope, key) {
				element.set(scope, key, value);

				props.delete(scope, key);
			}

			let key = v8::String::new(scope, "key").unwrap().into();
			if let Some(value) = props.get(scope, key) {
				element.set(scope, key, value);

				props.delete(scope, key);
			}

			let key = v8::String::new(scope, "props").unwrap();
			element.set(scope, key.into(), props.into());
		}

		rv.set(element.into());
	} else if let Ok(string) = type_.try_cast::<v8::String>() {
		println!("{}", string.to_rust_string_lossy(scope));
	}
}

fn create_fragment(
	scope: &mut v8::HandleScope,
	args: v8::FunctionCallbackArguments,
	mut rv: v8::ReturnValue,
) {
	println!("createFragment");
}

pub fn __m8_init(scope: &mut v8::HandleScope) {
	let specifier = "@torque-rs/jsx-runtime";

	let module_name = v8::String::new(scope, specifier).unwrap();
	let export_names = [
		v8::String::new(scope, "jsx").unwrap(),
		v8::String::new(scope, "jsxs").unwrap(),
		v8::String::new(scope, "Fragment").unwrap(),
	];

	let module = v8::Module::create_synthetic_module(scope, module_name, &export_names, evaluate);
}

fn evaluate<'a>(
	context: v8::Local<'a, v8::Context>,
	module: v8::Local<'a, v8::Module>,
) -> Option<v8::Local<'a, v8::Value>> {
	let scope = &mut unsafe { v8::CallbackScope::new(context) };
	let scope = &mut v8::EscapableHandleScope::new(scope);
	let scope = &mut v8::ContextScope::new(scope, context);

	let export_name = v8::String::new(scope, "jsx").unwrap();
	let export_value = v8::Function::new(scope, create_element).unwrap().into();
	module
		.set_synthetic_module_export(scope, export_name, export_value)
		.unwrap();

	let export_name = v8::String::new(scope, "jsxs").unwrap();
	let export_value = v8::Function::new(scope, create_element).unwrap().into();
	module
		.set_synthetic_module_export(scope, export_name, export_value)
		.unwrap();

	let export_name = v8::String::new(scope, "Fragment").unwrap();
	let export_value = v8::Function::new(scope, create_fragment).unwrap().into();
	module
		.set_synthetic_module_export(scope, export_name, export_value)
		.unwrap();

	let value = v8::Boolean::new(scope, true).into();

	Some(scope.escape(value))
}

m8::module! {
	name: "@torque-rs/jsx-runtime",
	exports: [
		fn create_element as jsx,
		fn create_element as jsxs,
		fn create_fragment as Fragment
	]
}
