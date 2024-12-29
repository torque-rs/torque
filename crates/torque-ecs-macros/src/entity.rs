use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{punctuated::Punctuated, token::Comma, Ident, ItemStruct};

pub fn derive(tokens: TokenStream) -> syn::Result<TokenStream> {
	let item_struct: ItemStruct = syn::parse2(tokens)?;
	let ident = &item_struct.ident;
	let name = ident.to_string();

	let type_ids = if let Some(parents) = item_struct
		.attrs
		.iter()
		.find(|v| v.path().is_ident("parents"))
	{
		parents
			.parse_args_with(Punctuated::<Ident, Comma>::parse_terminated)?
			.iter()
			.cloned()
			.collect::<Vec<Ident>>()
	} else {
		Vec::default()
	};

	let type_id_ident = format_ident!("{}_TYPE_ID", ident.to_string().to_uppercase());
	let type_ids_ident = format_ident!("{}_TYPE_IDS", ident.to_string().to_uppercase());

	let type_id_count = type_ids.len() + 1;

	Ok(quote! {
		static #type_id_ident: ::std::sync::LazyLock<::std::any::TypeId> = ::std::sync::LazyLock::new(|| ::std::any::TypeId::of::<#ident>());
		static #type_ids_ident: ::std::sync::LazyLock<[::std::any::TypeId; #type_id_count]> = ::std::sync::LazyLock::new(|| [#ident::type_id(), #(#type_ids::type_id(),)*]);

		impl ::torque_ecs::Entity for #ident {
			const NAME: &str = #name;

			fn type_id() -> ::std::any::TypeId {
				*#type_id_ident
			}

			fn type_ids() -> &'static [::std::any::TypeId] {
				&*#type_ids_ident
			}
		}
	})
}
