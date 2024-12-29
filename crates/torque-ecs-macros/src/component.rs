use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

pub fn derive(tokens: TokenStream) -> syn::Result<TokenStream> {
	let item_struct: ItemStruct = syn::parse2(tokens)?;
	let ident = &item_struct.ident;
	let name = ident.to_string();

	Ok(quote! {
		impl ::torque_ecs::Component for  #ident {
			const NAME: &str = #name;

			type Value = Self;
		}
	})
}
