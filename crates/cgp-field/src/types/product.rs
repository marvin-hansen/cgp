/*!
   Product type constructors for heterogeneous lists.

   This module provides type constructors for building heterogeneous lists,
   which are commonly used to represent product types (structs) in the CGP
   framework.
*/

/// A cons cell for building heterogeneous lists.
///
/// This type is a building block for constructing heterogeneous lists,
/// similar to cons cells in functional programming. It combines a head
/// element with a tail that can be either another `Cons` or `Nil`.
///
/// # Type Parameters
///
/// * `Head` - Type of the head element
/// * `Tail` - Type of the tail (another `Cons` or `Nil`)
///
/// # Examples
///
/// ```rust,ignore
/// use cgp_field::{Cons, Nil};
///
/// // Build a heterogeneous list
/// type MyList = Cons<i32, Cons<String, Cons<bool, Nil>>>;
///
/// let list = Cons(1, Cons("hello".to_string(), Cons(true, Nil)));
/// ```
pub struct Cons<Head, Tail>(pub Head, pub Tail);

/// Empty type representing the end of a heterogeneous list.
///
/// This type serves as the terminator for heterogeneous lists built with
/// `Cons`. It represents an empty list and is used as the base case in
/// recursive type definitions.
///
/// # Examples
///
/// ```rust,ignore
/// use cgp_field::{Cons, Nil};
///
/// // Use Nil as list terminator
/// type EmptyList = Nil;
/// type SingletonList = Cons<i32, Nil>;
///
/// let empty = Nil;
/// let singleton = Cons(42, Nil);
/// ```
pub struct Nil;
