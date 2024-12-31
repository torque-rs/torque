use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn main(attr_tokens: TokenStream, item_tokens: TokenStream) -> TokenStream {
	match main::generate(attr_tokens.into(), item_tokens.into()) {
		Ok(tokens) => tokens,
		Err(error) => error.into_compile_error(),
	}
	.into()
}

mod main {
	use std::mem::replace;

	use proc_macro2::{Span, TokenStream};
	use proc_macro_crate::{crate_name, FoundCrate};
	use quote::quote;

	pub fn generate(_attr_tokens: TokenStream, item_tokens: TokenStream) -> syn::Result<TokenStream> {
		let mut item_fn: syn::ItemFn = syn::parse2(item_tokens)?;
		let inner_ident: syn::Ident = syn::parse_quote! { _inner };
		let ident = replace(&mut item_fn.sig.ident, inner_ident.clone());
		let crate_path = crate_path()?;

		let run = if item_fn.sig.asyncness.is_some() {
			quote! { #crate_path::Runtime::run(#inner_ident) }
		} else {
			quote! { #crate_path::Runtime::run_sync(#inner_ident) }
		};

		Ok(quote! {
			pub fn #ident() -> Result<(), #crate_path::RuntimeError> {
				#item_fn

				// This is an ugly hack to work around https://github.com/dtolnay/inventory/issues/52 which is ultimately caused by
				// https://github.com/rust-lang/rust/issues/133491
				#[cfg(target_os = "macos")]
				static __M8_LINKER_HACK: ::std::sync::LazyLock<u8> =
					::std::sync::LazyLock::new(|| *::torque::jsx_runtime::__M8_LINKER_HACK + *::torque::ui::__M8_LINKER_HACK);

				#run?
			}
		})
	}

	fn found_crate(found_crate: FoundCrate, crate_tokens: TokenStream) -> TokenStream {
		match found_crate {
			FoundCrate::Itself => crate_tokens,
			FoundCrate::Name(name) => {
				let ident = syn::Ident::new(&name, Span::call_site());

				quote! {
					::#ident
				}
			}
		}
	}

	fn crate_path() -> syn::Result<TokenStream> {
		crate_name("torque")
			.map(|v| found_crate(v, quote! { ::torque }))
			.or_else(|_| {
				crate_name("torque-runtime").map(|v| found_crate(v, quote! { ::torque_runtime }))
			})
			.map_err(|error| syn::Error::new(Span::call_site(), format!("{}", error)))
	}
}
