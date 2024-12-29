/// Implementation generation for component consumers.
///
/// This module handles the generation of consumer implementations that allow
/// components to be used through a context type that has component capabilities.
use syn::punctuated::Punctuated;
use syn::token::{Brace, Comma, For, Impl, Plus};
use syn::{
    parse_quote, GenericParam, Ident, ImplItem, ItemImpl, ItemTrait, Path, TraitItem,
    TypeParamBound,
};

use crate::derive_component::delegate_fn::derive_delegated_fn_impl;
use crate::derive_component::delegate_type::derive_delegate_type_impl;

/// Derives an implementation of a consumer trait for a context type.
///
/// This function generates the implementation that allows a context type to use
/// component functionality through its component storage.
///
/// # Arguments
/// * `consumer_trait` - The trait defining the consumer interface
/// * `provider_name` - Name of the provider trait
/// * `context_type` - The context type that will implement the consumer trait
///
/// # Returns
/// * `ItemImpl` - The generated implementation block
///
/// # Generated Code Example
/// ```ignore
/// impl<Context, T> MyComponent<T> for Context
/// where
///     Context: HasComponents,
///     Context::Components: MyComponentProvider<Context, T>
/// {
///     // Delegated function implementations
///     fn my_function(&self) -> Result<(), Error> {
///         self.components().my_function()
///     }
///
///     // Delegated associated type implementations
///     type MyType = <Context::Components as MyComponentProvider<Context, T>>::MyType;
/// }
/// ```
///
/// # Implementation Details
/// The function:
/// 1. Constructs generic parameters for the implementation
/// 2. Adds necessary trait bounds for component access
/// 3. Generates delegating implementations for all trait items
/// 4. Handles both associated functions and associated types
/// 5. Preserves and propagates any supertraits from the consumer trait
pub fn derive_consumer_impl(
    consumer_trait: &ItemTrait,
    provider_name: &Ident,
    context_type: &Ident,
) -> ItemImpl {
    let consumer_name = &consumer_trait.ident;

    let consumer_generic_args = {
        let mut generic_args: Punctuated<Ident, Comma> = Punctuated::new();

        for param in consumer_trait.generics.params.iter() {
            match param {
                GenericParam::Type(ty) => {
                    generic_args.push(ty.ident.clone());
                }
                GenericParam::Const(arg) => {
                    generic_args.push(arg.ident.clone());
                }
                GenericParam::Lifetime(_life) => {
                    unimplemented!()
                }
            }
        }

        generic_args
    };

    let provider_generic_args = {
        let mut generic_args = consumer_generic_args.clone();

        generic_args.insert(0, parse_quote!(#context_type));

        generic_args
    };

    let impl_generics = {
        let mut impl_generics = consumer_trait.generics.clone();

        impl_generics.params.insert(0, parse_quote!(#context_type));

        {
            let supertrait_constraints = consumer_trait.supertraits.clone();

            if !supertrait_constraints.is_empty() {
                if let Some(where_clause) = &mut impl_generics.where_clause {
                    where_clause.predicates.push(parse_quote! {
                        #context_type : #supertrait_constraints
                    });
                } else {
                    impl_generics.where_clause = Some(parse_quote! {
                        where #context_type : #supertrait_constraints
                    });
                }
            }
        }

        {
            let has_component_constraint: Punctuated<TypeParamBound, Plus> = parse_quote! {
                HasComponents
            };

            let provider_constraint: Punctuated<TypeParamBound, Plus> = parse_quote! {
                #provider_name < #provider_generic_args >
            };

            if let Some(where_clause) = &mut impl_generics.where_clause {
                where_clause.predicates.push(parse_quote! {
                    #context_type : #has_component_constraint
                });

                where_clause.predicates.push(parse_quote! {
                    #context_type :: Components : #provider_constraint
                });
            } else {
                impl_generics.where_clause = Some(parse_quote! {
                    where
                        #context_type : #has_component_constraint,
                        #context_type :: Components : #provider_constraint
                });
            }
        }

        impl_generics
    };

    let mut impl_items: Vec<ImplItem> = Vec::new();

    for trait_item in consumer_trait.items.iter() {
        match trait_item {
            TraitItem::Fn(trait_fn) => {
                let impl_fn = derive_delegated_fn_impl(
                    &trait_fn.sig,
                    &parse_quote!(#context_type :: Components),
                );

                impl_items.push(ImplItem::Fn(impl_fn));
            }
            TraitItem::Type(trait_type) => {
                let type_name = &trait_type.ident;
                let type_generics = {
                    let mut type_generics = trait_type.generics.clone();
                    type_generics.where_clause = None;

                    for param in &mut type_generics.params {
                        if let GenericParam::Type(type_param) = param {
                            type_param.bounds.clear();
                        }
                    }

                    type_generics
                };

                let impl_type = derive_delegate_type_impl(
                    trait_type,
                    parse_quote!(
                        < #context_type :: Components as #provider_name < #provider_generic_args > > :: #type_name #type_generics
                    ),
                );

                impl_items.push(ImplItem::Type(impl_type));
            }
            _ => {}
        }
    }

    let trait_path: Path = parse_quote!( #consumer_name < #consumer_generic_args > );

    ItemImpl {
        attrs: consumer_trait.attrs.clone(),
        defaultness: None,
        unsafety: consumer_trait.unsafety,
        impl_token: Impl::default(),
        generics: impl_generics,
        trait_: Some((None, trait_path, For::default())),
        self_ty: Box::new(parse_quote!(#context_type)),
        brace_token: Brace::default(),
        items: impl_items,
    }
}
