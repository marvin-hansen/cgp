/*!
   Internal implementation library for CGP field macros.

   This is an internal crate used by the `cgp-field-macro` crate. It implements
   the procedural macros for `cgp-field` as a library to enable easier testing
   and maintenance. The implementations are then re-exported as procedural macros
   in the `cgp-field-macro` crate.

   # Modules

   * [`field`] - Field access trait derivation
   * [`product`] - Product and sum type construction
   * [`symbol`] - Symbol type generation

   # Functions

   * [`derive_fields`] - Implements field access traits
   * [`make_product_type`] - Creates product type definitions
   * [`make_sum_type`] - Creates sum type definitions
   * [`make_product_expr`] - Generates product type expressions
   * [`make_symbol`] - Creates symbol type definitions

   # Implementation Notes

   This crate is not meant to be used directly by end users. Instead, use the
   macros provided by the `cgp-field-macro` crate. The separation of implementation
   and macro definition allows for:

   1. Better testing capabilities
   2. Clearer code organization
   3. Easier maintenance and debugging
   4. Reuse of common functionality
*/

pub mod field;
pub mod product;
pub mod symbol;

#[cfg(test)]
mod tests;

pub use field::derive_fields;
pub use product::{make_product_expr, make_product_type, make_sum_type};
pub use symbol::make_symbol;
