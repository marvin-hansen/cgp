#![no_std]

extern crate alloc;

use cgp_async::Async;
use cgp_component::{derive_component, DelegateComponent, HasComponents};

#[derive_component(InnerComponent, ProvideInner<Context>)]
pub trait HasInner: Async {
    type Inner: Async;

    fn inner(&self) -> &Self::Inner;
}
