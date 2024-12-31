pub type ExportInitFn = Box<dyn FnOnce(&mut v8::HandleScope, &v8::Local<v8::Module>) + Send + Sync>;

pub struct Export {
	name: &'static str,
	init_fn: ExportInitFn,
}

impl Export {
	pub fn new(name: &'static str, init_fn: ExportInitFn) -> Self {
		Self { name, init_fn }
	}
}
