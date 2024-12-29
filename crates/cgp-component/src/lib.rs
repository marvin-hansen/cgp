#![no_std]

/*!
   Core traits and types for Component-based Generic Programming (CGP).

   This crate provides the fundamental building blocks for implementing component-based
   systems using the CGP framework. It defines core traits for component delegation
   and composition, along with utility types and macros for working with components.

   # Core Traits

   * [`DelegateComponent`] - Core trait for component delegation, allowing types to
     delegate component implementations to other types.
   * [`HasComponents`] - Trait for types that provide component implementations,
     enabling component composition and reuse.

   # Utility Types

   * [`UseContext`] - Type for using components with context
   * [`UseDelegate`] - Type for delegating component implementations
   * [`WithContext`] - Type for providing components with context
   * [`WithProvider`] - Type for providing component implementations

   # Macros

   The crate re-exports several macros from [`cgp_component_macro`]:
   * `cgp_component` - Derive macro for implementing component traits
   * `cgp_preset` - Macro for defining reusable component configurations
   * `delegate_components` - Macro for implementing component delegation
   * `for_each_replace` - Utility macro for type replacement
   * `replace_with` - Utility macro for type substitution

   # Example

   ```rust,ignore
   use cgp_component::{cgp_component, delegate_components, DelegateComponent};

   #[cgp_component]
   trait Logger {
       fn log(&self, message: &str);
   }

   struct ConsoleLogger;

   #[cgp_component]
   impl Logger for ConsoleLogger {
       fn log(&self, message: &str) {
           println!("{}", message);
       }
   }

   #[delegate_components]
   struct MyApp {
       logger: ConsoleLogger,
   }
   ```
*/

pub mod traits;
pub mod types;

pub use cgp_component_macro::{
    cgp_component, cgp_preset, delegate_components, for_each_replace, replace_with,
};
pub use traits::{DelegateComponent, HasComponents};
pub use types::{UseContext, UseDelegate, WithContext, WithProvider};
