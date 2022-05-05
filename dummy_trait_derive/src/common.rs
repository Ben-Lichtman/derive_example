use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Fields, Generics, Ident, TypeParam, WherePredicate};

pub struct FieldsExtender<'a> {
	fields: &'a Fields,
}

impl<'a> FieldsExtender<'a> {
	pub fn from_fields(fields: &'a Fields) -> Self { Self { fields } }

	pub fn expand_fields(&self) -> TokenStream {
		match self.fields {
			Fields::Unit => quote! {},
			Fields::Unnamed(fields) => {
				let info = fields.unnamed.iter().enumerate();

				let expanded_inner = info.map(|(n, _)| {
					let ident = tuple_ident(n);
					quote! { #ident }
				});

				quote! { ( #(#expanded_inner),* ) }
			}
			Fields::Named(fields) => {
				let info = fields.named.iter().enumerate();

				let expanded_inner = info.map(|(n, f)| {
					let ident = f.ident.as_ref().map(|i| regular_ident(n, i));
					quote! { #ident }
				});

				quote! { { #(#expanded_inner),* } }
			}
		}
	}

	pub fn idents(&self) -> impl Iterator<Item = Ident> + 'a {
		self.fields.iter().enumerate().map(|(n, f)| match &f.ident {
			Some(i) => regular_ident(n, i),
			None => tuple_ident(n),
		})
	}
}

pub fn regular_ident(n: usize, ident: &Ident) -> Ident { format_ident!("{}", ident) }

pub fn tuple_ident(n: usize) -> Ident { format_ident!("field_{}", n) }

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
