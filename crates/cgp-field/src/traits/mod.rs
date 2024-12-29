/*!
   Traits for field access and manipulation.

   This module provides the core traits for working with fields in the CGP framework:
   
   * [`HasField`] and [`FieldGetter`] for immutable field access
   * [`HasFieldMut`] and [`MutFieldGetter`] for mutable field access
*/

pub mod has_field;
pub mod has_field_mut;

pub use has_field::{FieldGetter, HasField};
pub use has_field_mut::{HasFieldMut, MutFieldGetter};
