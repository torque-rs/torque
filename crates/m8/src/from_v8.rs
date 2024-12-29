use core::fmt;

use serde::Deserialize;
use serde_v8::from_v8;

use crate::{V8Type, V8TypeGarbageCollected};

#[derive(Debug, thiserror::Error)]
pub enum TryFromV8Error {
	ExpectedObject(v8::DataError),
	ExpectedInternalField0,
	ExpectedInternalField0BigInt(v8::DataError),
	ExpectedInternalField1,
	ExpectedInternalField1External(v8::DataError),
	ContextNotInitialized,
	TypeMismatch,
	Serde(#[from] serde_v8::Error),
}

impl fmt::Display for TryFromV8Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}

pub trait FromV8 {
	fn from_v8(scope: &mut v8::HandleScope, value: v8::Local<v8::Value>) -> Self;
}

pub trait TryFromV8: Sized {
	fn try_from_v8(
		scope: &mut v8::HandleScope,
		value: v8::Local<v8::Value>,
	) -> Result<Self, TryFromV8Error>;
}

impl<T> FromV8 for T
where
	T: TryFromV8,
{
	fn from_v8(scope: &mut v8::HandleScope, value: v8::Local<v8::Value>) -> Self {
		<T as TryFromV8>::try_from_v8(scope, value).expect("from_v8")
	}
}

/*impl<T> TryFromV8 for Arc<T>
where
	T: V8Type,
{
	fn try_from_v8(
		scope: &mut v8::HandleScope,
		value: v8::Local<v8::Data>,
	) -> Result<Self, TryFromV8Error> {
		let object = value
			.try_cast::<v8::Object>()
			.map_err(TryFromV8Error::ExpectedObject)?;
		let type_id = object
			.get_internal_field(scope, 0)
			.ok_or(TryFromV8Error::ExpectedInternalField0)?;
		let type_id = type_id
			.try_cast::<v8::BigInt>()
			.map_err(TryFromV8Error::ExpectedInternalField0BigInt)?;
		let (type_id, lossless) = type_id.u64_value();
		let expected_type_id = T::id(scope).ok_or(TryFromV8Error::ContextNotInitialized)?;

		if !lossless || type_id != expected_type_id {
			return Err(TryFromV8Error::TypeMismatch);
		}

		let this = object
			.get_internal_field(scope, 1)
			.ok_or(TryFromV8Error::ExpectedInternalField1)?;
		let this = this
			.try_cast::<v8::External>()
			.map_err(TryFromV8Error::ExpectedInternalField1External)?;

		Ok(unsafe { Arc::from_raw(this.value() as *const T) })
	}
}*/

impl<T> TryFromV8 for v8::cppgc::Ptr<T>
where
	T: V8TypeGarbageCollected,
{
	fn try_from_v8(
		scope: &mut v8::HandleScope,
		value: v8::Local<v8::Value>,
	) -> Result<Self, TryFromV8Error> {
		let wrapper = value
			.try_cast::<v8::Object>()
			.map_err(TryFromV8Error::ExpectedObject)?;

		let ptr =
			<T as V8TypeGarbageCollected>::unwrap(scope, wrapper).ok_or(TryFromV8Error::TypeMismatch)?;

		Ok(ptr)
	}
}

impl<'de, T> TryFromV8 for T
where
	T: V8Type + Deserialize<'de>,
{
	fn try_from_v8(
		scope: &mut v8::HandleScope,
		value: v8::Local<v8::Value>,
	) -> Result<Self, TryFromV8Error> {
		Ok(from_v8(scope, value)?)
	}
}
