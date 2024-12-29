use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Generics, WhereClause, WherePredicate};

/// Merges two sets of generic parameters into a single unified set.
///
/// This function combines generic parameters and where clauses from two different
/// generic specifications into a single coherent set. It preserves all type parameters,
/// lifetime parameters, and where clause predicates from both inputs.
///
/// # Arguments
///
/// * `generics_a` - First set of generic parameters to merge
/// * `generics_b` - Second set of generic parameters to merge
///
/// # Returns
///
/// Returns a new [`syn::Generics`] instance containing:
/// - Combined generic parameters from both inputs
/// - Merged where clauses (if any exist in either input)
/// - Angle bracket tokens from the first input (`generics_a`)
///
/// # Examples
///
/// ```rust
/// # use syn::{parse_quote, Generics};
/// # use cgp_component_macro_lib::delegate_components::merge_generics::merge_generics;
/// # use std::fmt::{Debug, Display};
/// let generics_a: Generics = parse_quote!(<T: Debug>);
/// let generics_b: Generics = parse_quote!(<U: Display>);
/// let merged = merge_generics(&generics_a, &generics_b);
/// // Results in: <T: Debug, U: Display>
/// ```
///
/// # Note
///
/// The function preserves the angle bracket tokens (`lt_token` and `gt_token`) from
/// the first set of generics (`generics_a`). This is typically desirable as these
/// tokens usually carry the same span information throughout a macro expansion.
pub fn merge_generics(generics_a: &Generics, generics_b: &Generics) -> Generics {
    let mut params = generics_a.params.clone();
    params.extend(generics_b.params.clone());

    let mut predicates: Punctuated<WherePredicate, Comma> = Default::default();

    if let Some(where_clause) = &generics_a.where_clause {
        predicates.extend(where_clause.predicates.clone());
    }

    if let Some(where_clause) = &generics_b.where_clause {
        predicates.extend(where_clause.predicates.clone());
    }

    let where_clause = if predicates.is_empty() {
        None
    } else {
        Some(WhereClause {
            where_token: Default::default(),
            predicates,
        })
    };

    Generics {
        lt_token: generics_a.lt_token,
        params,
        gt_token: generics_a.gt_token,
        where_clause,
    }
}
