/// Core trait for component delegation in the CGP framework.
///
/// This trait establishes the relationship between a type and the components it
/// delegates to another type. It is a fundamental building block of the CGP
/// framework's component system.
///
/// # Type Parameters
///
/// * `N` - The component type being delegated. This is typically a trait that
///   represents the component's interface.
///
/// # Associated Types
///
/// * `Delegate` - The type that provides the actual implementation of the component.
///
/// # Examples
///
/// ```rust,ignore
/// use cgp_component::{cgp_component, delegate_components, DelegateComponent};
///
/// trait Logger {
///     fn log(&self, message: &str);
/// }
///
/// struct ConsoleLogger;
/// impl Logger for ConsoleLogger {
///     fn log(&self, message: &str) {
///         println!("{}", message);
///     }
/// }
///
/// struct MyApp {
///     logger: ConsoleLogger,
/// }
///
/// // Delegate the Logger component to ConsoleLogger
/// impl DelegateComponent<Logger> for MyApp {
///     type Delegate = ConsoleLogger;
/// }
/// ```
///
/// # Note
///
/// This trait is typically implemented using the `delegate_components` macro,
/// which handles the implementation details automatically. Manual implementation
/// is possible but not recommended for most use cases.
pub trait DelegateComponent<Name> {
    type Delegate;
}
