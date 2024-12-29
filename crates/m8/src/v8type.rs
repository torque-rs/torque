pub trait V8TypeInfo {
	fn object_template<'s>(
		&self,
		scope: &mut v8::HandleScope<'s>,
	) -> v8::Local<'s, v8::ObjectTemplate>;
}

pub trait V8Type {
	type Info: V8TypeInfo + 'static;

	fn object_template<'s>(
		scope: &mut v8::HandleScope<'s>,
	) -> Option<v8::Local<'s, v8::ObjectTemplate>> {
		let context = scope.get_current_context();

		match context.get_slot::<Self::Info>() {
			Some(info) => {
				let object_template = <Self::Info as V8TypeInfo>::object_template(info, scope);

				Some(object_template)
			}
			None => todo!(),
		}
	}
}

pub trait V8TypeGarbageCollected: V8Type + v8::cppgc::GarbageCollected + Sized {
	fn wrap(scope: &mut v8::HandleScope, wrapper: v8::Local<v8::Object>, ptr: v8::cppgc::Ptr<Self>);
	fn unwrap(
		scope: &mut v8::HandleScope,
		wrapper: v8::Local<v8::Object>,
	) -> Option<v8::cppgc::Ptr<Self>>;
}
