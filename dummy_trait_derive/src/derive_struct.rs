use crate::common::{extra_where_predicates, FieldsExtender};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, DataStruct, Generics, Ident, WherePredicate};

pub fn generate_struct(ident: &Ident, data: &DataStruct, generics: &mut Generics) -> TokenStream {
	extra_where_predicates(generics, |p| -> WherePredicate {
		let i = &p.ident;
		parse_quote! {
			#i: ::dummy_trait::DummyTrait
		}
	});

	construct(ident, data, generics)
}

fn construct(ident: &Ident, data: &DataStruct, generics: &Generics) -> TokenStream {
	let (impl_generics, ty_generics, whereclause) = generics.split_for_impl();
	let expander = FieldsExtender::from_fields(&data.fields);
	let expanded = expander.expand_fields();

	let lines = expander.idents().map(|i| quote! { #i.foo(); });

	quote! {
		impl #impl_generics ::dummy_trait::DummyTrait for #ident #ty_generics #whereclause {
			fn foo(&self) {
				let Self #expanded = self;
				#(#lines)*
			}
		}
	}
}
