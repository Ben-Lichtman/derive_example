use syn::{Generics, TypeParam, WherePredicate};

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
