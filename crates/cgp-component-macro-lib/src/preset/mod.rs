/// Component preset system for the CGP framework.
///
/// This module provides a powerful preset system that allows defining reusable
/// component configurations. Presets enable the creation of common component
/// patterns that can be applied to different types, reducing code duplication
/// and promoting consistent component usage patterns.
///
/// # Module Structure
///
/// * [`ast`] - Abstract Syntax Tree definitions for parsing preset specifications
/// * [`mod@define_preset`] - Core functionality for defining new presets
/// * [`impl_is_preset`] - Trait implementations for preset type relationships
/// * [`substitution_macro`] - Macro utilities for type substitution in presets
///
/// # Usage
///
/// Presets are typically defined using the `define_preset` macro:
///
/// ```rust,ignore
/// define_preset! {
///     MyPreset<T> {
///         [Debug, Display]: Inner<T>
///     }
/// }
/// ```
///
/// This generates:
/// - A struct `MyPreset<T>`
/// - A trait `IsMyPreset<Component>`
/// - Delegation implementations
/// - A `with_my_preset` macro for type substitution
///
/// The preset can then be used to apply common component patterns:
///
/// ```rust,ignore
/// #[derive(Debug, Display)]
/// struct Inner<T>(T);
///
/// struct Wrapper<T> {
///     inner: Inner<T>
/// }
///
/// impl<T> DelegatesTo<MyPreset<T>, Inner<T>> for Wrapper<T> {}
/// ```
pub mod define_preset;
pub use define_preset::define_preset;
pub mod ast;
pub mod impl_is_preset;
pub mod substitution_macro;
