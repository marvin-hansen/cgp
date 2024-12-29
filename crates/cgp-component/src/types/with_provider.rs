/// Type for wrapping component implementations with providers.
///
/// This module provides a type that enables component implementations to be
/// enhanced with additional capabilities through providers.
use core::marker::PhantomData;

/// Wrapper type for provider-enhanced component implementations.
///
/// This type wraps a component implementation with a provider type, enabling
/// the implementation to access additional capabilities or context provided
/// by the provider.
///
/// # Type Parameters
///
/// * `Provider` - A type that provides additional capabilities or context to
///   the component implementation.
///
/// # Example
///
/// ```rust,ignore
/// use cgp_component::WithProvider;
///
/// // A provider that adds logging capabilities
/// struct LogProvider;
///
/// trait DataStore {
///     fn save(&self, key: &str, value: &str);
/// }
///
/// struct MemoryStore;
///
/// // Implement DataStore with logging capabilities
/// impl DataStore for WithProvider<LogProvider, MemoryStore> {
///     fn save(&self, key: &str, value: &str) {
///         // Use the provider to log the operation
///         println!("Saving: {} = {}", key, value);
///         
///         // Perform the actual storage operation
///         // ...
///     }
/// }
/// ```
///
/// # Note
///
/// This type is commonly used to add cross-cutting concerns like logging,
/// metrics, or context to component implementations without modifying the
/// core implementation.
pub struct WithProvider<Provider>(pub PhantomData<Provider>);
