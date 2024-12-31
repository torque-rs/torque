use tracing::trace;

pub struct Module {
	specifier: &'static str,
	init_fn:
		&'static (dyn for<'s> Fn(&mut v8::HandleScope<'s>) -> v8::Local<'s, v8::Module> + Send + Sync),
}

impl Module {
	pub const fn new(
		specifier: &'static str,
		init_fn: &'static (dyn for<'s> Fn(&mut v8::HandleScope<'s>) -> v8::Local<'s, v8::Module>
		            + Send
		            + Sync),
	) -> Self {
		Self { specifier, init_fn }
	}

	pub(crate) fn init(
		&self,
		scope: &mut v8::HandleScope,
		init_callback: impl for<'s> Fn(&mut v8::HandleScope<'s>, &str, v8::Local<'s, v8::Module>),
	) {
		trace!("initializing {}", self.specifier);

		let module = (self.init_fn)(scope);

		init_callback(scope, self.specifier, module);
	}
}
