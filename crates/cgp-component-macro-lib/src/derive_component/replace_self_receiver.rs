/// Utilities for replacing self receivers in trait function signatures.
///
/// This module provides functionality to replace `self` receivers in trait methods
/// with explicit type parameters, preserving reference and mutability information.
use proc_macro2::Ident;
use syn::{parse_quote, FnArg, TraitItemFn};

use crate::derive_component::snake_case::to_snake_case_ident;

/// Replaces a self receiver in a trait function with an explicit type parameter.
///
/// This function modifies a trait function's signature by replacing any self receiver
/// with an explicit parameter of the specified type, maintaining all reference,
/// lifetime, and mutability qualifiers.
///
/// # Arguments
///
/// * `func` - The trait function to modify
/// * `replaced_type` - The type to use instead of Self
///
/// # Examples
///
/// ```rust,ignore
/// // Before:
/// fn process(&self) -> Result<()>
///
/// // After (with replaced_type = MyType):
/// fn process(my_type: &MyType) -> Result<()>
///
/// // Before:
/// fn consume(self) -> Result<()>
///
/// // After:
/// fn consume(my_type: MyType) -> Result<()>
///
/// // Before:
/// fn modify(&mut self) -> Result<()>
///
/// // After:
/// fn modify(my_type: &mut MyType) -> Result<()>
/// ```
///
/// # Note
///
/// The function handles all combinations of:
/// - Owned self (self)
/// - Shared reference (&self)
/// - Mutable reference (&mut self)
/// - Named lifetimes (&'a self)
/// - Named mutable lifetimes (&'a mut self)
pub fn replace_self_receiver(func: &mut TraitItemFn, replaced_type: &Ident) {
    if let Some(arg) = func.sig.inputs.first_mut() {
        if let FnArg::Receiver(receiver) = arg {
            let replaced_var = to_snake_case_ident(replaced_type);

            match (&receiver.reference, &receiver.mutability) {
                (None, None) => {
                    *arg = parse_quote!(#replaced_var : #replaced_type);
                }
                (Some((_and, None)), None) => {
                    *arg = parse_quote!(#replaced_var : & #replaced_type);
                }
                (Some((_and, Some(life))), None) => {
                    *arg = parse_quote!(#replaced_var : & #life #replaced_type);
                }
                (Some((_and, None)), Some(_mut)) => {
                    *arg = parse_quote!(#replaced_var : &mut #replaced_type);
                }
                (Some((_and, Some(life))), Some(_mut)) => {
                    *arg = parse_quote!(#replaced_var : & #life mut #replaced_type);
                }
                _ => {}
            }
        }
    }
}
