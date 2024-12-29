/// Utilities for deriving associated type implementations for delegated components.
///
/// This module provides functionality to create implementations of associated types
/// when delegating component traits to other types.
use syn::token::Eq;
use syn::{ImplItemType, TraitItemType, Type, Visibility};

/// Derives an implementation of an associated type for a delegated component.
///
/// Creates an implementation that sets a trait's associated type to the specified
/// delegated type, preserving attributes and generic parameters from the original
/// trait definition.
///
/// # Arguments
///
/// * `trait_type` - The associated type definition from the trait
/// * `delegated_type` - The concrete type to use in the implementation
///
/// # Returns
///
/// An `ImplItemType` representing the associated type implementation
///
/// # Examples
///
/// ```rust,ignore
/// // For a trait definition:
/// trait MyComponent {
///     type Output;
/// }
///
/// // And a delegated type:
/// type DelegatedOutput = String;
///
/// // This function generates:
/// type Output = DelegatedOutput;
/// ```
///
/// # Note
///
/// The generated implementation:
/// - Inherits visibility from the trait
/// - Preserves all attributes from the trait definition
/// - Maintains generic parameters and where clauses
pub fn derive_delegate_type_impl(trait_type: &TraitItemType, delegated_type: Type) -> ImplItemType {
    ImplItemType {
        attrs: trait_type.attrs.clone(),
        vis: Visibility::Inherited,
        defaultness: None,
        type_token: trait_type.type_token,
        ident: trait_type.ident.clone(),
        generics: trait_type.generics.clone(),
        eq_token: Eq::default(),
        ty: delegated_type,
        semi_token: trait_type.semi_token,
    }
}
