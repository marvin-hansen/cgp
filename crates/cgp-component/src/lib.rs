#![no_std]

pub mod macros;
pub mod traits;

pub use cgp_component_macro::derive_component;
pub use traits::{DelegateComponent, HasComponents};
