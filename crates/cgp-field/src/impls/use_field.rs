/*!
   Field access implementations through type providers.

   This module provides implementations that enable field access through
   the type provider mechanism. It allows fields to be accessed in a
   type-safe manner using the CGP component system.
*/

use core::marker::PhantomData;

use cgp_component::WithProvider;
use cgp_type::traits::has_type::ProvideType;

use crate::traits::has_field::{FieldGetter, HasField};
use crate::traits::has_field_mut::{HasFieldMut, MutFieldGetter};

/// Type provider for field access.
///
/// This type implements the provider pattern for field access, allowing
/// fields to be accessed through the type system. It uses phantom types
/// to ensure type safety.
///
/// # Type Parameters
///
/// * `Tag` - Phantom type identifying the field
///
/// # Examples
///
/// ```rust,ignore
/// use cgp_field::UseField;
/// use core::marker::PhantomData;
///
/// struct Name;
/// let provider = UseField(PhantomData::<Name>);
/// ```
pub struct UseField<Tag>(pub PhantomData<Tag>);

/// Type alias for a provider-wrapped field access.
///
/// This type combines `UseField` with `WithProvider` to create a complete
/// field access provider that can be used in the component system.
///
/// # Type Parameters
///
/// * `Tag` - Phantom type identifying the field
pub type WithField<Tag> = WithProvider<UseField<Tag>>;

/// Implementation of `ProvideType` for field access.
///
/// This implementation allows fields to be accessed as types through
/// the type provider system. It connects the field access system with
/// the type system.
impl<Context, TypeTag, FieldTag, Field> ProvideType<Context, TypeTag> for UseField<FieldTag>
where
    Context: HasField<FieldTag, Value = Field>,
{
    type Type = Field;
}

/// Implementation of `FieldGetter` for field access providers.
///
/// This implementation enables read-only access to fields through
/// the provider system. It delegates to the context's field access
/// implementation.
impl<Context, OutTag, Tag, Value> FieldGetter<Context, OutTag> for UseField<Tag>
where
    Context: HasField<Tag, Value = Value>,
{
    type Value = Value;

    fn get_field(context: &Context, _tag: PhantomData<OutTag>) -> &Value {
        context.get_field(PhantomData)
    }
}

/// Implementation of `MutFieldGetter` for field access providers.
///
/// This implementation enables mutable access to fields through
/// the provider system. It delegates to the context's mutable
/// field access implementation.
impl<Context, OutTag, Tag, Value> MutFieldGetter<Context, OutTag> for UseField<Tag>
where
    Context: HasFieldMut<Tag, Value = Value>,
{
    fn get_field_mut(context: &mut Context, _tag: PhantomData<OutTag>) -> &mut Value {
        context.get_field_mut(PhantomData)
    }
}
