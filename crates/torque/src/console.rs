use std::io::{stdout, Write};

pub struct Console {}

impl Console {
	pub fn log<const N: usize>(_args: [impl Into<v8::Value>; N]) {}
}

fn log(scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, mut _rv: v8::ReturnValue) {
	for i in 0..args.length() {
		let arg = args.get(i);

		stdout()
			.write_all(arg.to_rust_string_lossy(scope).as_bytes())
			.unwrap();

		print!(" ");
	}

	println!();
}

pub fn init(scope: &mut v8::HandleScope) {
	let context = scope.get_current_context();
	let global = context.global(scope);

	{
		let key = v8::String::new(scope, "console").unwrap().into();

		let console = global.get(scope, key).unwrap();

		println!("console: {}", console.to_rust_string_lossy(scope));

		let key = v8::String::new(scope, "log").unwrap().into();
		let log = console.cast::<v8::Object>().get(scope, key).unwrap();

		println!("log: {}", log.to_rust_string_lossy(scope));
	}

	let console = v8::Object::new(scope);
	let log = v8::Function::new(scope, log).unwrap();

	let key = v8::String::new(scope, "log").unwrap().into();

	console.set(scope, key, log.into());

	let key = v8::String::new(scope, "console").unwrap().into();

	global.set(scope, key, console.into());
}
