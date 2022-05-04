use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Fields, FieldsNamed, FieldsUnnamed, Generics, TypeParam, WherePredicate};

pub enum FieldsExpander<'a> {
	Unit,
	Tuple(&'a FieldsUnnamed),
	Named(&'a FieldsNamed),
}

impl<'a> FieldsExpander<'a> {
	pub fn from_fields(fields: &'a Fields) -> Self {
		match fields {
			Fields::Unit => FieldsExpander::Unit,
			Fields::Unnamed(fields) => FieldsExpander::Tuple(fields),
			Fields::Named(fields) => FieldsExpander::Named(fields),
		}
	}

	pub fn expand_fields(&self) -> TokenStream {
		match self {
			Self::Unit => quote! {},
			Self::Tuple(fields) => {
				let info = fields.unnamed.iter().enumerate().map(|(n, f)| (n, &f.ty));

				let expanded_inner = info.map(|(n, t)| {
					let ident = format_ident!("field_{}", n);
					quote! { #ident }
				});

				quote! { ( #(#expanded_inner),* ) }
			}
			Self::Named(fields) => {
				let info = fields
					.named
					.iter()
					.enumerate()
					.map(|(n, f)| (n, f.ident.as_ref(), &f.ty));

				let expanded_inner = info.map(|(n, i, t)| quote! { #i });

				quote! { { #(#expanded_inner),* } }
			}
		}
	}
}

pub fn extra_where_predicates(
	generics: &mut Generics,
	make_predicate: impl Fn(&TypeParam) -> WherePredicate,
) {
	let extra_where_predicates = generics
		.params
		.iter()
		.filter_map(|p| match p {
			syn::GenericParam::Type(t) => Some(t),
			_ => None,
		})
		.map(make_predicate)
		.collect::<Vec<_>>();
	generics
		.make_where_clause()
		.predicates
		.extend(extra_where_predicates);
}
