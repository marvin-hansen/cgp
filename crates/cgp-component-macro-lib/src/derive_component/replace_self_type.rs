/// Utilities for replacing Self type references in token streams.
///
/// This module provides functionality to replace occurrences of the `Self` type
/// with a concrete type identifier while preserving associated type references.
use itertools::Itertools;
use proc_macro2::{Group, Ident, TokenStream, TokenTree};
use quote::{format_ident, ToTokens};
use syn::parse::Parse;

/// Processes a collection of items, replacing Self types in each item.
///
/// # Arguments
///
/// * `vals` - Collection of items to process
/// * `replaced_ident` - The identifier to replace Self with
/// * `local_assoc_types` - List of local associated type names to preserve
///
/// # Returns
///
/// A Result containing the processed collection with Self types replaced
///
/// # Type Parameters
///
/// * `I` - Collection type that can be converted to and from an iterator
/// * `T` - Item type that can be converted to tokens and parsed
pub fn iter_parse_and_replace_self_type<I, T>(
    vals: I,
    replaced_ident: &Ident,
    local_assoc_types: &Vec<Ident>,
) -> syn::Result<I>
where
    I: IntoIterator<Item = T> + FromIterator<T>,
    T: ToTokens + Parse,
{
    vals.into_iter()
        .map(|val| parse_and_replace_self_type(&val, replaced_ident, local_assoc_types))
        .collect()
}

/// Processes a single item, replacing Self types within it.
///
/// # Arguments
///
/// * `val` - The item to process
/// * `replaced_ident` - The identifier to replace Self with
/// * `local_assoc_types` - List of local associated type names to preserve
///
/// # Returns
///
/// A Result containing the processed item with Self types replaced
///
/// # Type Parameters
///
/// * `T` - Type that can be converted to tokens and parsed
pub fn parse_and_replace_self_type<T>(
    val: &T,
    replaced_ident: &Ident,
    local_assoc_types: &Vec<Ident>,
) -> syn::Result<T>
where
    T: ToTokens + Parse,
{
    let stream = replace_self_type(val.to_token_stream(), replaced_ident, local_assoc_types);
    syn::parse2(stream)
}

/// Replaces Self type references in a token stream.
///
/// This function walks through a token stream and replaces occurrences of the
/// `Self` type with a specified identifier, while being careful to preserve
/// associated type expressions (e.g., `Self::AssocType`).
///
/// # Arguments
///
/// * `stream` - The token stream to process
/// * `replaced_ident` - The identifier to replace Self with
/// * `local_assoc_types` - List of local associated type names to preserve
///
/// # Returns
///
/// A new TokenStream with Self types replaced
///
/// # Examples
///
/// ```rust,ignore
/// let stream = quote!(fn process(input: Self) -> Self::Output);
/// let replaced = replace_self_type(stream, &format_ident!("MyType"), &vec![]);
/// // Results in: fn process(input: MyType) -> MyType::Output
/// ```
///
/// # Note
///
/// The function is careful to handle:
/// - Associated type expressions (Self::Type)
/// - Local associated types that should not be replaced
/// - Nested token groups (parentheses, brackets, braces)
pub fn replace_self_type(
    stream: TokenStream,
    replaced_ident: &Ident,
    local_assoc_types: &Vec<Ident>,
) -> TokenStream {
    let self_type = format_ident!("Self");

    let mut result_stream: Vec<TokenTree> = Vec::new();

    let mut token_iter = stream.into_iter().multipeek();

    while let Some(tree) = token_iter.next() {
        match tree {
            TokenTree::Ident(ident) => {
                if ident == self_type {
                    let replaced_ident = replaced_ident.clone();

                    // Do not replace self if it is an associated type expression that refers to local associated type
                    let replaced = match token_iter.peek() {
                        Some(TokenTree::Punct(p)) if p.as_char() == ':' => {
                            match token_iter.peek() {
                                Some(TokenTree::Punct(p)) if p.as_char() == ':' => {
                                    match token_iter.peek() {
                                        Some(TokenTree::Ident(assoc_type))
                                            if local_assoc_types.contains(assoc_type) =>
                                        {
                                            ident
                                        }
                                        _ => replaced_ident,
                                    }
                                }
                                _ => replaced_ident,
                            }
                        }
                        _ => replaced_ident,
                    };

                    result_stream.push(TokenTree::Ident(replaced));
                } else {
                    result_stream.push(TokenTree::Ident(ident));
                }
            }
            TokenTree::Group(group) => {
                let replaced_stream =
                    replace_self_type(group.stream(), replaced_ident, local_assoc_types);
                let replaced_group = Group::new(group.delimiter(), replaced_stream);

                result_stream.push(TokenTree::Group(replaced_group));
            }
            TokenTree::Punct(punct) => {
                result_stream.push(TokenTree::Punct(punct));
            }
            TokenTree::Literal(lit) => result_stream.push(TokenTree::Literal(lit)),
        }
    }

    result_stream.into_iter().collect()
}
