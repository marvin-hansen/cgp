use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, ImplItemFn, Signature, TypePath, Visibility};

use crate::derive_component::signature_args::signature_to_args;

/// Generates a function implementation that delegates calls to another type.
///
/// Creates an implementation of a function that forwards all its calls to a corresponding
/// function on the delegator type. Handles both synchronous and asynchronous functions,
/// automatically adding `.await` when needed.
///
/// # Arguments
///
/// * `sig` - The function signature to implement, containing name, arguments, and return type
/// * `delegator` - The type path to delegate the function calls to
///
/// # Returns
///
/// A `syn::ImplItemFn` representing the delegating function implementation with:
/// - The same signature as the input
/// - A body that forwards the call to the delegator
/// - Inherited visibility
/// - No attributes
///
/// # Examples
///
/// ```rust,ignore
/// // For a sync function
/// let sig = parse_quote!(fn process(&self, data: String) -> Result<(), Error>);
/// let delegator = parse_quote!(Inner);
/// derive_delegated_fn_impl(&sig, &delegator)
/// // Generates: fn process(&self, data: String) -> Result<(), Error> {
/// //     Inner::process(data)
/// // }
///
/// // For an async function
/// let sig = parse_quote!(async fn process(&self, data: String) -> Result<(), Error>);
/// derive_delegated_fn_impl(&sig, &delegator)
/// // Generates: async fn process(&self, data: String) -> Result<(), Error> {
/// //     Inner::process(data).await
/// // }
/// ```
///
/// # Notes
///
/// The generated implementation automatically:
/// - Preserves all function arguments and their order
/// - Handles async/await syntax when the source function is async
/// - Maintains the original function's signature including generics and where clauses
pub fn derive_delegated_fn_impl(sig: &Signature, delegator: &TypePath) -> ImplItemFn {
    let fn_name = &sig.ident;

    let args = signature_to_args(sig);

    let await_expr: TokenStream = if sig.asyncness.is_some() {
        quote!( .await )
    } else {
        TokenStream::new()
    };

    let body = parse_quote!({
        #delegator :: #fn_name (
            #args
        ) #await_expr
    });

    ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        defaultness: None,
        sig: sig.clone(),
        block: body,
    }
}
