pub use cgp_strip_async::strip_async as async_trait;

pub trait Async: Sized {}

impl<T> Async for T where T: Sized {}
