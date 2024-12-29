/// Imports required for token stream processing and AST manipulation
use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::delegate_components::ast::DelegateComponentsAst;
use crate::delegate_components::impl_delegate::impl_delegate_components;

/// Processes component delegation macro by generating the necessary trait implementations.
///
/// # Arguments
/// * `body` - The input token stream containing the delegation specification
///
/// # Returns
/// * `syn::Result<TokenStream>` - The generated implementation code as a token stream
///
/// # Example
/// This function handles macro invocations like:
/// ```ignore
/// #[delegate_components]
/// struct MyStruct {
///     delegate: DelegateType,
/// }
/// ```
pub fn delegate_components(body: TokenStream) -> syn::Result<TokenStream> {
    let ast: DelegateComponentsAst = syn::parse2(body)?;

    let impl_items = impl_delegate_components(
        &ast.target_type,
        &ast.target_generics,
        &ast.delegate_entries,
    );

    let mut output = TokenStream::new();

    for impl_item in impl_items {
        output.extend(impl_item.to_token_stream());
    }

    Ok(output)
}
