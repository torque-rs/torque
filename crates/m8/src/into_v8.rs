use std::fmt;

use serde::Serialize;
use serde_v8::to_v8;

use crate::{V8Type, V8TypeGarbageCollected};

#[derive(Debug, thiserror::Error)]
pub enum IntoV8Error {
	ContextNotInitialized,
	NewInstanceFailed,
	Serde(#[from] serde_v8::Error),
}

impl fmt::Display for IntoV8Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

pub trait IntoV8 {
	fn into_v8<'s>(scope: &'s mut v8::HandleScope<'s>, value: Self) -> v8::Local<'s, v8::Value>;
}

pub trait TryIntoV8 {
	fn try_into_v8<'s>(
		scope: &'s mut v8::HandleScope<'s>,
		value: Self,
	) -> Result<v8::Local<'s, v8::Value>, IntoV8Error>;
}

impl<T> IntoV8 for T
where
	T: TryIntoV8,
{
	fn into_v8<'s>(scope: &'s mut v8::HandleScope<'s>, value: Self) -> v8::Local<'s, v8::Value> {
		<T as TryIntoV8>::try_into_v8(scope, value).expect("into_v8")
	}
}

impl<T> TryIntoV8 for v8::cppgc::Ptr<T>
where
	T: V8TypeGarbageCollected,
{
	fn try_into_v8<'s>(
		scope: &mut v8::HandleScope<'s>,
		value: Self,
	) -> Result<v8::Local<'s, v8::Value>, IntoV8Error> {
		let object_template = T::object_template(scope).ok_or(IntoV8Error::ContextNotInitialized)?;
		let object = object_template
			.new_instance(scope)
			.ok_or(IntoV8Error::NewInstanceFailed)?;

		<T as V8TypeGarbageCollected>::wrap(scope, object, value);

		Ok(object.into())
	}
}

impl<T> TryIntoV8 for T
where
	T: V8Type + Serialize,
{
	fn try_into_v8<'s>(
		scope: &'s mut v8::HandleScope<'s>,
		value: Self,
	) -> Result<v8::Local<'s, v8::Value>, IntoV8Error> {
		Ok(to_v8(scope, value)?)
	}
}
