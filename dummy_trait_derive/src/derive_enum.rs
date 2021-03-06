use crate::common::{extra_where_predicates, FieldsExtender};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, DataEnum, Generics, Ident, WherePredicate};

pub fn generate_enum(ident: &Ident, data: &DataEnum, generics: &mut Generics) -> TokenStream {
	extra_where_predicates(generics, |p| -> WherePredicate {
		let i = &p.ident;
		parse_quote! {
			#i: ::dummy_trait::DummyTrait
		}
	});

	construct(ident, data, generics)
}

fn construct(ident: &Ident, data: &DataEnum, generics: &Generics) -> TokenStream {
	let (impl_generics, ty_generics, whereclause) = generics.split_for_impl();

	let variant_info = data
		.variants
		.iter()
		.map(|v| (&v.ident, &v.fields))
		.collect::<Vec<_>>();
	let match_arms = variant_info.iter().map(|(i, f)| {
		let expander = FieldsExtender::from_fields(f);
		let expanded = expander.expand_fields();

		let lines = expander.info().map(|(i, _)| quote! { #i.foo(); });

		quote! {
			Self::#i #expanded => { #(#lines)* }
		}
	});
	quote! {
		impl #impl_generics ::dummy_trait::DummyTrait for #ident #ty_generics #whereclause {
			fn foo(&self) {
				match self {
					#(#match_arms),*
				}
			}
		}
	}
}
