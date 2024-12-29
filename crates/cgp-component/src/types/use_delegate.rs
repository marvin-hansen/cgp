/// Type for declaring component delegation.
///
/// This module provides a type that helps declare which components should be
/// delegated to another type.
use core::marker::PhantomData;

/// Marker type for component delegation.
///
/// This type is used to specify which components should be delegated to another
/// type. It uses a phantom type parameter to carry the component type information
/// without actually storing any data.
///
/// # Type Parameters
///
/// * `Components` - A type representing the components to be delegated. This is
///   typically a tuple of component traits.
///
/// # Example
///
/// ```rust,ignore
/// use cgp_component::{DelegateComponent, UseDelegate};
///
/// trait Logger {
///     fn log(&self, message: &str);
/// }
///
/// trait Metrics {
///     fn record(&self, metric: &str, value: f64);
/// }
///
/// struct MyService {
///     delegate: UseDelegate<(Logger, Metrics)>,
/// }
///
/// // This indicates that both Logger and Metrics components
/// // should be delegated to another type
/// impl<T> DelegateComponent<Logger> for MyService {
///     type Delegate = T;
/// }
///
/// impl<T> DelegateComponent<Metrics> for MyService {
///     type Delegate = T;
/// }
/// ```
///
/// # Note
///
/// This type is typically used in conjunction with the `delegate_components`
/// macro, which handles the delegation implementation details automatically.
pub struct UseDelegate<Components>(pub PhantomData<Components>);
