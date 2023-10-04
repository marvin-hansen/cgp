#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use cgp_async::{async_trait, Async};
use cgp_component::{derive_component, DelegateComponent, HasComponents};
use cgp_error::HasErrorType;

#[derive_component(RunnerComponent, Runner<App>)]
#[async_trait]
pub trait CanRun: HasErrorType {
    async fn run(&self) -> Result<(), Self::Error>;
}
