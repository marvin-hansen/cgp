/*!
   Traits for immutable field access.

   This module provides traits for read-only access to fields in the CGP framework.
   It enables type-safe field access through phantom types and trait implementations.
*/

use core::marker::PhantomData;
use core::ops::Deref;

use cgp_component::UseContext;

/// Trait for types that have accessible fields.
///
/// This trait enables type-safe field access through phantom types. The `Tag`
/// type parameter identifies the field, while the associated `Value` type
/// specifies the field's type.
///
/// # Type Parameters
///
/// * `Tag` - Phantom type identifying the field
///
/// # Associated Types
///
/// * `Value` - Type of the field value
///
/// # Examples
///
/// ```rust,ignore
/// use cgp_field::HasField;
/// use core::marker::PhantomData;
///
/// struct Name; // Field tag
///
/// struct Person {
///     name: String,
/// }
///
/// impl HasField<Name> for Person {
///     type Value = String;
///
///     fn get_field(&self, _: PhantomData<Name>) -> &Self::Value {
///         &self.name
///     }
/// }
/// ```
pub trait HasField<Tag> {
    type Value;

    fn get_field(&self, tag: PhantomData<Tag>) -> &Self::Value;
}

/// Trait for field getters in a context.
///
/// This trait provides a way to access fields from a context type. It is
/// typically used in conjunction with the `UseContext` type to enable
/// field access in component systems.
///
/// # Type Parameters
///
/// * `Context` - The context type containing the field
/// * `Tag` - Phantom type identifying the field
///
/// # Associated Types
///
/// * `Value` - Type of the field value
///
/// # Examples
///
/// ```rust,ignore
/// use cgp_field::FieldGetter;
/// use core::marker::PhantomData;
///
/// struct Name; // Field tag
///
/// impl FieldGetter<MyContext, Name> for MyFieldGetter {
///     type Value = String;
///
///     fn get_field(context: &MyContext, _: PhantomData<Name>) -> &Self::Value {
///         context.get_name()
///     }
/// }
/// ```
pub trait FieldGetter<Context, Tag> {
    type Value;

    fn get_field(context: &Context, tag: PhantomData<Tag>) -> &Self::Value;
}

/// Implementation of `HasField` for types that can be dereferenced.
///
/// This implementation allows field access through dereferencing, enabling
/// field access on wrapper types that implement `Deref`.
impl<Context, Tag, Target, Value> HasField<Tag> for Context
where
    Context: Deref<Target = Target>,
    Target: HasField<Tag, Value = Value> + 'static,
{
    type Value = Value;

    fn get_field(&self, tag: PhantomData<Tag>) -> &Self::Value {
        self.deref().get_field(tag)
    }
}

/// Implementation of `FieldGetter` for `UseContext`.
///
/// This implementation enables field access through the `UseContext` type,
/// which is commonly used in component systems.
impl<Context, Tag, Field> FieldGetter<Context, Tag> for UseContext
where
    Context: HasField<Tag, Value = Field>,
{
    type Value = Field;

    fn get_field(context: &Context, _tag: PhantomData<Tag>) -> &Self::Value {
        context.get_field(PhantomData)
    }
}
