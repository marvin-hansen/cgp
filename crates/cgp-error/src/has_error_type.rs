use core::fmt::Debug;

use cgp_async::Async;
use cgp_component::{cgp_component, DelegateComponent, HasComponents, WithProvider};
use cgp_type::traits::has_type::ProvideType;

/**
   This is used for contexts to declare that they have a _unique_ `Self::Error` type.

   Although it is possible for each context to declare their own associated
   `Error` type, doing so may result in having multiple ambiguous `Self::Error` types,
   if there are multiple associated types with the same name in different traits.

   As a result, it is better for context traits to include `HasError` as their
   parent traits, so that multiple traits can all refer to the same abstract
   `Self::Error` type.
*/
#[cgp_component {
    name: ErrorTypeComponent,
    provider: ProvideErrorType,
}]
pub trait HasErrorType {
    /**
       The `Error` associated type is also required to implement [`Debug`].

       This is to allow `Self::Error` to be used in calls like `.unwrap()`,
       as well as for simpler error logging.
    */
    type Error: Async + Debug;
}

pub type ErrorOf<Context> = <Context as HasErrorType>::Error;

impl<Context, Provider, Error> ProvideErrorType<Context> for WithProvider<Provider>
where
    Provider: ProvideType<Context, ErrorTypeComponent, Type = Error>,
    Error: Async + Debug,
{
    type Error = Error;
}
