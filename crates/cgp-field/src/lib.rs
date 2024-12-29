#![no_std]

/*!
   Core traits and types for field manipulation in the CGP framework.

   This crate provides the fundamental building blocks for working with fields
   in algebraic data types. It enables type-safe field access, product and sum
   types, and field manipulation through a set of traits and types.

   # Core Traits

   * [`FieldGetter`] - Trait for read-only field access
   * [`HasField`] - Trait for types with accessible fields
   * [`HasFieldMut`] - Trait for mutable field access
   * [`MutFieldGetter`] - Trait for mutable field getters

   # Types

   * [`Field`] - Type representing a field accessor
   * [`Char`] - Character type for field names
   * [`Cons`] - Type constructor for heterogeneous lists
   * [`Either`] - Sum type for two alternatives
   * [`Index`] - Type-safe indexing
   * [`Nil`] - Empty type for list termination
   * [`Void`] - Type with no values

   # Macros

   The crate re-exports several macros from [`cgp_field_macro`]:
   * `HasField` - Derive macro for field access traits
   * `Product` - Macro for defining product types
   * `Sum` - Macro for defining sum types
   * `product` - Macro for product type expressions
   * `symbol` - Macro for type-safe symbol creation

   # Examples

   ```rust,ignore
   use cgp_field::{HasField, Product, Field};

   // Define a product type
   Product! {
       struct Point {
           x: f64,
           y: f64,
       }
   }

   // Use field access
   let point = Point { x: 1.0, y: 2.0 };
   let x: f64 = point.get_field(Field::new());
   ```
*/

pub mod impls;
pub mod traits;
pub mod types;

pub use cgp_field_macro::{product, symbol, HasField, Product, Sum};
pub use traits::{FieldGetter, HasField, HasFieldMut, MutFieldGetter};
pub use types::{Char, Cons, Either, Field, Index, Nil, Void};
