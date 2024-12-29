/// Trait for types that provide component implementations.
///
/// This trait is used to declare what components a type provides. It is a key part
/// of the CGP framework's component system, enabling component composition and reuse.
///
/// # Associated Types
///
/// * `Components` - A type that represents the collection of components provided.
///   This is typically a tuple of component traits.
///
/// # Examples
///
/// ```rust,ignore
/// use cgp_component::HasComponents;
///
/// trait Logger {
///     fn log(&self, message: &str);
/// }
///
/// trait Metrics {
///     fn record(&self, metric: &str, value: f64);
/// }
///
/// struct MyService;
///
/// // Declare that MyService provides both Logger and Metrics components
/// impl HasComponents for MyService {
///     type Components = (Logger, Metrics);
/// }
///
/// // Implement the components
/// impl Logger for MyService {
///     fn log(&self, message: &str) {
///         println!("{}", message);
///     }
/// }
///
/// impl Metrics for MyService {
///     fn record(&self, metric: &str, value: f64) {
///         println!("{}: {}", metric, value);
///     }
/// }
/// ```
///
/// # Note
///
/// This trait is typically implemented using the `cgp_component` macro,
/// which handles the implementation details automatically. The macro ensures
/// that all declared components are properly implemented.
pub trait HasComponents {
    type Components;
}
