/// Utilities for converting identifiers to snake_case format.
use proc_macro2::Span;
use syn::Ident;

/// Converts a string to snake_case format.
///
/// This function converts a camelCase or PascalCase string to snake_case by:
/// 1. Inserting underscores before uppercase letters (except at the start)
/// 2. Converting all characters to lowercase
///
/// # Arguments
///
/// * `val` - The string to convert
///
/// # Returns
///
/// A new String in snake_case format
///
/// # Examples
///
/// ```
/// # use cgp_component_macro_lib::derive_component::snake_case::to_snake_case_str;
/// assert_eq!(to_snake_case_str("MyComponent"), "my_component");
/// assert_eq!(to_snake_case_str("processData"), "process_data");
/// ```
pub fn to_snake_case_str(val: &str) -> String {
    let mut acc = String::new();
    let mut prev = '_';

    for ch in val.chars() {
        if ch.is_uppercase() && prev != '_' {
            acc.push('_');
        }
        acc.push(ch);
        prev = ch;
    }

    acc.to_lowercase()
}

/// Converts an identifier to its snake_case equivalent.
///
/// Creates a new identifier with the same content as the input but converted
/// to snake_case format. The new identifier is created with the call site span.
///
/// # Arguments
///
/// * `val` - The identifier to convert
///
/// # Returns
///
/// A new `syn::Ident` in snake_case format
///
/// # Examples
///
/// ```
/// # use syn::{parse_quote, Ident};
/// # use cgp_component_macro_lib::derive_component::snake_case::to_snake_case_ident;
/// let ident: Ident = parse_quote!(MyComponent);
/// let snake = to_snake_case_ident(&ident);
/// assert_eq!(snake.to_string(), "my_component");
/// ```
pub fn to_snake_case_ident(val: &Ident) -> Ident {
    Ident::new(&to_snake_case_str(&val.to_string()), Span::call_site())
}
