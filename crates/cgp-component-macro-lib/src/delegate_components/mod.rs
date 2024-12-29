/// Component delegation functionality for the CGP framework.
///
/// This module provides the core functionality for delegating component implementations
/// in the Component-based Generic Programming (CGP) framework. It enables the creation
/// of composite components by delegating their functionality to underlying implementations.
///
/// # Module Structure
///
/// * [`ast`] - Abstract Syntax Tree definitions for parsing delegation syntax
/// * [`define_struct`] - Utilities for generating struct definitions with proper generic handling
/// * [`delegate`] - Core delegation implementation and macro expansion
/// * [`delegates_to`] - Trait definitions and bounds for delegation relationships
/// * [`impl_delegate`] - Implementation details for component delegation
/// * [`merge_generics`] - Utilities for combining generic parameters from multiple sources
///
/// # Main Entry Point
///
/// The primary entry point for component delegation is the [`delegate_components`]
/// function, which processes delegation specifications and generates the necessary
/// implementation code.
///
/// # Example
///
/// ```rust,ignore
/// #[delegate_components]
/// struct MyComposite<T> {
///     inner: Inner<T>
/// }
///
/// // Delegates the Debug and Display components to the inner field
/// impl<T> DelegatesTo<[Debug, Display], Inner<T>> for MyComposite<T> {}
/// ```
pub mod ast;
pub mod define_struct;
pub mod delegate;
pub mod delegates_to;
pub mod impl_delegate;
pub mod merge_generics;

pub use delegate::delegate_components;
