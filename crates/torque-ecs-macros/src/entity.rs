use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_quote, ItemStruct, Type};

pub fn derive(tokens: TokenStream) -> syn::Result<TokenStream> {
	let item_struct: ItemStruct = syn::parse2(tokens)?;
	let ident = &item_struct.ident;
	let name = ident.to_string();
	let base_type = if let Some(extends) = item_struct
		.attrs
		.iter()
		.find(|v| v.path().is_ident("extends"))
	{
		extends.parse_args::<Type>()?
	} else {
		parse_quote! { () }
	};

	let type_id_ident = format_ident!("{}_TYPE_ID", ident.to_string().to_uppercase());
	let type_ids_ident = format_ident!("{}_TYPE_IDS", ident.to_string().to_uppercase());

	Ok(quote! {
		static #type_id_ident: ::std::sync::LazyLock<::std::any::TypeId> = ::std::sync::LazyLock::new(|| ::std::any::TypeId::of::<#ident>());
		static #type_ids_ident: ::std::sync::LazyLock<Vec<::std::any::TypeId>> = ::std::sync::LazyLock::new(|| {
			let mut type_ids = vec![*#type_id_ident];

			type_ids.extend(<#base_type as ::torque_ecs::Entity>::type_ids());

			type_ids
		});

		impl ::torque_ecs::Entity for #ident {
			const NAME: &str = #name;
			type Base = #base_type;

			fn type_id() -> ::std::any::TypeId {
				*#type_id_ident
			}

			fn type_ids() -> &'static [::std::any::TypeId] {
				&*#type_ids_ident
			}
		}

		impl ::torque_ecs::Extends<#base_type> for #ident {}
	})
}
