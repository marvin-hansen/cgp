/*!
   Type-safe indexing type for field access.

   This module provides a type-level representation of indices that can be
   used for type-safe array and tuple field access in the CGP framework.
*/

/// A type-level index representation.
///
/// This type allows compile-time indices to be used as type parameters,
/// enabling type-safe access to array elements and tuple fields. The index
/// is encoded at the type level through a const generic parameter.
///
/// # Type Parameters
///
/// * `I` - A const generic parameter representing the index value
///
/// # Examples
///
/// ```rust,ignore
/// use cgp_field::Index;
///
/// // Define type-safe indices
/// type First = Index<0>;
/// type Second = Index<1>;
///
/// // Use with tuples or arrays
/// let tuple = (1, "hello");
/// let first: i32 = tuple.get_field(First::new());
/// let second: &str = tuple.get_field(Second::new());
/// ```
pub struct Index<const I: usize>;
