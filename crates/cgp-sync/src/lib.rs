pub use cgp_strip_async::strip_async as async_trait;

pub trait Async: Sized + 'static {}

impl<T> Async for T where T: Sized + 'static {}
