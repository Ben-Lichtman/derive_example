use crate::common::{extra_where_predicates, FieldsExpander};
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
	let (impl_generics, ty_generics, whereclause) = generics.split_for_impl();

	let fn_inner = construct_fn_inner(data);

	quote! {
		impl #impl_generics ::dummy_trait::DummyTrait for #ident #ty_generics #whereclause {
			fn foo(&self) {
				#fn_inner
			}
		}
	}
}

fn construct_fn_inner(data: &DataStruct) -> TokenStream {
	let expanded = FieldsExpander::from_fields(&data.fields).expand_fields();
	quote! { let Self #expanded = self; }
}
