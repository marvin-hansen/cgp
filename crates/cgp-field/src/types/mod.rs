/*!
   Type definitions for field manipulation.

   This module provides various type definitions used in the CGP field system:

   * [`mod@char`] - Character type definitions for field names
   * [`field`] - Field accessor types
   * [`index`] - Type-safe indexing types
   * [`product`] - Product type (struct) definitions
   * [`sum`] - Sum type (enum) definitions

   These types work together to provide a type-safe system for field access
   and manipulation in the CGP framework.
*/

pub mod char;
pub mod field;
pub mod index;
pub mod product;
pub mod sum;

pub use char::*;
pub use field::*;
pub use index::*;
pub use product::*;
pub use sum::*;
