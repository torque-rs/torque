mod class;
mod from_v8;
mod into_v8;
mod tags;
mod v8type;

pub use m8_macros::{class, module};

pub use inventory;
use scoped_thread_local::scoped_thread_local;

pub use self::{
	class::{Class, ClassContext},
	from_v8::{FromV8, TryFromV8},
	into_v8::{IntoV8, TryIntoV8},
	tags::Tags,
	v8type::{V8Type, V8TypeGarbageCollected, V8TypeInfo},
};

scoped_thread_local! {
	static CURRENT_SCOPE: for <'s> v8::HandleScope<'s>
}

pub fn enter_scope<R>(scope: &mut v8::HandleScope, f: impl FnOnce() -> R) -> R {
	CURRENT_SCOPE.set(scope, f)
}

pub fn with_scope<R>(f: impl FnOnce(&mut v8::HandleScope) -> R) -> R {
	CURRENT_SCOPE.with(f)
}

#[macro_export]
macro_rules! throw_error {
	($scope: ident, $message: expr) => {
		let message = v8::String::new($scope, $message).unwrap();
		let exception = v8::Exception::error($scope, message);

		$scope.throw_exception(exception);
	};
}

pub struct ExportDefinition {}

pub struct ModuleDefinition {
	name: &'static str,
	//exports: &'static [ExportDefinition],
}

impl ModuleDefinition {
	pub const fn new(name: &'static str) -> Self {
		static EXPORTS: &[ExportDefinition] = &[];

		Self {
			name,
			//exports: EXPORTS,
		}
	}

	fn init(&self, scope: &mut v8::HandleScope) {
		println!("adding {}", self.name);
	}
}

inventory::collect!(ModuleDefinition);

pub fn init(scope: &mut v8::HandleScope) {
	for module in inventory::iter::<ModuleDefinition> {
		module.init(scope);
	}
}