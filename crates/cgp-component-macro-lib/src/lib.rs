/*!
   This is an internal crate used by the `cgp-component-macro` crate. We implement the
   proc macros for `cgp-component` as a library, so that it can be more easily tested.
   The constructs are then re-exported as proc macros in the `cgp-component-macro` crate,
   which is defined as a proc macro crate.

   # Overview
   This crate provides the core implementation for component-based programming macros.
   It includes functionality for:
   - Delegating component implementations
   - Deriving component traits
   - Pattern replacement utilities
   - Preset component definitions
*/

pub mod delegate_components;
pub mod derive_component;
pub mod for_each_replace;
pub mod preset;

#[cfg(test)]
mod tests;

pub use crate::delegate_components::delegate_components;
pub use crate::derive_component::derive_component;
pub use crate::for_each_replace::{handle_for_each_replace, handle_replace};
pub use crate::preset::define_preset;
