use crate::common::{extra_where_predicates, FieldsExpander};
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

fn construct_fn_inner(data: &DataEnum) -> TokenStream {
	let variant_info = data
		.variants
		.iter()
		.map(|v| (&v.ident, &v.fields))
		.collect::<Vec<_>>();
	let match_arms = variant_info.iter().map(|(i, f)| {
		let expanded = FieldsExpander::from_fields(f).expand_fields();
		quote! {
			Self::#i #expanded => {}
		}
	});
	quote! {
		match self {
			#(#match_arms),*
		}
	}
}
