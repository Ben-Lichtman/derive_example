mod common;
mod derive_enum;
mod derive_struct;

use crate::{derive_enum::generate_enum, derive_struct::generate_struct};
use proc_macro2::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(DummyTrait, attributes())]
pub fn derive_mutator(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let derive_input = parse_macro_input!(input as DeriveInput);
	proc_macro::TokenStream::from(process_input(&derive_input))
}

fn process_input(input: &DeriveInput) -> TokenStream {
	match &input.data {
		syn::Data::Struct(s) => generate_struct(&input.ident, s, &input.generics),
		syn::Data::Enum(e) => generate_enum(&input.ident, e, &input.generics),
		syn::Data::Union(_) => unimplemented!(),
	}
}
