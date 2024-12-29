/*!
   Traits for mutable field access.

   This module provides traits for mutable access to fields in the CGP framework.
   It extends the immutable field access traits with mutable counterparts.
*/

use core::marker::PhantomData;
use core::ops::DerefMut;

use crate::traits::has_field::HasField;
use crate::FieldGetter;

/// Trait for types that have mutably accessible fields.
///
/// This trait extends [`HasField`] to provide mutable access to fields.
/// Types implementing this trait allow both reading from and writing to
/// their fields.
///
/// # Type Parameters
///
/// * `Tag` - Phantom type identifying the field
///
/// # Examples
///
/// ```rust,ignore
/// use cgp_field::{HasField, HasFieldMut};
/// use core::marker::PhantomData;
///
/// struct Name; // Field tag
///
/// struct Person {
///     name: String,
/// }
///
/// impl HasFieldMut<Name> for Person {
///     fn get_field_mut(&mut self, _: PhantomData<Name>) -> &mut Self::Value {
///         &mut self.name
///     }
/// }
/// ```
pub trait HasFieldMut<Tag>: HasField<Tag> {
    fn get_field_mut(&mut self, tag: PhantomData<Tag>) -> &mut Self::Value;
}

/// Trait for mutable field getters in a context.
///
/// This trait extends [`FieldGetter`] to provide mutable access to fields
/// through a context type. It is typically used in conjunction with component
/// systems that require mutable field access.
///
/// # Type Parameters
///
/// * `Context` - The context type containing the field
/// * `Tag` - Phantom type identifying the field
///
/// # Examples
///
/// ```rust,ignore
/// use cgp_field::{FieldGetter, MutFieldGetter};
/// use core::marker::PhantomData;
///
/// struct Name; // Field tag
///
/// impl MutFieldGetter<MyContext, Name> for MyFieldGetter {
///     fn get_field_mut(context: &mut MyContext, _: PhantomData<Name>) -> &mut Self::Value {
///         context.get_name_mut()
///     }
/// }
/// ```
pub trait MutFieldGetter<Context, Tag>: FieldGetter<Context, Tag> {
    fn get_field_mut(context: &mut Context, tag: PhantomData<Tag>) -> &mut Self::Value;
}

/// Implementation of `HasFieldMut` for types that can be mutably dereferenced.
///
/// This implementation allows mutable field access through dereferencing,
/// enabling field modification on wrapper types that implement `DerefMut`.
impl<Context, Tag, Target, Value> HasFieldMut<Tag> for Context
where
    Context: DerefMut<Target = Target>,
    Target: HasFieldMut<Tag, Value = Value> + 'static,
{
    fn get_field_mut(&mut self, tag: PhantomData<Tag>) -> &mut Self::Value {
        self.deref_mut().get_field_mut(tag)
    }
}
