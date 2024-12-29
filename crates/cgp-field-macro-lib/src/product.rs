/*!
   Product and sum type construction implementation.

   This module provides the implementation for constructing product and sum
   types in the CGP framework. It handles the parsing and generation of
   heterogeneous lists and variant types.
*/

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Expr, Type};

/// Parser for comma-separated lists.
///
/// This type wraps a `Punctuated` sequence to provide parsing functionality
/// for comma-separated lists of items in macro input.
///
/// # Type Parameters
///
/// * `T` - The type of items in the list
pub struct ParsePunctuated<T>(pub Punctuated<T, Comma>);

impl<T: Parse> Parse for ParsePunctuated<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let types = Punctuated::parse_terminated(input)?;
        Ok(ParsePunctuated(types))
    }
}

/// Generate a product type from a list of types.
///
/// This function creates a nested `Cons` type structure from a list of
/// types, terminating with `Nil`. It's used to construct heterogeneous
/// lists at the type level.
///
/// # Arguments
///
/// * `input` - Token stream containing comma-separated types
///
/// # Returns
///
/// Token stream representing the nested `Cons` type
///
/// # Examples
///
/// Input: `i32, String, bool`
/// Output: `Cons<i32, Cons<String, Cons<bool, Nil>>>`
pub fn make_product_type(input: TokenStream) -> TokenStream {
    let types: ParsePunctuated<Type> = syn::parse2(input).unwrap();

    types.0.iter().rfold(quote! { Nil }, |res, item| {
        quote! {
            Cons< #item , #res >
        }
    })
}

/// Generate a sum type from a list of types.
///
/// This function creates a nested `Either` type structure from a list of
/// types, terminating with `Void`. It's used to construct variant types
/// at the type level.
///
/// # Arguments
///
/// * `input` - Token stream containing comma-separated types
///
/// # Returns
///
/// Token stream representing the nested `Either` type
///
/// # Examples
///
/// Input: `i32, String, bool`
/// Output: `Either<i32, Either<String, Either<bool, Void>>>`
pub fn make_sum_type(input: TokenStream) -> TokenStream {
    let types: ParsePunctuated<Type> = syn::parse2(input).unwrap();

    types.0.iter().rfold(quote! { Void }, |res, item| {
        quote! {
            Either< #item , #res >
        }
    })
}

/// Generate a product expression from a list of expressions.
///
/// This function creates a nested `Cons` value structure from a list of
/// expressions, terminating with `Nil`. It's used to construct heterogeneous
/// lists at the value level.
///
/// # Arguments
///
/// * `input` - Token stream containing comma-separated expressions
///
/// # Returns
///
/// Token stream representing the nested `Cons` expression
///
/// # Examples
///
/// Input: `1, "hello", true`
/// Output: `Cons(1, Cons("hello", Cons(true, Nil)))`
pub fn make_product_expr(input: TokenStream) -> TokenStream {
    let types: ParsePunctuated<Expr> = syn::parse2(input).unwrap();

    types.0.iter().rfold(quote! { Nil }, |res, item| {
        quote! {
            Cons( #item , #res )
        }
    })
}
