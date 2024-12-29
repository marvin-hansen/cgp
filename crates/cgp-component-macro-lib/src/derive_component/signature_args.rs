/// Utilities for extracting and processing function signature arguments.
use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_quote, FnArg, Ident, Signature};

/// Converts a function signature's arguments into a sequence of identifiers.
///
/// This function processes a function's signature and extracts the names of all
/// its arguments, handling both self receivers and typed arguments.
///
/// # Arguments
///
/// * `sig` - The function signature to process
///
/// # Returns
///
/// A comma-separated sequence of identifiers representing the argument names.
/// For self receivers, returns the identifier "self".
/// For typed arguments, returns the pattern identifier.
///
/// # Examples
///
/// ```rust,ignore
/// let sig: Signature = parse_quote!(fn process(&self, data: String) -> Result<()>);
/// let args = signature_to_args(&sig);
/// // Results in: self, data
///
/// let sig: Signature = parse_quote!(fn transform(input: i32, scale: f64) -> i32);
/// let args = signature_to_args(&sig);
/// // Results in: input, scale
/// ```
///
/// # Note
///
/// This function is particularly useful when generating delegating implementations
/// where the argument names need to be forwarded to another function.
pub fn signature_to_args(sig: &Signature) -> Punctuated<Ident, Comma> {
    let args = sig
        .inputs
        .iter()
        .map(|arg| -> Ident {
            match arg {
                FnArg::Receiver(_) => Ident::new("self", Span::call_site()),
                FnArg::Typed(pat) => {
                    let ident_pat = &pat.pat;
                    parse_quote!( #ident_pat )
                }
            }
        })
        .collect();

    args
}
