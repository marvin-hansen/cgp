/// Provider trait generation and transformation.
///
/// This module handles the generation of provider traits from consumer traits,
/// including the transformation of self types and receivers into context types.
use syn::punctuated::Punctuated;
use syn::{parse_quote, Ident, ItemTrait, TraitItem};

use crate::derive_component::replace_self_receiver::replace_self_receiver;
use crate::derive_component::replace_self_type::{
    iter_parse_and_replace_self_type, parse_and_replace_self_type,
};

/// Derives a provider trait from a consumer trait definition.
///
/// This function transforms a consumer trait into a provider trait by:
/// 1. Adding a context type parameter
/// 2. Converting self-type references to context type references
/// 3. Moving supertrait bounds to where clauses on the context type
///
/// # Arguments
/// * `consumer_trait` - The original consumer trait to transform
/// * `provider_name` - Name for the generated provider trait
/// * `context_type` - Name of the context type parameter
///
/// # Returns
/// * `syn::Result<ItemTrait>` - The generated provider trait
///
/// # Example Transformation
/// From consumer trait:
/// ```ignore
/// trait MyComponent: SuperTrait {
///     fn my_method(&self) -> Self::Output;
///     type Output;
/// }
/// ```
///
/// To provider trait:
/// ```ignore
/// trait MyComponentProvider<Context>
/// where
///     Context: SuperTrait
/// {
///     fn my_method(&self, context: &Context) -> Self::Output;
///     type Output;
/// }
/// ```
///
/// # Implementation Details
/// The function performs these transformations:
/// 1. Adds the context type parameter
/// 2. Moves supertrait bounds to where clauses
/// 3. Replaces self type references with context type
/// 4. Transforms method signatures to accept context parameters
/// 5. Preserves associated types while updating their bounds
pub fn derive_provider_trait(
    consumer_trait: &ItemTrait,
    provider_name: &Ident,
    context_type: &Ident,
) -> syn::Result<ItemTrait> {
    let mut provider_trait = consumer_trait.clone();

    provider_trait.ident = provider_name.clone();

    // Add generic parameter `Context` to the front of generics
    {
        provider_trait
            .generics
            .params
            .insert(0, parse_quote!(#context_type));
    }

    let local_assoc_types: Vec<Ident> = provider_trait
        .items
        .iter()
        .filter_map(|item| {
            if let TraitItem::Type(assoc_type) = item {
                Some(assoc_type.ident.clone())
            } else {
                None
            }
        })
        .collect();

    // Turn the supertrait constraints into `Context` constraints in the `where` clause
    {
        let context_constraints = iter_parse_and_replace_self_type(
            provider_trait.supertraits.clone(),
            context_type,
            &local_assoc_types,
        )?;

        provider_trait.supertraits = Punctuated::default();

        if !context_constraints.is_empty() {
            if let Some(where_clause) = &mut provider_trait.generics.where_clause {
                let mut predicates = iter_parse_and_replace_self_type(
                    where_clause.predicates.clone(),
                    context_type,
                    &local_assoc_types,
                )?;

                predicates.push(parse_quote! {
                    #context_type : #context_constraints
                });

                where_clause.predicates = predicates;
            } else {
                provider_trait.generics.where_clause = Some(parse_quote! {
                    where #context_type : #context_constraints
                });
            }
        }
    }

    // Replace self type and argument into context type argument
    {
        for item in provider_trait.items.iter_mut() {
            let mut replaced_item =
                parse_and_replace_self_type(item, context_type, &local_assoc_types)?;

            if let TraitItem::Fn(func) = &mut replaced_item {
                replace_self_receiver(func, context_type);
            }

            *item = replaced_item;
        }
    }

    Ok(provider_trait)
}
