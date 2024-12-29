use proc_macro2::{Group, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::__private::parse_brackets;
use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Or};
use syn::{braced, Ident, Type};

use crate::delegate_components::ast::ComponentAst;

/// Specification for token replacement in a macro.
///
/// This structure holds the information needed to perform token replacements
/// in a macro expansion, including the target identifier to replace,
/// the replacement tokens, and the body where replacements should occur.
#[derive(Debug)]
pub struct ReplaceSpecs {
    /// The identifier to be replaced in the body
    pub target_ident: Ident,
    /// List of token streams that will replace the target identifier
    pub replacements: Vec<TokenStream>,
    /// The body of code where replacements will occur
    pub body: TokenStream,
}

/// Parser implementation for ReplaceSpecs
///
/// Parses the syntax:
/// ```ignore
/// [Type1, Type2, ...], [ExcludeType1, ExcludeType2, ...] | target_ident | { body }
/// ```
impl Parse for ReplaceSpecs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let raw_replacements: Vec<ComponentAst> = {
            let content = parse_brackets(input)?.content;
            let types = <Punctuated<ComponentAst, Comma>>::parse_terminated(&content)?;
            types.into_iter().collect()
        };

        Comma::parse(input)?;

        let exclude: Vec<Type> = {
            let fork = input.fork();

            if let Ok(bracket) = parse_brackets(&fork) {
                let types = <Punctuated<Type, Comma>>::parse_terminated(&bracket.content)?;

                input.advance_to(&fork);
                Comma::parse(input)?;

                types.into_iter().collect()
            } else {
                Vec::new()
            }
        };

        Or::parse(input)?;

        let target_ident = Ident::parse(input)?;

        Or::parse(input)?;

        let body = {
            let content;
            braced!(content in input);
            TokenStream::parse(&content)?
        };

        let replacements = raw_replacements
            .into_iter()
            .filter(|replacement| {
                !exclude
                    .iter()
                    .any(|exclude| exclude == &replacement.component_type)
            })
            .map(|ast| ast.to_token_stream())
            .collect();

        Ok(ReplaceSpecs {
            target_ident,
            replacements,
            body,
        })
    }
}

/// Handles the for_each_replace macro expansion
///
/// # Arguments
/// * `tokens` - Input token stream containing replacement specifications
///
/// # Returns
/// * `syn::Result<TokenStream>` - Expanded code with all replacements applied
pub fn handle_for_each_replace(tokens: TokenStream) -> syn::Result<TokenStream> {
    let specs: ReplaceSpecs = syn::parse2(tokens)?;

    Ok(for_each_replace(
        &specs.target_ident,
        &specs.replacements,
        &specs.body,
    ))
}

/// Handles the replace macro expansion for a single replacement
///
/// # Arguments
/// * `tokens` - Input token stream containing replacement specification
///
/// # Returns
/// * `syn::Result<TokenStream>` - Expanded code with replacement applied
pub fn handle_replace(tokens: TokenStream) -> syn::Result<TokenStream> {
    let specs: ReplaceSpecs = syn::parse2(tokens)?;

    let items: Punctuated<TokenStream, Comma> = specs.replacements.into_iter().collect();

    let tokens = quote! { [ #items ] };

    Ok(replace_stream(&specs.target_ident, &tokens, specs.body))
}

/// Performs multiple replacements in a body of code
///
/// # Arguments
/// * `target_ident` - The identifier to replace
/// * `replacements` - List of token streams to use as replacements
/// * `body` - The code where replacements should occur
///
/// # Returns
/// * `TokenStream` - The code with all replacements applied
pub fn for_each_replace(
    target_ident: &Ident,
    replacements: &[TokenStream],
    body: &TokenStream,
) -> TokenStream {
    replacements
        .iter()
        .map(|replacement| replace_stream(target_ident, replacement, body.clone()))
        .collect()
}

/// Replaces a target identifier with a replacement token stream in a body of code
///
/// # Arguments
/// * `target_ident` - The identifier to replace
/// * `replacement` - The token stream to use as replacement
/// * `body` - The code where replacement should occur
///
/// # Returns
/// * `TokenStream` - The code with replacement applied
pub fn replace_stream(
    target_ident: &Ident,
    replacement: &TokenStream,
    body: TokenStream,
) -> TokenStream {
    body.into_iter()
        .map(|tree| replace_tree(target_ident, replacement, tree))
        .collect()
}

/// Replaces a target identifier with a replacement token stream in a single token tree
///
/// # Arguments
/// * `target_ident` - The identifier to replace
/// * `replacement` - The token stream to use as replacement
/// * `body` - The token tree where replacement should occur
///
/// # Returns
/// * `TokenStream` - The token tree with replacement applied
pub fn replace_tree(
    target_ident: &Ident,
    replacement: &TokenStream,
    body: TokenTree,
) -> TokenStream {
    match body {
        TokenTree::Group(group) => TokenTree::Group(Group::new(
            group.delimiter(),
            replace_stream(target_ident, replacement, group.stream()),
        ))
        .into(),
        TokenTree::Ident(ident) => {
            if &ident == target_ident {
                replacement.to_token_stream()
            } else {
                TokenTree::Ident(ident).into()
            }
        }
        tokens => tokens.into(),
    }
}
