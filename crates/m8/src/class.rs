pub trait ClassContext {
	fn object_template<'s>(
		&self,
		scope: &mut v8::HandleScope<'s>,
	) -> v8::Local<'s, v8::ObjectTemplate>;
}

pub trait Class: v8::cppgc::GarbageCollected + Sized {
	type Context: ClassContext + 'static;

	const TAG: u16;

	fn init(scope: &mut v8::HandleScope, module: &v8::Local<v8::Module>);

	fn constructor(
		_scope: &mut v8::HandleScope,
		_args: v8::FunctionCallbackArguments,
		mut _rv: v8::ReturnValue,
	) {
	}

	fn object_template<'s>(
		scope: &mut v8::HandleScope<'s>,
	) -> Option<v8::Local<'s, v8::ObjectTemplate>> {
		let context = scope.get_current_context();

		context.get_slot::<Self::Context>().map(|class_context| {
			let object_template = <Self::Context as ClassContext>::object_template(class_context, scope);

			object_template
		})
	}

	fn wrap(scope: &mut v8::HandleScope, wrapper: v8::Local<v8::Object>, ptr: v8::cppgc::Ptr<Self>);
	fn unwrap(
		scope: &mut v8::HandleScope,
		wrapper: v8::Local<v8::Object>,
	) -> Option<v8::cppgc::Ptr<Self>>;
}
