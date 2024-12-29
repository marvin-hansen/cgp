/// Core traits for the CGP component system.
///
/// This module provides the fundamental traits that enable component-based
/// programming in the CGP framework:
///
/// # Traits
///
/// * [`DelegateComponent`] - Core trait for delegating component implementations
///   to other types. This enables composition and reuse of components through
///   delegation.
///
/// * [`HasComponents`] - Trait for declaring what components a type provides.
///   This enables type-safe component composition and discovery.
///
/// These traits work together to provide a flexible and type-safe component
/// system. Types can both provide components (via `HasComponents`) and delegate
/// component implementations to other types (via `DelegateComponent`).
///
/// # Example
///
/// ```rust,ignore
/// use cgp_component::{DelegateComponent, HasComponents};
///
/// // Define a component
/// trait Logger {
///     fn log(&self, message: &str);
/// }
///
/// // A type that provides the Logger component
/// struct ConsoleLogger;
/// impl HasComponents for ConsoleLogger {
///     type Components = Logger;
/// }
///
/// // A type that delegates the Logger component
/// struct MyApp {
///     logger: ConsoleLogger,
/// }
/// impl DelegateComponent<Logger> for MyApp {
///     type Delegate = ConsoleLogger;
/// }
/// ```
pub mod delegate_component;
pub mod has_components;

pub use delegate_component::DelegateComponent;
pub use has_components::HasComponents;
