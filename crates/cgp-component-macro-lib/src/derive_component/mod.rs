/// Module for deriving component implementations.
///
/// This module contains the core functionality for deriving component implementations
/// in the CGP (Component-based Generic Programming) framework. It handles both provider
/// and consumer implementations, along with various utility functions for processing
/// component specifications and type manipulations.
pub mod component_name;
pub mod component_spec;
pub mod consumer_impl;
pub mod delegate_fn;
pub mod delegate_type;
pub mod derive;
pub mod entry;
pub mod provider_impl;
pub mod provider_trait;
pub mod replace_self_receiver;
pub mod replace_self_type;
pub mod signature_args;
pub mod snake_case;

pub use derive::derive_component;
