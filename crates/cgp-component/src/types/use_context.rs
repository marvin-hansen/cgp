/// Types for working with contextual component implementations.
///
/// This module provides types that enable components to be implemented in a way
/// that depends on some context or environment.
use crate::WithProvider;

/// Marker type for context-dependent component implementations.
///
/// This type is used as a marker to indicate that a component implementation
/// requires some form of context or environment to function.
///
/// # Example
///
/// ```rust,ignore
/// use cgp_component::{UseContext, WithContext};
///
/// trait Logger {
///     fn log(&self, message: &str);
/// }
///
/// // A logger that needs some context to function
/// struct ContextLogger<Ctx> {
///     context: Ctx,
/// }
///
/// // Implement Logger for ContextLogger when wrapped with WithContext
/// impl<Ctx> Logger for WithContext<ContextLogger<Ctx>> {
///     fn log(&self, message: &str) {
///         // Use context to determine how to log
///         println!("[{}] {}", self.context, message);
///     }
/// }
/// ```
pub struct UseContext;

/// Type alias for a context-dependent component implementation.
///
/// This type wraps a component implementation with context support, enabling
/// the implementation to access context information when executing component
/// methods.
pub type WithContext = WithProvider<UseContext>;
