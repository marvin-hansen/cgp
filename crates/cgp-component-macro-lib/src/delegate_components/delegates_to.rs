/// Functionality for defining delegate component trait bounds and trait implementations
use syn::punctuated::Punctuated;
use syn::token::Plus;
use syn::{parse_quote, Generics, Ident, ItemImpl, ItemTrait, Type, TypeParamBound};

use crate::delegate_components::ast::DelegateEntriesAst;

/// Generates trait bounds for delegated components.
///
/// This function creates a set of trait bounds that ensure each component
/// can be delegated to the target type. It processes all components in the
/// delegate entries and creates appropriate DelegateComponent bounds.
///
/// # Arguments
/// * `target_type` - The type that components will delegate to
/// * `delegate_entries` - AST containing all component delegation specifications
///
/// # Returns
/// * `Punctuated<TypeParamBound, Plus>` - A sequence of trait bounds separated by '+'
///
/// # Example
/// For a component of type `MyComponent` delegating to `TargetType`, generates:
/// ```ignore
/// DelegateComponent<MyComponent, Delegate = TargetType>
/// ```
pub fn define_delegate_component_trait_bounds(
    target_type: &Type,
    delegate_entries: &DelegateEntriesAst,
) -> Punctuated<TypeParamBound, Plus> {
    let mut trait_bounds: Punctuated<TypeParamBound, Plus> = Punctuated::new();

    for component in delegate_entries.all_components() {
        let component_type = &component.component_type;
        let trait_bound: TypeParamBound = parse_quote!(
            DelegateComponent<#component_type, Delegate = #target_type>
        );
        trait_bounds.push(trait_bound);
    }

    trait_bounds
}

/// Defines a trait and its implementation for component delegation.
///
/// This function creates:
/// 1. A trait with bounds ensuring all components can delegate to the target type
/// 2. A generic implementation of this trait for any type meeting these bounds
///
/// # Arguments
/// * `trait_name` - Name of the trait to define
/// * `target_type` - The type that components will delegate to
/// * `target_generics` - Generic parameters for the target type
/// * `delegate_entries` - AST containing all component delegation specifications
///
/// # Returns
/// * `(ItemTrait, ItemImpl)` - Tuple containing the trait definition and its implementation
///
/// # Example
/// For trait name `DelegatesTo`, generates something like:
/// ```ignore
/// pub trait DelegatesTo<T>: DelegateComponent<Component1> + DelegateComponent<Component2> {}
/// impl<T, Components> DelegatesTo<T> for Components
/// where
///     Components: DelegateComponent<Component1> + DelegateComponent<Component2>
/// {}
/// ```
pub fn define_delegates_to_trait(
    trait_name: &Ident,
    target_type: &Type,
    target_generics: &Generics,
    delegate_entries: &DelegateEntriesAst,
) -> (ItemTrait, ItemImpl) {
    let trait_bounds = define_delegate_component_trait_bounds(target_type, delegate_entries);

    let item_trait = parse_quote! {
        pub trait #trait_name #target_generics: #trait_bounds {}
    };

    let mut impl_generics = target_generics.clone();
    impl_generics.params.push(parse_quote!(Components));

    let type_generics = target_generics.split_for_impl().1;

    let item_impl = parse_quote! {
        impl #impl_generics #trait_name #type_generics  for Components
        where
            Components: #trait_bounds
        {}
    };

    (item_trait, item_impl)
}
