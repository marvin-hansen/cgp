#![no_std]

#[cfg(feature = "sync")]
pub mod sync;

#[cfg(feature = "async")]
pub mod r#async;

#[cfg(all(feature = "sync", not(feature = "async")))]
pub use sync::*;

#[cfg(all(feature = "async", not(feature = "sync")))]
#[cfg(feature = "async")]
pub use r#async::*;

#[cfg(all(feature = "async", feature = "sync"))]
compile_error!("sync and async feature flags cannot be both activated");