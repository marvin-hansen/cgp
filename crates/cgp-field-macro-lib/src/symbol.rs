/*!
   Symbol type generation implementation.

   This module provides functionality for generating type-level symbols
   from strings. These symbols are used as type-safe field identifiers
   in the CGP framework.
*/

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, LitStr, Type};

/// Convert a string into a type-level symbol.
///
/// This function converts a string into a type-level representation using
/// `Cons` and `Char` types. Each character in the string becomes a `Char`
/// type parameter in a nested `Cons` list.
///
/// # Arguments
///
/// * `value` - The string to convert into a type-level symbol
///
/// # Returns
///
/// A `Type` representing the nested `Cons` structure
///
/// # Examples
///
/// ```rust,ignore
/// let symbol = symbol_from_string("abc");
/// // Generates: Cons<Char<'a'>, Cons<Char<'b'>, Cons<Char<'c'>, Nil>>>
/// ```
pub fn symbol_from_string(value: &str) -> Type {
    value
        .chars()
        .rfold(parse_quote! { Nil }, |tail, c: char| -> Type {
            parse_quote!( Cons< Char< #c >, #tail > )
        })
}

/// Generate a symbol type from a string literal in macro input.
///
/// This function parses a string literal from the macro input and converts
/// it into a type-level symbol representation. It's the main entry point
/// for the symbol macro.
///
/// # Arguments
///
/// * `input` - Token stream containing a string literal
///
/// # Returns
///
/// Token stream representing the generated symbol type
///
/// # Examples
///
/// Input: `"name"`
/// Output: `Cons<Char<'n'>, Cons<Char<'a'>, Cons<Char<'m'>, Cons<Char<'e'>, Nil>>>>`
pub fn make_symbol(input: TokenStream) -> TokenStream {
    let literal: LitStr = syn::parse2(input).unwrap();

    let symbol = symbol_from_string(&literal.value());

    symbol.to_token_stream()
}
