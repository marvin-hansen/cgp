/// Macro generation utilities for type substitution in presets.
///
/// This module provides functionality for creating macros that perform
/// type substitution when working with presets.
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

/// Defines a macro for component type substitution.
///
/// Creates a macro that can substitute component types in a preset with
/// their concrete implementations. This is useful for applying preset
/// configurations to specific types.
///
/// # Arguments
///
/// * `macro_name` - Name of the macro to generate
/// * `substitution` - Token stream containing the types to substitute
///
/// # Returns
///
/// Returns a token stream containing the macro definition.
///
/// # Examples
///
/// For a preset with components `[ComponentA, ComponentB]`, this generates:
/// ```text
/// macro_rules! with_my_preset {
///     ($($body:tt)*) => {
///         replace_with! {
///             [ComponentA, ComponentB],
///             $($body)*
///         }
///     };
/// }
/// ```
pub fn define_substitution_macro(macro_name: &Ident, substitution: &TokenStream) -> TokenStream {
    quote! {
        #[macro_export]
        macro_rules! #macro_name {
            ( $( $body:tt )* ) => {
                replace_with! {
                    [ #substitution ],
                    $( $body )*
                }
            };
        }

        pub use #macro_name;
    }
}
