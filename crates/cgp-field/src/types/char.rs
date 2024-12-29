/*!
   Character type for type-level field names.

   This module provides a type-level representation of characters that can be
   used as field identifiers in the CGP framework.
*/

/// A type-level character representation.
///
/// This type allows characters to be used as type parameters, enabling
/// field names to be encoded at the type level. This is particularly
/// useful for creating type-safe field identifiers.
///
/// # Type Parameters
///
/// * `CHAR` - A const generic parameter representing a character value
///
/// # Examples
///
/// ```rust,ignore
/// use cgp_field::Char;
///
/// // Define field tags using characters
/// type NameField = Char<'n'>;
/// type AgeField = Char<'a'>;
/// ```
pub struct Char<const CHAR: char>;
