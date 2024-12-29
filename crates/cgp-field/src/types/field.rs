/*!
   Field type for type-safe field access.

   This module provides the [`Field`] type, which represents a field in a
   data structure with type-safe access through phantom types.
*/

use core::marker::PhantomData;

/// A type-safe field representation.
///
/// This type represents a field in a data structure, using phantom types
/// to ensure type safety at compile time. It combines a value with a
/// phantom type tag that identifies the field.
///
/// # Type Parameters
///
/// * `Tag` - Phantom type identifying the field
/// * `Value` - Type of the field value
///
/// # Examples
///
/// ```rust,ignore
/// use cgp_field::Field;
///
/// struct Name; // Field tag
///
/// let name_field: Field<Name, String> = Field::from("Alice".to_string());
/// assert_eq!(name_field.value, "Alice");
/// ```
pub struct Field<Tag, Value> {
    /// The actual value stored in the field
    pub value: Value,
    /// Phantom data for type-safe field identification
    pub phantom: PhantomData<Tag>,
}

/// Implementation of `From` for converting values into fields.
///
/// This implementation allows easy creation of field values from their
/// underlying value type.
impl<Tag, Value> From<Value> for Field<Tag, Value> {
    fn from(value: Value) -> Self {
        Self {
            value,
            phantom: PhantomData,
        }
    }
}
