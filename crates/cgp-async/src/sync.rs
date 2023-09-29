pub use cgp_macros::strip_async as async_generic_trait;

pub trait Async {}

impl<T> Async for T {}
