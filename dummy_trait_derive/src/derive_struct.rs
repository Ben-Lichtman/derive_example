use crate::common::extra_where_predicates;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, DataStruct, Generics, Ident, WherePredicate};

pub fn generate_struct(ident: &Ident, _data: &DataStruct, generics: &mut Generics) -> TokenStream {
	extra_where_predicates(generics, |p| -> WherePredicate {
		let i = &p.ident;
		parse_quote! {
			#i: ::dummy_trait::DummyTrait
		}
	});
	let (impl_generics, ty_generics, whereclause) = generics.split_for_impl();
	quote! {
		impl #impl_generics ::dummy_trait::DummyTrait for #ident #ty_generics #whereclause {}
	}
}
