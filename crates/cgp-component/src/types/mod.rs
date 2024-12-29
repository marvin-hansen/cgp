/// Utility types for component implementation and composition.
///
/// This module provides various utility types that support the implementation
/// and composition of components in the CGP framework. These types enable
/// patterns like delegation, context injection, and provider enhancement.
///
/// # Types
///
/// * [`UseContext`] and [`WithContext`] - Types for implementing components
///   that require context or environment information.
///
/// * [`UseDelegate`] - Type for declaring which components should be delegated
///   to another type.
///
/// * [`WithProvider`] - Type for enhancing component implementations with
///   additional capabilities through providers.
///
/// # Example
///
/// ```rust,ignore
/// use cgp_component::{UseContext, UseDelegate, WithProvider};
///
/// // A component that requires context
/// struct ContextualLogger<Ctx> {
///     context: UseContext<Ctx>,
/// }
///
/// // A component that delegates its implementation
/// struct DelegatingService {
///     delegate: UseDelegate<Logger>,
/// }
///
/// // A component enhanced with a provider
/// struct EnhancedStore<P> {
///     store: WithProvider<P>,
/// }
/// ```
///
/// These types work together to provide a flexible and type-safe way to
/// compose and enhance component implementations.
pub mod use_context;
pub mod use_delegate;
pub mod with_provider;

pub use use_context::{UseContext, WithContext};
pub use use_delegate::UseDelegate;
pub use with_provider::WithProvider;
