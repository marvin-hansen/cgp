/*!
   Sum type constructors for variant types.

   This module provides type constructors for building sum types (enums)
   in the CGP framework. It includes both binary sum types and the empty
   type.
*/

/// A binary sum type representing one of two possible types.
///
/// This type is similar to Rust's `Result` type but without the success/error
/// semantics. It can hold a value of either the `Head` type or the `Tail`
/// type, making it useful for building larger sum types through composition.
///
/// # Type Parameters
///
/// * `Head` - Type of the left variant
/// * `Tail` - Type of the right variant
///
/// # Examples
///
/// ```rust,ignore
/// use cgp_field::Either;
///
/// // Define a sum type
/// type StringOrInt = Either<String, i32>;
///
/// let s: StringOrInt = Either::Left("hello".to_string());
/// let n: StringOrInt = Either::Right(42);
///
/// match s {
///     Either::Left(s) => println!("String: {}", s),
///     Either::Right(n) => println!("Number: {}", n),
/// }
/// ```
pub enum Either<Head, Tail> {
    /// The left variant containing a value of type `Head`
    Left(Head),
    /// The right variant containing a value of type `Tail`
    Right(Tail),
}

/// The empty type with no values.
///
/// This type is useful in generic programming as it represents a type that
/// cannot be instantiated. It's often used as a base case in recursive type
/// definitions or to indicate impossible states.
///
/// # Examples
///
/// ```rust,ignore
/// use cgp_field::Void;
///
/// // Function that can never return
/// fn never_returns() -> Void {
///     loop {
///         // Can never construct a Void value
///     }
/// }
///
/// // Use in generic types
/// type NoRight<T> = Either<T, Void>;
/// ```
pub enum Void {}
