use cgp_core::prelude::*;
use cgp_core::ErrorRaiser;
use cgp_core::ProvideErrorType;
use eyre::Report;
use std::error::Error as StdError;

pub struct HandleErrorsWithEyre;

impl<Context> ProvideErrorType<Context> for HandleErrorsWithEyre
where
    Context: Async,
{
    type Error = Report;
}

impl<Context, E> ErrorRaiser<Context, E> for HandleErrorsWithEyre
where
    Context: HasErrorType<Error = Report>,
    E: StdError + Async,
{
    fn raise_error(e: E) -> Report {
        e.into()
    }
}
