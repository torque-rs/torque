mod component;
mod entity;

use proc_macro::TokenStream;

#[proc_macro_derive(Entity, attributes(extends))]
pub fn derive_entity(tokens: TokenStream) -> TokenStream {
	match entity::derive(tokens.into()) {
		Ok(tokens) => tokens,
		Err(error) => error.into_compile_error(),
	}
	.into()
}

#[proc_macro_derive(Component)]
pub fn derive_component(tokens: TokenStream) -> TokenStream {
	match component::derive(tokens.into()) {
		Ok(tokens) => tokens,
		Err(error) => error.into_compile_error(),
	}
	.into()
}
